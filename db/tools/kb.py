"""Shared client for the bavovniatko knowledge base (SurrealDB 3.3 via the Python SDK 2.0).

Everything that talks to the database goes through `connect()` and `run()`:
- credentials and ns/db come from db/.env
- `run()` executes one or more statements and raises on the first ERR status
- `geom()` turns GeoJSON dicts into the SDK geometry classes SurrealDB accepts
- `rid()` builds record ids, including array ids such as kind:["place","settlement"]
- `prov()`, `time_of()` and `plain()` give every tool the same provenance, time and JSON shapes
- `Writer` is the one write path for loaders: it skips records whose content hash (ext._hash) has not changed
- the enums mirror the schema's closed string sets, so code compares against names rather than literals
"""
from __future__ import annotations

import datetime as dt
import decimal
import hashlib
import json
import os
import re
from collections import Counter
from enum import StrEnum
from functools import cache
from pathlib import Path
from typing import Any

import surrealdb.data.cbor as _cbor
import yaml
from surrealdb import RecordID, Surreal
from surrealdb.data.types.geometry import (
    Geometry,
    GeometryCollection,
    GeometryLine,
    GeometryMultiLine,
    GeometryMultiPoint,
    GeometryMultiPolygon,
    GeometryPoint,
    GeometryPolygon,
)

import ids
from ids import DB_DIR, schema_files

REPO = DB_DIR.parent
# Paths stored in records (`anchor.file`, `prov.input`, `topic.file`, …) are relative to db/: "research/03-fires-air.md".
RESEARCH = DB_DIR / "research"
VOCAB = DB_DIR / "vocab"
UTC = dt.timezone.utc
NOW = dt.datetime.now(UTC)                 # one timestamp per process: every record a run writes carries it
STAMP = "%Y%m%dT%H%M%SZ"                   # the run id's time format, e.g. load@20260101T120000Z


class Precision(StrEnum):
    """How precisely a time is known (every `time`-shaped field)."""
    DAY = "day"
    MONTH = "month"
    QUARTER = "quarter"
    YEAR = "year"

    @staticmethod
    def of(date: str) -> Precision:
        """'2025' -> year, '2025-06' -> month, '2025-06-01' -> day."""
        return (Precision.YEAR, Precision.MONTH, Precision.DAY)[str(date).count("-")]


def date_of(v) -> dt.datetime:
    """'2025' | '2025-06' | '2025-06-01' | 2025 | date | datetime -> UTC datetime at the start of the period."""
    if isinstance(v, dt.datetime):
        return v.astimezone(UTC)
    if isinstance(v, dt.date):
        return dt.datetime(v.year, v.month, v.day, tzinfo=UTC)
    y, m, d = (str(v).strip().split("-") + ["1", "1"])[:3]
    return dt.datetime(int(y), int(m), int(d), tzinfo=UTC)


def time_of(frm, to=None, precision: str | None = None) -> dict:
    """A partial date (and optional end) -> the schema's time object {from, precision[, to]}.
    `to` keeps the start of its last period; the precision covers both ends and defaults to what `frm` states."""
    out = {"from": date_of(frm), "precision": precision or Precision.of(frm)}
    if to:
        out["to"] = date_of(to)
    return out


class Side(StrEnum):
    """Whose side an actor is on (observations allow all but INTERNATIONAL)."""
    UA = "ua"
    RU = "ru"
    WESTERN = "western"
    INTERNATIONAL = "international"
    OTHER = "other"


OBSERVATION_SIDES = [Side.UA, Side.RU, Side.WESTERN, Side.OTHER]


class Qualifier(StrEnum):
    """How an observation's value is meant (observation.qualifier)."""
    EXACT = "exact"
    APPROX = "approx"
    AT_LEAST = "at_least"
    AT_MOST = "at_most"
    RANGE = "range"


class ObsMethod(StrEnum):
    """Where an observation's value comes from (observation.method)."""
    REPORTED = "reported"
    ESTIMATED = "estimated"
    DERIVED = "derived"
    COMPUTED = "computed"


class Relation(StrEnum):
    """The claim-to-claim edge tables."""
    SUPPORTS = "supports"
    WEAKENS = "weakens"
    CONTRADICTS = "contradicts"
    REFINES = "refines"
    SUPERSEDES = "supersedes"
    DUPLICATES = "duplicates"


@cache
def vocab(name: str) -> Any:
    """A controlled vocabulary, db/vocab/<name>.yaml."""
    return yaml.safe_load((VOCAB / f"{name}.yaml").read_text())


def jsonl(path: Path) -> list[dict]:
    """The records of a JSON-lines file (blank lines skipped)."""
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def run_stamp(prefix: str, at: dt.datetime | None = None) -> str:
    """A run id: '<tool>@<UTC time>'."""
    return f"{prefix}@{(at or NOW).strftime(STAMP)}"


def prov(by: str, method: str, input: str | None = None, run: str | None = None) -> dict:
    """The provenance object every written record carries (schema field `prov`)."""
    p = {"by": by, "method": method, "run": run or run_stamp(by), "at": NOW}
    if input:
        p["input"] = input
    return p

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


def rid(table: str, key: Any) -> RecordID:
    return RecordID(table, key)


def rid_of(s: str) -> RecordID:
    """'table:key' -> record id (plain string keys, as in the registry and staging files)."""
    table, key = s.split(":", 1)
    return RecordID(table, key)


def claim_rid(local_id: str) -> RecordID:
    """Staging local id 'c03-0004-01' -> claim:cc03_0004_01."""
    return RecordID("claim", "c" + local_id.replace("-", "_"))


def rid_str(r: RecordID) -> str:
    """Stable textual form, used in dumps and generated markdown."""
    k = r.id
    if isinstance(k, str):
        return f"{r.table_name}:{ids.key_text(k)}"
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


_GEO_TYPES = {"GeometryPoint": "Point", "GeometryLine": "LineString", "GeometryPolygon": "Polygon",
              "GeometryMultiPoint": "MultiPoint", "GeometryMultiLine": "MultiLineString",
              "GeometryMultiPolygon": "MultiPolygon"}


def geo_json(g) -> dict:
    """SDK geometry object -> GeoJSON geometry dict (inverse of geom())."""
    if isinstance(g, GeometryCollection):
        return {"type": "GeometryCollection", "geometries": [geo_json(x) for x in g.geometries]}
    return {"type": _GEO_TYPES[type(g).__name__], "coordinates": g.get_coordinates()}


def iso_z(d: dt.datetime) -> str:
    """UTC ISO-8601 with a Z suffix, the dump's datetime form."""
    return d.astimezone(UTC).isoformat().replace("+00:00", "Z")


def plain(v, *, rid=rid_str, dt_=iso_z, drop=()):
    """SurrealDB value -> JSON-safe Python. Record ids go through `rid`, datetimes through `dt_`; geometry becomes
    {"$geo": GeoJSON} and decimals {"$dec": str}; dicts lose None values and the keys in `drop` (at every level)."""
    if isinstance(v, RecordID):
        return rid(v)
    if isinstance(v, dt.datetime):
        return dt_(v)
    if isinstance(v, (Geometry, GeometryCollection)):
        return {"$geo": geo_json(v)}
    if isinstance(v, decimal.Decimal):
        return {"$dec": str(v)}
    if isinstance(v, dict):
        return {k: plain(x, rid=rid, dt_=dt_, drop=drop) for k, x in v.items() if x is not None and k not in drop}
    if isinstance(v, (list, tuple)):
        return [plain(x, rid=rid, dt_=dt_, drop=drop) for x in v]
    if hasattr(v, "to_string") or type(v).__name__ == "Duration":
        return str(v)
    return v


def content_hash(data: dict) -> str:
    """The hash `Writer` keeps in ext._hash: the record without its provenance, as sorted JSON."""
    body = {k: v for k, v in data.items() if k != "prov"}
    as_json = lambda o: plain(o, dt_=dt.datetime.isoformat)
    return hashlib.sha256(json.dumps(body, sort_keys=True, default=as_json).encode()).hexdigest()[:16]


class Writer:
    """Writes records, skipping the ones whose content has not changed since the last run, so re-running a loader
    is safe and the audit trail stays meaningful. The content hash of every record written lives in ext._hash.

    `put` upserts a record (MERGE, never CONTENT: an actor is shared between the imported sources and the curated
    registry, so no loader may wipe fields it does not own). A loader that writes a record its own way, as load.py
    writes a claim in one transaction with its edges, asks `changed` first and reports with `wrote`. A loader whose
    stored hashes were made another way passes its own `h`, so its re-runs keep recognising them."""

    def __init__(self, conn: Surreal):
        self.conn = conn
        self.hashes: dict[str, dict[str, str | None]] = {}     # table -> record id -> its stored hash
        self.stats = Counter()

    def existing(self, table: str) -> dict[str, str | None]:
        if table not in self.hashes:
            rows = one(self.conn, f"SELECT id, ext._hash AS h FROM {table}")
            self.hashes[table] = {rid_str(r["id"]): r.get("h") for r in rows}
        return self.hashes[table]

    def changed(self, rid: RecordID, h: str) -> bool:
        """Whether the stored record's hash differs from `h`; counts the record as unchanged when it does not."""
        if self.existing(rid.table_name).get(rid_str(rid)) == h:
            self.stats[f"{rid.table_name}:unchanged"] += 1
            return False
        return True

    def wrote(self, rid: RecordID, h: str):
        """Record that the caller has written `rid` with content hash `h`."""
        ex, key = self.existing(rid.table_name), rid_str(rid)
        self.stats[f"{rid.table_name}:{'updated' if key in ex else 'created'}"] += 1
        ex[key] = h

    def put(self, rid: RecordID, data: dict, h: str | None = None) -> bool:
        """Upsert the record unless it is unchanged; returns whether it was written."""
        h = h or content_hash(data)
        if not self.changed(rid, h):
            return False
        data = {**data, "ext": {**(data.get("ext") or {}), "_hash": h}}
        run(self.conn, "UPSERT $id MERGE $data", {"id": rid, "data": data})
        self.wrote(rid, h)
        return True


def define_db(ns: str, db: str, *, exists_ok: bool = False) -> Surreal:
    """Create the namespace and a STRICT database; returns the root connection."""
    root = connect()
    run(root, f"DEFINE NAMESPACE IF NOT EXISTS {ns}; USE NS {ns}; "
              f"DEFINE DATABASE {'IF NOT EXISTS ' if exists_ok else ''}{db} STRICT;")
    return root


def fresh_db(ns: str, db: str) -> tuple[Surreal, Surreal]:
    """A new database with the whole schema applied: (root connection, connection to the new database)."""
    root = define_db(ns, db)
    conn = connect(ns, db)
    for f in schema_files():
        run(conn, f.read_text())
    return root, conn


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
