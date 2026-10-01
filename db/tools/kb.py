"""Shared client for the bavovniatko knowledge base (SurrealDB 3.3 via the Python SDK 2.0).

Everything that talks to the database goes through `connect()` and `run()`:
- credentials and ns/db come from db/.env
- `run()` executes one or more statements and raises on the first ERR status
- `geom()` turns GeoJSON dicts into the SDK geometry classes SurrealDB accepts
- `rid()` builds record ids, including array ids such as kind:["place","settlement"]
"""
from __future__ import annotations

import os
import re
from pathlib import Path
from typing import Any

import surrealdb.data.cbor as _cbor
from surrealdb import RecordID, Surreal
from surrealdb.data.types.geometry import (
    GeometryCollection, GeometryLine, GeometryMultiLine, GeometryMultiPoint,
    GeometryMultiPolygon, GeometryPoint, GeometryPolygon,
)

DB_DIR = Path(__file__).resolve().parent.parent
REPO = DB_DIR.parent
RESEARCH = REPO / "research"

# SurrealDB 3 encodes the `set` type as CBOR tag 56, which SDK 2.0.0 cannot decode yet.
_TAG_SET = 56
_orig_tag_decoder = _cbor.tag_decoder


def _tag_decoder(decoder, tag, shareable_index=None):
    if tag.tag == _TAG_SET:
        return list(tag.value)
    return _orig_tag_decoder(decoder, tag, shareable_index)


_cbor.tag_decoder = _tag_decoder


class KBError(RuntimeError):
    pass


def env() -> dict[str, str]:
    vals = {}
    for line in (DB_DIR / ".env").read_text().splitlines():
        if "=" in line and not line.lstrip().startswith("#"):
            k, v = line.split("=", 1)
            vals[k.strip()] = v.strip()
    vals.update({k: v for k, v in os.environ.items() if k.startswith("SURREAL_")})
    return vals


def connect(ns: str | None = None, db: str | None = None) -> Surreal:
    e = env()
    conn = Surreal(f"ws://127.0.0.1:{e['SURREAL_PORT']}")
    conn.signin({"username": e["SURREAL_USER"], "password": e["SURREAL_PASS"]})
    conn.use(ns or e["SURREAL_NS"], db or e["SURREAL_DB"])
    return conn


def run(conn: Surreal, query: str, vars: dict[str, Any] | None = None) -> list[Any]:
    """Run statements; return each statement's result; raise KBError on any failure."""
    raw = conn.query_raw(query, vars or {})
    if "error" in raw:
        raise KBError(raw["error"].get("message", raw["error"]))
    out = []
    for i, res in enumerate(raw["result"]):
        if res["status"] != "OK":
            raise KBError(f"statement {i + 1} failed: {res['result']}")
        out.append(res["result"])
    return out


def one(conn: Surreal, query: str, vars: dict[str, Any] | None = None) -> Any:
    return run(conn, query, vars)[-1]


_SAFE_ID = re.compile(r"^[A-Za-z0-9_]+$")


def rid(table: str, key: Any) -> RecordID:
    return RecordID(table, key)


def rid_str(r: RecordID) -> str:
    """Stable textual form, used in dumps and generated markdown."""
    k = r.id
    if isinstance(k, str):
        return f"{r.table_name}:{k}" if _SAFE_ID.match(k) else f"{r.table_name}:⟨{k}⟩"
    if isinstance(k, list):
        return f"{r.table_name}:[" + ", ".join(f'"{x}"' if isinstance(x, str) else str(x) for x in k) + "]"
    return f"{r.table_name}:{k}"


def _pt(c):
    return GeometryPoint(float(c[0]), float(c[1]))


def _line(cs):
    return GeometryLine(*[_pt(c) for c in cs])


def _poly(rings):
    return GeometryPolygon(_line(rings[0]), *[_line(r) for r in rings[1:]])


def geom(g: dict | None):
    """GeoJSON geometry dict -> SDK geometry object (lon, lat order, WGS84)."""
    if g is None:
        return None
    t, c = g["type"], g.get("coordinates")
    if t == "Point":
        return _pt(c)
    if t == "LineString":
        return _line(c)
    if t == "Polygon":
        return _poly(c)
    if t == "MultiPoint":
        return GeometryMultiPoint(*[_pt(p) for p in c])
    if t == "MultiLineString":
        return GeometryMultiLine(*[_line(l) for l in c])
    if t == "MultiPolygon":
        return GeometryMultiPolygon(*[_poly(p) for p in c])
    if t == "GeometryCollection":
        return GeometryCollection(*[geom(x) for x in g["geometries"]])
    raise ValueError(f"unsupported geometry type {t}")


CODE = r"(?:\d\d|[cv]\d\d)"          # document codes: 00–08 (topic files), cNN (chapters), vNN (vignettes)


def topic_key(code: str) -> str:
    """Document code -> topic record key: "03" -> "t03", "c02" -> "c02"."""
    return f"t{code}" if code.isdigit() else code


def documents() -> list[Path]:
    """All knowledge-base documents: research/NN-*.md and research/chapters/[cv]NN-*.md."""
    return sorted(RESEARCH.glob("0[0-9]-*.md")) + sorted((RESEARCH / "chapters").glob("[cv][0-9][0-9]-*.md"))


def doc_code(path: Path) -> str:
    return path.name.split("-", 1)[0]


def slug(text: str, maxlen: int = 60) -> str:
    s = re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")
    return s[:maxlen].rstrip("_") or "x"
