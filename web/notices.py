#!/usr/bin/env python3
"""Write the explorer's third-party licence notices into crates/app/public/licenses/ (copied into the site).

  web/notices.py     run by web/build.sh after the bridge is bundled and before Trunk builds the site

- **THIRD-PARTY-NOTICES.txt:** every Rust crate linked into the two wasm binaries (kbapp and kbworker: normal
  dependencies for wasm32, without proc-macros or build scripts) and every npm package esbuild bundled (from its
  metafile), with name, version, licence and the full licence texts found in the package. Identical texts are
  printed once.
- **SurrealDB-BSL-1.1.txt:** SurrealDB's Business Source License, which must be displayed with every copy.
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

WEB = Path(__file__).resolve().parent
OUT = WEB / "crates" / "app" / "public" / "licenses"
BRIDGE = WEB / "bridge"
METAFILE = WEB / "crates" / "app" / "bridge" / "meta.json"
ROOTS = {"kbapp", "kbworker"}
LICENSE_NAMES = ("LICENSE", "LICENCE", "COPYING", "NOTICE", "UNLICENSE", "COPYRIGHT")
REPO = WEB.parent
# npm packages that declare no licence in package.json; their GitHub repository (surrealdb/codemirror) is Apache-2.0.
NPM_LICENSE = {"@surrealdb/codemirror": "Apache-2.0", "@surrealdb/lezer": "Apache-2.0"}
MIT_TEMPLATE = (REPO / "LICENSE-MIT").read_text().split("\n", 3)[3]   # the MIT text without our copyright line

HEADER = """Third-party software in the bavovniatko knowledge-base explorer
=================================================================

This site's own code is licensed under MIT OR Apache-2.0, and its content under CC BY 4.0, with OpenStreetMap-derived
geometry under ODbL 1.0 (see LICENSE-CONTENT.md in https://github.com/barafael/bavovniatko).

The site also contains the third-party software listed below, each under its own licence.

IMPORTANT: SurrealDB (the surrealdb* crates) is licensed under the Business Source License 1.1, not an open-source
licence. Its full text is in SurrealDB-BSL-1.1.txt next to this file. This site uses SurrealDB under the licence's
Additional Use Grant: visitors query a read-only copy and cannot create, manage or control schemas or tables.

"""


def licence_files(directory: Path) -> list[Path]:
    if not directory.is_dir():
        return []
    return sorted(p for p in directory.iterdir()
                  if p.is_file() and p.name.upper().startswith(LICENSE_NAMES) and p.suffix.lower() not in (".rs", ".js"))


def rust_packages() -> list[dict]:
    meta = json.loads(subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--filter-platform", "wasm32-unknown-unknown", "--locked"],
        cwd=WEB, check=True, capture_output=True, text=True).stdout)
    nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
    pkgs = {p["id"]: p for p in meta["packages"]}
    todo = [p["id"] for p in meta["packages"] if p["name"] in ROOTS]
    seen: set[str] = set()
    while todo:
        i = todo.pop()
        if i in seen:
            continue
        seen.add(i)
        for d in nodes[i]["deps"]:
            if any(k["kind"] is None for k in d["dep_kinds"]):      # normal dependencies only
                todo.append(d["pkg"])
    out = []
    for i in seen:
        p = pkgs[i]
        if p["name"] in ROOTS or p["source"] is None:            # our own crates
            continue
        if all("proc-macro" in t["kind"] for t in p["targets"]):  # compile-time only, not in the binary
            continue
        out.append({"name": p["name"], "version": p["version"], "license": p.get("license") or "see licence file",
                    "repository": p.get("repository") or "", "authors": ", ".join(p.get("authors") or []),
                    "files": licence_files(Path(p["manifest_path"]).parent)})
    return sorted(out, key=lambda p: (p["name"], p["version"]))


def npm_packages() -> list[dict]:
    names: set[str] = {"maplibre-gl"}                             # its worker files are copied as well as bundled
    if METAFILE.exists():
        for path in json.loads(METAFILE.read_text())["inputs"]:
            parts = path.split("node_modules/")[-1].split("/") if "node_modules/" in path else []
            if parts:
                names.add("/".join(parts[:2]) if parts[0].startswith("@") else parts[0])
    out = []
    for n in sorted(names):
        d = BRIDGE / "node_modules" / n
        try:
            pj = json.loads((d / "package.json").read_text())
        except FileNotFoundError:
            continue
        repo = pj.get("repository")
        author = pj.get("author")
        out.append({"name": n, "version": pj.get("version", ""),
                    "license": pj.get("license") or NPM_LICENSE.get(n, "see licence file"),
                    "repository": repo.get("url", "") if isinstance(repo, dict) else (repo or ""),
                    "authors": author.get("name", "") if isinstance(author, dict) else (author or ""),
                    "files": licence_files(d)})
    return out


def surrealdb_licence(rust: list[dict]) -> str:
    for p in rust:
        if p["name"] == "surrealdb-core":
            for f in p["files"]:
                if f.name.upper().startswith("LICENSE"):
                    return f.read_text(errors="replace")
    sys.exit("surrealdb-core's LICENSE not found; run `cargo fetch` in web/ first")


def main() -> int:
    rust, npm = rust_packages(), npm_packages()
    texts: dict[str, list[str]] = {}
    lines = [HEADER]
    for title, pkgs in (("Rust crates (compiled into the WebAssembly)", rust), ("npm packages (bundled JavaScript)", npm)):
        lines.append(f"{title}: {len(pkgs)}\n" + "-" * 72)
        for p in pkgs:
            lines.append(f"{p['name']} {p['version']}  —  {p['license']}" + (f"  —  {p['repository']}" if p["repository"] else ""))
            for f in p["files"]:
                texts.setdefault(f.read_text(errors="replace").strip(), []).append(f"{p['name']} {p['version']} ({f.name})")
            if not p["files"]:
                # No licence file published: include the standard text of the declared licence(s), with the authors.
                lic, who = p["license"], p["authors"] or f"the {p['name']} authors"
                label = f"{p['name']} {p['version']} (no licence file published; standard text of its declared licence)"
                if "MIT" in lic:
                    texts.setdefault(f"MIT License\n\nCopyright (c) {who}\n\n{MIT_TEMPLATE}".strip(), []).append(label)
                if "Apache-2.0" in lic:
                    texts.setdefault((REPO / "LICENSE-APACHE").read_text().strip(), []).append(label)
        lines.append("")
    lines.append("Licence texts\n" + "=" * 72)
    for text, used_by in sorted(texts.items(), key=lambda kv: kv[1][0]):
        lines.append(f"\nUsed by: {', '.join(used_by)}\n" + "-" * 72 + f"\n{text}\n")
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "THIRD-PARTY-NOTICES.txt").write_text("\n".join(lines))
    (OUT / "SurrealDB-BSL-1.1.txt").write_text(surrealdb_licence(rust))
    missing = [p["name"] for p in rust + npm if not p["files"] and not any(k in p["license"] for k in ("MIT", "Apache-2.0"))]
    print(f"notices: {len(rust)} crates, {len(npm)} npm packages, {len(texts)} distinct licence texts -> {OUT}"
          + (f"; no licence file in: {', '.join(missing)}" if missing else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
