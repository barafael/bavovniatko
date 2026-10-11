#!/usr/bin/env python3
"""Download the equipment catalogue in research/assets/README.md.

The README tables are the source of truth: this script parses them, re-verifies every
Sketchfab model's licence at download time (labels move), refuses anything whose
description admits a game rip, then stores each asset under files/<category>/<slug>/
together with a meta.yaml (author, licence + URL, provenance tier, hashes, attribution
line). CREDITS.md and REPORT.md are written at files/ root. files/ is gitignored.

Usage:
  fetch.py --list                       # parse + show the plan, download nothing
  fetch.py --only-extras                # Kenney/packs that need no Sketchfab token
  fetch.py [--token T | SKETCHFAB_TOKEN env | ~/.config/sketchfab/token]
                                        # verify + download everything
  fetch.py --only tanks --token T       # restrict to entries whose id contains 'tanks'
  fetch.py --force ...                  # re-download even if meta.yaml says ok

Resumable: an asset whose meta.yaml has status: ok is skipped.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import time
import urllib.error
import urllib.request
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent          # research/assets/
FILES = ROOT / "files"
README = ROOT / "README.md"
API = "https://api.sketchfab.com/v3"
UA = "bavovniatko-asset-fetch/1.0 (research use; contact via repo)"
TODAY = time.strftime("%Y-%m-%d")

SECTIONS = [  # (header regex, dir slug, human name)
    (r"^## 1\.", "tanks", "Tanks"),
    (r"^## 2\.", "ifvs", "IFVs, APCs, light armour"),
    (r"^## 3\.", "artillery", "Artillery"),
    (r"^## 4\.", "mlrs", "MLRS"),
    (r"^## 5\.", "air-defence", "Air defence"),
    (r"^## 6\.", "glide-missiles", "Glide weapons and missiles"),
    (r"^## 7\.", "helicopters", "Helicopters"),
    (r"^## 8\.", "aircraft", "Fixed-wing aircraft"),
    (r"^## 9\.", "drones", "Drones (air)"),
    (r"^## 10\.", "naval", "Naval and sea drones"),
    (r"^## 11\.", "trucks", "Trucks, engineering, UGVs"),
    (r"^## 12\.", "civilian", "Civilian cars, vans, buses"),
]

# Free-standing downloads that are not table rows in the README.
EXTRAS = [
    {
        "id": "filler-packs_kenney-car-kit",
        "equipment": "civilian car filler pack",
        "title": "Kenney Car Kit",
        "author": "Kenney",
        "author_url": "https://kenney.nl",
        "source": "https://kenney.nl/assets/car-kit",
        "kind": "kenney",
        "category": "filler-packs",
        "licence": ("CC0 1.0", "https://creativecommons.org/publicdomain/zero/1.0/"),
        "provenance": "verified-original",
        "faces": None,
        "note": "45 modular low-poly cars; CC0 civilian-traffic layer",
    },
    {
        "id": "filler-packs_quaternius-cars",
        "equipment": "civilian car filler pack",
        "title": "Quaternius Cars Pack",
        "author": "Quaternius",
        "author_url": "https://quaternius.com",
        "source": "https://quaternius.com/packs/cars.html",
        "kind": "manual",
        "category": "filler-packs",
        "licence": ("CC0 1.0", "https://creativecommons.org/publicdomain/zero/1.0/"),
        "provenance": "verified-original",
        "faces": None,
        "note": "8 cars; download button is JS/Patreon-gated — fetch by hand",
    },
    {
        "id": "filler-packs_quaternius-public-transport",
        "equipment": "civilian bus/ambulance filler pack",
        "title": "Quaternius Public Transport Pack",
        "author": "Quaternius",
        "author_url": "https://quaternius.com",
        "source": "https://quaternius.com/packs/publictransport.html",
        "kind": "manual",
        "category": "filler-packs",
        "licence": ("CC0 1.0", "https://creativecommons.org/publicdomain/zero/1.0/"),
        "provenance": "verified-original",
        "faces": None,
        "note": "12 vehicles; download button is JS/Patreon-gated — fetch by hand",
    },
]

# Descriptions containing these phrases mean the uploader does not hold the rights.
RIP_PHRASES = [
    "ripped from", "rip from", "war thunder", "warno", "combat mission",
    "wargame: red dragon", "wargame red dragon", "armored warfare",
    "call to arms", "cold waters", "spintires", "dcs world", "from dcs",
    "from battlefield", "game rip",
]

LICENCE_URLS = {
    "cc attribution": ("CC BY 4.0", "https://creativecommons.org/licenses/by/4.0/"),
    "cc attribution-sharealike": ("CC BY-SA 4.0", "https://creativecommons.org/licenses/by-sa/4.0/"),
    "cc0": ("CC0 1.0", "https://creativecommons.org/publicdomain/zero/1.0/"),
    "public domain": ("CC0 1.0", "https://creativecommons.org/publicdomain/zero/1.0/"),
}
LICENCE_FALLBACK = ("CC BY 4.0", "https://creativecommons.org/licenses/by/4.0/")

PROVENANCE = {"✓": "verified-original", "?": "unconfirmed", "⚠SA": "share-alike-flagged"}

http_sleep = {"verify": 1.2, "download": 2.5}


# ---------------------------------------------------------------- catalogue parsing

def slugify(text: str, maxlen: int = 42) -> str:
    s = re.sub(r"\*+", "", text.lower())
    s = re.sub(r"[^a-z0-9]+", "-", s).strip("-")
    return s[:maxlen].rstrip("-") or "asset"


def parse_faces(cell: str):
    cell = cell.strip().replace("~", "").replace(",", "")
    m = re.fullmatch(r"([\d.]+)\s*k", cell, re.I)
    if m:
        return int(float(m.group(1)) * 1_000)
    m = re.fullmatch(r"([\d.]+)\s*m", cell, re.I)
    if m:
        return int(float(m.group(1)) * 1_000_000)
    return int(cell) if cell.isdigit() else None


def sketchfab_uid(url: str) -> str | None:
    tail = url.rstrip("/").rsplit("/", 1)[-1]
    m = re.search(r"([0-9a-f]{32})$", tail)
    return m.group(1) if m else None


def parse_catalogue() -> list[dict]:
    entries: list[dict] = []
    category, cat_name = None, None
    seen_ids: dict[str, int] = {}
    for line in README.read_text().splitlines():
        hit = next((s for s in SECTIONS if re.match(s[0], line)), None)
        if hit:
            category, cat_name = hit[1], hit[2]
            continue
        if not line.startswith("|") or category is None:
            continue
        clean = line.replace("\\|", "·")          # keep escaped pipes out of the split
        cols = [c.strip() for c in clean.strip().strip("|").split("|")]
        if len(cols) != 6 or set(cols[0]) <= {"-", " "} or cols[0] == "equipment":
            continue
        equipment, asset_cell, author, faces, tier, note = cols
        links = re.findall(r"\[((?:[^\[\]]|\[[^\]]*\])*)\]\((http[^)\s]+)\)", asset_cell)
        if not links:
            entries.append({
                "id": f"{category}_{slugify(equipment)}", "equipment": equipment,
                "title": asset_cell, "author": author, "author_url": "",
                "source": "", "kind": "no-url", "category": category,
                "provenance": PROVENANCE.get(tier, "unconfirmed"),
                "faces": parse_faces(faces), "note": note,
            })
            continue
        for idx, (title, url) in enumerate(links):
            uid = sketchfab_uid(url) if "sketchfab.com" in url else None
            kind = "sketchfab" if uid else ("thingiverse" if "thingiverse.com" in url else "direct")
            base = f"{category}_{slugify(equipment)}_{slugify(author, 24)}"
            if len(links) > 1:
                base += f"-{idx + 1}"
            n = seen_ids.get(base, 0)
            seen_ids[base] = n + 1
            entry_id = base if n == 0 else f"{base}-{n + 1}"
            entries.append({
                "id": entry_id, "equipment": equipment, "title": title, "author": author,
                "author_url": f"https://sketchfab.com/@{author}" if uid else "",
                "source": url, "sketchfab_uid": uid, "kind": kind, "category": category,
                "provenance": PROVENANCE.get(tier, "unconfirmed"),
                "faces": parse_faces(faces), "note": note,
            })
    entries += EXTRAS
    for e in entries:
        e.setdefault("sketchfab_uid", None)
    return entries


# ---------------------------------------------------------------- http helpers

def http_json(url: str, token: str | None = None, tries: int = 3):
    last = None
    for attempt in range(tries):
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        if token:
            req.add_header("Authorization", f"Token {token}")
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return json.loads(r.read().decode())
        except urllib.error.HTTPError as e:
            body = e.read()[:400].decode(errors="replace")
            last = f"HTTP {e.code}: {body}"
            if e.code in (429, 500, 502, 503) and attempt < tries - 1:
                time.sleep((10, 30, 60)[attempt])
                continue
            raise RuntimeError(last) from e
        except (urllib.error.URLError, TimeoutError) as e:
            last = str(e)
            if attempt < tries - 1:
                time.sleep(10)
                continue
            raise RuntimeError(last) from e
    raise RuntimeError(last or "unreachable")


def http_file(url: str, dest: Path, max_bytes: int = 2_000_000_000) -> Path:
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    part = dest.with_suffix(dest.suffix + ".part")
    with urllib.request.urlopen(req, timeout=300) as r, open(part, "wb") as f:
        total = 0
        while chunk := r.read(1 << 17):
            total += len(chunk)
            if total > max_bytes:
                raise RuntimeError(f"over size cap ({max_bytes} bytes)")
            f.write(chunk)
    part.replace(dest)
    return dest


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while chunk := f.read(1 << 20):
            h.update(chunk)
    return h.hexdigest()


# ---------------------------------------------------------------- yaml/meta output

def yaml_str(value) -> str:
    s = str(value)
    return json.dumps(s, ensure_ascii=False) if s and re.search(r"[:#'\"]", s) else s


def write_meta(entry: dict, status: str, detail: str, files: list[dict], licence: tuple):
    d = FILES / entry["category"] / entry["id"]
    d.mkdir(parents=True, exist_ok=True)
    lines = [
        f"id: {entry['id']}",
        f"equipment: {yaml_str(entry['equipment'])}",
        f"title: {yaml_str(entry['title'])}",
        f"author: {yaml_str(entry['author'])}",
        f"author_url: {entry['author_url']}",
        f"source: {entry['source']}",
        f"source_kind: {entry['kind']}",
        f"sketchfab_uid: {entry.get('sketchfab_uid') or ''}",
        f"category: {entry['category']}",
        "licence:",
        f"  label: {yaml_str(licence[0])}",
        f"  url: {licence[1]}",
        f"provenance: {entry['provenance']}",
        f"faces: {entry.get('faces') or ''}",
        f"note: {yaml_str(entry['note'])}",
        f"catalogue: research/assets/README.md",
        f"attribution: {yaml_str('%s by %s — %s — %s' % (entry['title'], entry['author'], licence[0], entry['source']))}",
        f"downloaded: {TODAY}",
        f"status: {status}",
        f"status_detail: {yaml_str(detail)}",
        "files:",
    ]
    lines += [f"  - {{name: {yaml_str(f['name'])}, bytes: {f['bytes']}, sha256: {f['sha256']}}}"
              for f in files[:200]]
    if len(files) > 200:
        lines.append(f"  # ...and {len(files) - 200} more")
    (d / "meta.yaml").write_text("\n".join(lines) + "\n")


def meta_status(entry: dict) -> str | None:
    p = FILES / entry["category"] / entry["id"] / "meta.yaml"
    if not p.exists():
        return None
    m = re.search(r"^status: (\S+)", p.read_text(), re.M)
    return m.group(1) if m else None


# ---------------------------------------------------------------- asset fetchers

def unzip_into(zip_path: Path, dest: Path) -> list[dict]:
    out = []
    with zipfile.ZipFile(zip_path) as z:
        for info in z.infolist():
            if info.is_dir():
                continue
            name = info.filename.replace("\\", "/")
            target = (dest / name).resolve()
            if not str(target).startswith(str(dest.resolve())):
                raise RuntimeError(f"zip path escape: {name}")
            target.parent.mkdir(parents=True, exist_ok=True)
            with z.open(info) as src, open(target, "wb") as f:
                f.write(src.read())
            out.append({"name": name, "bytes": info.file_size, "sha256": sha256(target)})
    zip_path.unlink()
    return out


def verify_sketchfab(entry: dict, token: str) -> tuple[dict, tuple]:
    """Re-check licence + provenance at download time. Returns (model json, licence tuple)."""
    m = http_json(f"{API}/models/{entry['sketchfab_uid']}", token=token)
    time.sleep(http_sleep["verify"])
    if not m.get("isDownloadable"):
        raise RuntimeError("no longer downloadable")
    label = (m.get("license") or {}).get("label", "").lower()
    if any(bad in label for bad in ("noncommercial", "noderiv")):
        raise RuntimeError(f"licence now {label!r}")
    licence = next((v for k, v in LICENCE_URLS.items() if k in label), None)
    if licence is None and label:
        raise RuntimeError(f"unrecognised licence {label!r}")
    licence = licence or (LICENCE_FALLBACK if entry["provenance"] != "share-alike-flagged"
                          else LICENCE_URLS["cc attribution-sharealike"])
    desc = (m.get("description") or "").lower()
    rip = next((p for p in RIP_PHRASES if p in desc), None)
    if rip:
        raise RuntimeError(f"description admits rip: {rip!r}")
    return m, licence


def fetch_sketchfab(entry: dict, token: str):
    model, licence = verify_sketchfab(entry, token)
    dl = http_json(f"{API}/models/{entry['sketchfab_uid']}/download", token=token)
    time.sleep(http_sleep["download"])
    pick = next((k for k in ("gltf", "glb", "source", "usdz") if dl.get(k)), None)
    if not pick:
        raise RuntimeError(f"no downloadable format in {sorted(dl)}")
    fmt = dl[pick]
    d = FILES / entry["category"] / entry["id"]
    d.mkdir(parents=True, exist_ok=True)
    archive = d / f"{entry['id']}.{pick}.zip"
    http_file(fmt["url"], archive)
    files = unzip_into(archive, d) if zipfile.is_zipfile(archive) else [
        {"name": archive.name, "bytes": archive.stat().st_size, "sha256": sha256(archive)}]
    write_meta(entry, "ok", f"format {pick}, {fmt.get('size', '?')} bytes", files, licence)
    return f"{pick}, {len(files)} files"


def fetch_kenney(entry: dict):
    url = entry["source"]
    try:
        html = urllib.request.urlopen(urllib.request.Request(
            url, headers={"User-Agent": UA}), timeout=60).read().decode()
        m = re.search(r"href='([^']*kenney_[^']*\.zip)'", html)
        if m:
            url = m.group(1)
    except Exception:
        pass  # fall back to the last-known URL below
    if "media/pages" not in url:
        url = "https://kenney.nl/media/pages/assets/car-kit/1a312ec241-1775131960/kenney_car-kit.zip"
    d = FILES / entry["category"] / entry["id"]
    d.mkdir(parents=True, exist_ok=True)
    archive = d / "kenney_car-kit.zip"
    http_file(url, archive)
    files = unzip_into(archive, d)
    write_meta(entry, "ok", f"{url}", files, entry["licence"])
    return f"{len(files)} files"


def probe_thingiverse(entry: dict):
    try:
        req = urllib.request.Request(entry["source"], headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=60) as r:
            html = r.read().decode(errors="replace")
        links = sorted(set(re.findall(r'https://cdn\.thingiverse\.com/[^"\'\s]+', html)))
        if not links:
            raise RuntimeError("no cdn file links in page (login-gated)")
        d = FILES / entry["category"] / entry["id"]
        d.mkdir(parents=True, exist_ok=True)
        files = []
        for i, link in enumerate(links):
            fn = d / f"file{i:02d}_{link.rsplit('/', 1)[-1].split('?')[0]}"
            http_file(link, fn)
            files.append({"name": fn.name, "bytes": fn.stat().st_size, "sha256": sha256(fn)})
        write_meta(entry, "ok", "grabbed from cdn links", files, entry["licence"])
        return f"{len(files)} files"
    except Exception as e:
        write_meta(entry, "manual-download", f"thingiverse login required ({e})", [],
                   entry.get("licence", LICENCE_FALLBACK))
        return "manual-download"


# ---------------------------------------------------------------- main

def load_token(explicit: str | None) -> str | None:
    if explicit:
        return explicit
    import os
    if os.environ.get("SKETCHFAB_TOKEN"):
        return os.environ["SKETCHFAB_TOKEN"]
    for p in (ROOT.parents[2] / "SKETCHFAB_API_TOKEN",           # repo/../SKETCHFAB_API_TOKEN
              Path.home() / ".config/sketchfab/token"):
        if p.exists():
            return p.read_text().strip()
    return None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--token", help="Sketchfab API token (sketchfab.com → Settings → Password & API)")
    ap.add_argument("--only", help="restrict to entries whose id contains this substring")
    ap.add_argument("--only-extras", action="store_true", help="fetch only the non-Sketchfab extras")
    ap.add_argument("--force", action="store_true", help="re-download even if status is ok")
    ap.add_argument("--list", action="store_true", help="parse and list, download nothing")
    args = ap.parse_args()

    entries = parse_catalogue()
    if args.only:
        entries = [e for e in entries if args.only in e["id"]]
    sketch = [e for e in entries if e["kind"] == "sketchfab"]
    other = [e for e in entries if e["kind"] != "sketchfab"]

    if args.list:
        for e in entries:
            mark = {"sketchfab": "sf", "no-url": "--", "manual": "md", "kenney": "kn",
                    "thingiverse": "tv", "direct": "dl"}.get(e["kind"], e["kind"])
            print(f"{mark:2} {e['id'][:58]:58} {e['provenance']:20} {e['title'][:40]}")
        print(f"\n{len(entries)} entries ({len(sketch)} sketchfab, {len(other)} other)")
        return 0

    FILES.mkdir(parents=True, exist_ok=True)
    results: dict[str, list] = {"ok": [], "skipped": [], "manual-download": [], "failed": []}
    todo = other if args.only_extras else entries
    token = None
    if not args.only_extras and any(e["kind"] == "sketchfab" for e in todo):
        token = load_token(args.token)
        if not token:
            print("Sketchfab models present but no token. Pass --token, set SKETCHFAB_TOKEN,\n"
                  "or write ~/.config/sketchfab/token. Get one (free account) at:\n"
                  "  https://sketchfab.com/settings/passwords", file=sys.stderr)
            return 2

    for e in todo:
        status = None if args.force else meta_status(e)
        if status == "ok":
            results["skipped"].append(e["id"])
            continue
        try:
            if e["kind"] == "sketchfab":
                detail = fetch_sketchfab(e, token)
            elif e["kind"] == "kenney":
                detail = fetch_kenney(e)
            elif e["kind"] == "thingiverse":
                detail = probe_thingiverse(e)
            elif e["kind"] == "manual":
                write_meta(e, "manual-download", e["note"], [], e.get("licence", LICENCE_FALLBACK))
                detail = "manual-download"
            else:  # no-url rows (e.g. cross-references into another section)
                write_meta(e, "skipped-no-url", e["note"], [], LICENCE_FALLBACK)
                detail = "no url in catalogue row"
            if detail == "manual-download":
                results["manual-download"].append(e["id"])
            elif detail == "no url in catalogue row":
                results["skipped"].append(f"{e['id']} (no url)")
                print(f"skip      {e['id']}: {detail}", flush=True)
                continue
            else:
                results["ok"].append(f"{e['id']} ({detail})")
            print(f"ok        {e['id']}: {detail}", flush=True)
        except Exception as ex:
            write_meta(e, "failed", str(ex)[:300], [], ("unknown", ""))
            results["failed"].append(f"{e['id']}: {ex}")
            print(f"FAILED    {e['id']}: {ex}", flush=True)

    report = ["# fetch report " + TODAY, ""]
    for key in ("ok", "skipped", "manual-download", "failed"):
        report.append(f"## {key} ({len(results[key])})")
        report += [f"- {x}" for x in results[key]] + [""]
    (FILES / "REPORT.md").write_text("\n".join(report) + "\n")

    credits = ["# Credits — downloaded 3D assets (CC BY requires attribution)", ""]
    for meta in sorted(FILES.glob("*/*/meta.yaml")):
        text = meta.read_text()
        def field(name):
            m = re.search(rf"^{name}: (.+)$", text, re.M)
            return m.group(1).strip().strip('"') if m else ""
        if field("status") != "ok":
            continue
        credits.append(f"- {field('attribution')}  ")
        credits.append(f"  local: {meta.parent.relative_to(FILES)}")
    (FILES / "CREDITS.md").write_text("\n".join(credits) + "\n")

    print(f"\ndone: {len(results['ok'])} ok, {len(results['skipped'])} skipped, "
          f"{len(results['manual-download'])} manual, {len(results['failed'])} failed")
    return 1 if results["failed"] else 0


if __name__ == "__main__":
    sys.exit(main())
