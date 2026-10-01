"""Split the research markdown into ordered blocks with their section path.

A block is a maximal run of non-blank lines; a heading line always forms its own block.
Joining the blocks' text with blank lines reproduces the document (modulo runs of blank lines),
which is what makes the generated markdown round-trip.
"""
from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path

HEADING = re.compile(r"^(#{1,6})\s+(.*)$")
GENERATED_PREFIX = "<!-- Generated from the knowledge base"
SRC = re.compile(r"\[src:([^\]]+)\]")


@dataclass
class Block:
    order: int
    role: str                 # heading | prose | list | table | rule | other
    text: str
    section: str              # heading path, e.g. "3. Per-era notes / e9-counteroffensive-26 (…)"
    level: int = 0            # heading level for role == heading
    heading: str = ""         # this heading's own title (headings only)
    top: str = ""             # top-level (##) section title
    lines: list[str] = field(default_factory=list)
    tight: bool = False       # next block follows on the very next line (no blank line in between)


def join(blocks_: list[Block]) -> str:
    """Inverse of blocks(): reassemble the document text."""
    parts = []
    for i, b in enumerate(blocks_):
        parts.append(b.text)
        if i < len(blocks_) - 1:
            parts.append("\n" if b.tight else "\n\n")
    return "".join(parts) + "\n"


def role_of(line: str) -> str:
    s = line.lstrip()
    if HEADING.match(line):
        return "heading"
    if s.startswith("|"):
        return "table"
    if re.match(r"^(-{3,}|\*{3,}|_{3,})$", s):
        return "rule"
    if re.match(r"^([-*+]|\d+[.)])\s", s):
        return "list"
    return "prose"


def blocks(path: Path) -> list[Block]:
    out: list[Block] = []
    stack: list[tuple[int, str]] = []          # (level, title)
    buf: list[str] = []

    def section() -> str:
        return " / ".join(t for lvl, t in stack if lvl >= 2)

    def top() -> str:
        return next((t for lvl, t in stack if lvl == 2), "")

    def flush():
        if buf:
            out.append(Block(len(out), role_of(buf[0]), "\n".join(buf), section(), top=top(), lines=list(buf)))
            buf.clear()

    lines = path.read_text().splitlines()
    # generated files start with a marker comment (db/tools/render.py); it is not part of the content
    if lines and lines[0].startswith(GENERATED_PREFIX):
        lines = lines[1:]
        while lines and not lines[0].strip():
            lines = lines[1:]
    for line in lines:
        m = HEADING.match(line)
        if m:
            had_buf = bool(buf)
            flush()
            if had_buf:
                out[-1].tight = True
            lvl, title = len(m.group(1)), m.group(2).strip()
            while stack and stack[-1][0] >= lvl:
                stack.pop()
            stack.append((lvl, title))
            out.append(Block(len(out), "heading", line, section(), lvl, title, top(), [line], tight=True))
        elif not line.strip():
            if out and not buf:
                out[-1].tight = False
            flush()
        else:
            buf.append(line)
    flush()
    return out


def list_items(block: Block) -> list[str]:
    """Top-level items of a list block, each with its indented continuation/sub-items."""
    items: list[list[str]] = []
    for line in block.lines:
        if re.match(r"^([-*+]|\d+[.)])\s", line):      # top-level item (column 0)
            items.append([line])
        elif items:
            items[-1].append(line)
        else:
            items.append([line])
    return ["\n".join(i) for i in items]


def strip_bullet(text: str) -> str:
    return re.sub(r"^([-*+]|\d+[.)])\s+", "", text, count=1)


def table_rows(block: Block) -> tuple[list[str], list[list[str]]]:
    """(header cells, body rows) for a pipe table; handles escaped pipes minimally."""
    rows = []
    for line in block.lines:
        s = line.strip()
        if not s.startswith("|"):
            continue
        cells = [c.strip() for c in re.split(r"(?<!\\)\|", s.strip("|"))]
        rows.append(cells)
    if len(rows) >= 2 and all(re.fullmatch(r":?-{2,}:?", c) for c in rows[1] if c):
        return rows[0], rows[2:]
    return (rows[0] if rows else []), rows[1:]


_DESIGN = re.compile(r"^\d+\.\s*game\W+(and\W+)?sim\w*\s+relevance", re.I)
_QUESTION = re.compile(r"^\d+\.\s*open questions", re.I)
_SKIP = re.compile(r"^\d+\.\s*(sources|terms)\b", re.I)


def meta_section(top: str) -> str | None:
    """Classify a top-level (##) section: "design_note", "question", "skip" (not extracted) or None (claims)."""
    if _DESIGN.match(top or ""):
        return "design_note"
    if _QUESTION.match(top or ""):
        return "question"
    if _SKIP.match(top or ""):
        return "skip"
    return None


def src_ids(text: str) -> list[str]:
    ids = []
    for m in SRC.findall(text):
        for c in re.split(r"[,;]\s*(?:src:)?", m):
            c = c.strip()
            if c and c != "<id>" and c not in ids:
                ids.append(c)
    return ids
