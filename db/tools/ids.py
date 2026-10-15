"""Record ids and schema definitions as text: the parts of the tooling that need no database and no packages.

webexport.py imports this and runs on plain system Python (the GitHub Pages workflow has no surrealdb or yaml), so
this module imports the standard library only. kb.py builds on it.

- `key_text()` and `ident_key()` write a record key the two ways the tools need: the textual form the dumps and the
  generated markdown use, and the SurrealQL literal the explorer's dataset loads
- `definitions()` parses the DEFINE statements of a schema migration or of the explorer's function library
- `ORDER`, `load_order()` and `schema_files()` give the dump and the explorer the same table order and migrations
"""
from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path

DB_DIR = Path(__file__).resolve().parent.parent

# --- record keys ---------------------------------------------------------------------------------------------------

WORD = re.compile(r"[A-Za-z0-9_]+")       # a key of word characters only needs no ⟨⟩


def angle(key: str) -> str:
    """A key in ⟨⟩, with any ⟩ inside escaped: source:⟨rusi-2025-watling-third-year⟩."""
    return "⟨" + key.replace("⟩", "\\⟩") + "⟩"


def key_text(key: str) -> str:
    """A string key as the dumps and the generated markdown write it: bare when it is all word characters, which
    includes the hex media keys that start with a digit (media:0abc…), else in ⟨⟩."""
    return key if WORD.fullmatch(key) else angle(key)


def ident_key(key: str) -> str:
    """A string key as a SurrealQL literal: bare only when it is an identifier. A key that starts with a digit is put
    in ⟨⟩ as well, so the parser cannot read it as a number."""
    return key if WORD.fullmatch(key) and not key[0].isdigit() else angle(key)


# --- schema and function definitions -------------------------------------------------------------------------------

@dataclass(frozen=True)
class Definition:
    """One DEFINE statement."""
    kind: str               # TABLE, FIELD, INDEX, ANALYZER, EVENT, FUNCTION, …
    name: str               # the table, field, index, analyzer or event name; fn::name for a function
    on: str | None          # the table a FIELD, INDEX or EVENT is defined ON
    params: str | None      # a FUNCTION's parameter list, as written
    clauses: str            # everything after the name, its ON table and its parameters, up to the `;`
    text: str               # the whole statement as written, from DEFINE through the `;`
    doc: tuple[str, ...]    # the `--` lines directly above the statement, without their dashes
    note: str | None        # a `--` comment after the `;` on the same line

    @property
    def comment(self) -> str | None:
        """Its COMMENT "…", as written (escapes kept)."""
        return comment_of(self.clauses)


COMMENT = re.compile(r"COMMENT\s+\"((?:[^\"\\]|\\.)*)\"")
_HEAD = re.compile(r"DEFINE\s+(\w+)\s+(?:(?:IF NOT EXISTS|OVERWRITE)\s+)?(fn::\w+|[\w.\[\]*]+)")
_ON = re.compile(r"\s*ON\s+(?:TABLE\s+)?(\w+)")
_PARAMS = re.compile(r"\(([^)]*)\)")
_NOTE = re.compile(r"[ \t]*--\s*(.*)")


def comment_of(clauses: str) -> str | None:
    m = COMMENT.search(clauses)
    return m.group(1) if m else None


def _statement_end(text: str, i: int) -> int:
    """The index of the `;` that ends the statement starting at i (outside strings, comments and brackets)."""
    depth, n = 0, len(text)
    while i < n:
        c = text[i]
        if c in "\"'":
            i += 1
            while i < n and text[i] != c:
                i += 2 if text[i] == "\\" else 1
        elif text.startswith("--", i):
            i = text.find("\n", i)
            if i < 0:
                return n
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == ";" and depth == 0:
            return i
        i += 1
    return n


def definitions(text: str) -> list[Definition]:
    """Every DEFINE statement of a .surql file, in order."""
    out, i, n = [], 0, len(text)
    while i < n:
        if text[i].isspace():
            i += 1
            continue
        if text.startswith("--", i):
            i = text.find("\n", i)
            i = n if i < 0 else i
            continue
        end = _statement_end(text, i)
        stmt = text[i:end + 1]
        head = _HEAD.match(stmt)
        if head:
            kind, name = head.group(1).upper(), head.group(2)
            rest, on, params = stmt[head.end():].rstrip(";"), None, None
            if kind == "FUNCTION":
                p = _PARAMS.match(rest)
                if p:
                    params, rest = p.group(1), rest[p.end():]
            elif kind in ("FIELD", "INDEX", "EVENT") and (m := _ON.match(rest)):
                on, rest = m.group(1), rest[m.end():]
            line_start = text.rfind("\n", 0, i) + 1
            doc: list[str] = []
            if line_start == i:
                for line in reversed(text[:line_start].splitlines()):
                    if not line.startswith("--"):
                        break
                    doc.insert(0, line[2:].strip())
            line_end = text.find("\n", end)
            note = _NOTE.fullmatch(text[end + 1:n if line_end < 0 else line_end])
            out.append(Definition(kind, name, on, params, rest.strip(), stmt, tuple(doc),
                                  (note.group(1).strip() or None) if note else None))
        i = end + 1
    return out


# --- tables and migrations -----------------------------------------------------------------------------------------

# Load order: vocabularies, entities, claims and their meta, the audit trail; then the rest alphabetically, then the
# relations (both of their ends must exist).
ORDER = ["kind", "era", "topic", "unit", "metric", "actor", "place", "event", "system", "work", "source", "term",
         "media", "snapshot", "claim", "observation", "passage", "question", "design_note", "revision"]


def load_order(tables, relations) -> list[str]:
    """Tables in load order: ORDER first, then the rest alphabetically, relations last."""
    return ([t for t in ORDER if t in tables] + sorted(t for t in tables if t not in ORDER and t not in relations)
            + sorted(t for t in tables if t in relations))


def schema_files(root: Path | None = None) -> list[Path]:
    """The ordered migrations db/schema/NNNN_*.surql (or another directory in the same layout, e.g. db/fixes)."""
    return sorted((root or DB_DIR / "schema").glob("[0-9][0-9][0-9][0-9]_*.surql"))
