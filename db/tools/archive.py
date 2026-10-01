#!/usr/bin/env python3
"""Private offline backup of everything the knowledge base points to (source and media URLs).

The archive lives OUTSIDE the repository and is never committed:
    $BAVOVNIATKO_ARCHIVE (default ~/bavovniatko-archive/)
        objects/ab/cd/<sha256>.<ext>   content-addressed response bodies
        warc/<run>.warc.gz             full HTTP request/response records (WARC 1.1)
        log.jsonl                      one line per fetch attempt
Each fetch is recorded as a `snapshot` record in the database (status, mime, size, sha256, path).

  archive.py fetch [--table source|media] [--limit N] [--retry-failed] [--wayback-submit] [--video]
  archive.py verify        re-hash every archived object against its snapshot record
  archive.py status        counts by outcome
  archive.py pack [--out FILE]   tar.gz of the archive for cold storage

Politeness: one request per host at a time, >= 2 s between requests to the same host, a generic
User-Agent with no personal data. On 403/429/5xx the Wayback Machine's latest capture is used instead
(recorded as via = "wayback"). `--wayback-submit` additionally asks the Internet Archive to capture
the live page (an outward-facing action, off by default). Video needs `--video` and yt-dlp.
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import mimetypes
import os
import shutil
import subprocess
import sys
import tarfile
import threading
import time
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from urllib.parse import quote, urlparse

import io

import requests
from warcio.statusandheaders import StatusAndHeaders
from warcio.warcwriter import WARCWriter

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402

ROOT = Path(os.environ.get("BAVOVNIATKO_ARCHIVE", Path.home() / "bavovniatko-archive"))
UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126 Safari/537.36"
MIN_GAP = 2.0
TIMEOUT = 45
MAX_BYTES = 200 * 1024 * 1024
VIDEO_HOSTS = ("youtube.com", "youtu.be", "vimeo.com", "t.me")

_host_locks: dict[str, threading.Lock] = defaultdict(threading.Lock)
_host_last: dict[str, float] = {}
_log_lock = threading.Lock()
_warc_lock = threading.Lock()
_warc_writer: WARCWriter | None = None


def warc_record(r: requests.Response):
    """Append the final response (headers + body) as a WARC response record."""
    status = f"{r.status_code} {r.reason or ''}".strip()
    headers = StatusAndHeaders(status, list(r.headers.items()), protocol="HTTP/1.1")
    with _warc_lock:
        rec = _warc_writer.create_warc_record(r.url, "response", payload=io.BytesIO(r.content or b""),
                                              http_headers=headers)
        _warc_writer.write_record(rec)


def now() -> dt.datetime:
    return dt.datetime.now(dt.timezone.utc)


def polite(host: str):
    lock = _host_locks[host]
    lock.acquire()
    wait = MIN_GAP - (time.monotonic() - _host_last.get(host, 0))
    if wait > 0:
        time.sleep(wait)
    return lock


def ext_for(mime: str | None, url: str) -> str:
    if mime:
        e = mimetypes.guess_extension(mime.split(";")[0].strip())
        if e:
            return {".htm": ".html", ".jpe": ".jpg"}.get(e, e)
    suffix = Path(urlparse(url).path).suffix
    return suffix if 1 < len(suffix) <= 6 else ".bin"


def store(body: bytes, mime: str | None, url: str) -> tuple[str, str]:
    sha = hashlib.sha256(body).hexdigest()
    rel = Path("objects") / sha[:2] / sha[2:4] / f"{sha}{ext_for(mime, url)}"
    path = ROOT / rel
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(body)
    return sha, str(rel)


def get(session: requests.Session, url: str) -> requests.Response:
    host = urlparse(url).netloc
    lock = polite(host)
    try:
        r = session.get(url, timeout=TIMEOUT, allow_redirects=True, stream=True,
                        headers={"User-Agent": UA, "Accept": "*/*", "Accept-Language": "en;q=0.9"})
        body = r.raw.read(MAX_BYTES + 1, decode_content=True)
        r._content = body[:MAX_BYTES]
        r._content_consumed = True
        return r
    finally:
        _host_last[host] = time.monotonic()
        lock.release()


def wayback_latest(session: requests.Session, url: str) -> str | None:
    try:
        lock = polite("archive.org")
        try:
            r = session.get(f"https://archive.org/wayback/available?url={quote(url, safe='')}", timeout=TIMEOUT,
                            headers={"User-Agent": UA})
        finally:
            _host_last["archive.org"] = time.monotonic()
            lock.release()
        snap = r.json().get("archived_snapshots", {}).get("closest", {})
        if snap.get("available") and snap.get("url"):
            ts = snap.get("timestamp", "")
            # id_ = the original bytes without the Wayback toolbar
            return f"https://web.archive.org/web/{ts}id_/{url}"
    except Exception:
        return None
    return None


def wayback_submit(session: requests.Session, url: str) -> str | None:
    try:
        lock = polite("web.archive.org")
        try:
            r = session.get(f"https://web.archive.org/save/{url}", timeout=120, headers={"User-Agent": UA})
        finally:
            _host_last["web.archive.org"] = time.monotonic()
            lock.release()
        loc = r.headers.get("Content-Location") or ""
        return f"https://web.archive.org{loc}" if loc.startswith("/web/") else (r.url if "/web/" in r.url else None)
    except Exception:
        return None


def fetch_one(target, url: str, warc_path: Path, opts) -> dict:
    snap = {"target": target, "url": url, "retrieved_at": now(), "ok": False}
    session = requests.Session()
    try:
        r = get(session, url)
        via = "direct"
        if r.status_code in (401, 403, 406, 429) or r.status_code >= 500 or not r.content:
            wb = wayback_latest(session, url)
            if wb:
                r2 = get(session, wb)
                if r2.ok and r2.content:
                    r, via = r2, "wayback"
        warc_record(r)
        snap.update({"http_status": r.status_code, "final_url": r.url, "via": via, "warc_file": str(warc_path.relative_to(ROOT))})
        if r.ok and r.content:
            mime = r.headers.get("Content-Type")
            sha, rel = store(r.content, mime, r.url)
            snap.update({"ok": True, "mime": mime, "bytes": len(r.content), "sha256": sha, "archive_path": rel})
            if len(r.content) >= MAX_BYTES:
                snap["error"] = f"truncated at {MAX_BYTES} bytes"
        else:
            snap["error"] = f"HTTP {r.status_code}"
    except Exception as e:  # network errors are data, not crashes
        snap["error"] = f"{type(e).__name__}: {e}"[:500]
    if opts.wayback_submit:
        wb = wayback_submit(session, url)
        if wb:
            snap["wayback_url"] = wb
    if opts.video and any(h in urlparse(url).netloc for h in VIDEO_HOSTS) and shutil.which("yt-dlp"):
        vdir = ROOT / "video"
        vdir.mkdir(parents=True, exist_ok=True)
        res = subprocess.run(["yt-dlp", "--no-progress", "-o", str(vdir / "%(id)s.%(ext)s"), "--print", "after_move:filepath", url],
                             capture_output=True, text=True, timeout=3600)
        if res.returncode == 0 and res.stdout.strip():
            vp = Path(res.stdout.strip().splitlines()[-1])
            sha, rel = store(vp.read_bytes(), mimetypes.guess_type(vp.name)[0], vp.name)
            vp.unlink()
            snap.setdefault("ext", {})["video"] = {"sha256": sha, "archive_path": rel}
    return snap


def cmd_fetch(opts):
    ROOT.mkdir(parents=True, exist_ok=True)
    (ROOT / "warc").mkdir(exist_ok=True)
    conn = kb.connect()
    run = now().strftime("%Y%m%dT%H%M%SZ")
    tables = [opts.table] if opts.table else ["source", "media"]
    todo = []
    for t in tables:
        rows = kb.one(conn, f"""SELECT id, url, alt_urls,
                (SELECT VALUE ok FROM snapshot WHERE target = $parent.id) AS oks FROM {t}""") if t == "source" else \
            kb.one(conn, f"SELECT id, url, [] AS alt_urls, (SELECT VALUE ok FROM snapshot WHERE target = $parent.id) AS oks FROM {t}")
        for r in rows:
            oks = r.get("oks") or []
            if any(oks) or (oks and not opts.retry_failed):
                continue
            todo.append((r["id"], r["url"]))
    if opts.limit:
        todo = todo[: opts.limit]
    print(f"archive: {ROOT}\nfetching {len(todo)} URLs (run {run})")
    # one WARC per worker thread avoids interleaved writes
    results = Counter()
    lock = threading.Lock()
    global _warc_writer
    warc_path = ROOT / "warc" / f"{run}.warc.gz"
    warc_fh = open(warc_path, "ab")
    _warc_writer = WARCWriter(warc_fh, gzip=True)

    def work(item):
        target, url = item
        snap = fetch_one(target, url, warc_path, opts)
        with lock:
            kb.run(conn, "CREATE snapshot CONTENT $s", {"s": snap})
        with _log_lock, open(ROOT / "log.jsonl", "a") as fh:
            fh.write(json.dumps({k: (kb.rid_str(v) if hasattr(v, "table_name") else str(v) if isinstance(v, dt.datetime) else v)
                                 for k, v in snap.items()}, ensure_ascii=False) + "\n")
        return snap

    with ThreadPoolExecutor(max_workers=opts.workers) as ex:
        for i, fut in enumerate(as_completed([ex.submit(work, x) for x in todo]), 1):
            s = fut.result()
            results["ok" if s["ok"] else "failed"] += 1
            results[f"via:{s.get('via', '-')}"] += 1
            if i % 25 == 0 or i == len(todo):
                print(f"  {i}/{len(todo)} {dict(results)}", flush=True)
    warc_fh.close()


def cmd_verify(_opts) -> int:
    conn = kb.connect()
    rows = kb.one(conn, "SELECT id, sha256, archive_path FROM snapshot WHERE ok = true")
    bad = missing = 0
    for r in rows:
        p = ROOT / r["archive_path"]
        if not p.exists():
            missing += 1
            continue
        if hashlib.sha256(p.read_bytes()).hexdigest() != r["sha256"]:
            bad += 1
            print("MISMATCH", kb.rid_str(r["id"]), p)
    print(f"verified {len(rows) - bad - missing}/{len(rows)}; missing {missing}; mismatched {bad}")
    return 1 if bad or missing else 0


def cmd_status(_opts):
    conn = kb.connect()
    rows = kb.one(conn, """SELECT target.tb() AS t, ok, via, count() AS n FROM snapshot GROUP BY t, ok, via""")
    for r in rows:
        print(r)
    latest = kb.one(conn, """SELECT count() AS n FROM source WHERE count((SELECT id FROM snapshot WHERE target = $parent.id AND ok = true)) = 0 GROUP ALL""")
    print("sources without a good snapshot:", latest[0]["n"] if latest else 0)
    size = sum(p.stat().st_size for p in ROOT.rglob("*") if p.is_file()) if ROOT.exists() else 0
    print(f"archive size: {size / 1e6:.1f} MB at {ROOT}")


def cmd_pack(opts):
    out = Path(opts.out or Path.home() / f"bavovniatko-archive-{now():%Y%m%d}.tar.gz")
    with tarfile.open(out, "w:gz") as tar:
        tar.add(ROOT, arcname=ROOT.name)
    print("packed", out, f"{out.stat().st_size / 1e6:.1f} MB")


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    f = sub.add_parser("fetch")
    f.add_argument("--table", choices=["source", "media"])
    f.add_argument("--limit", type=int)
    f.add_argument("--workers", type=int, default=8)
    f.add_argument("--retry-failed", action="store_true")
    f.add_argument("--wayback-submit", action="store_true")
    f.add_argument("--video", action="store_true")
    sub.add_parser("verify")
    sub.add_parser("status")
    p = sub.add_parser("pack"); p.add_argument("--out")
    opts = ap.parse_args()
    sys.exit({"fetch": cmd_fetch, "verify": cmd_verify, "status": cmd_status, "pack": cmd_pack}[opts.cmd](opts) or 0)


if __name__ == "__main__":
    main()
