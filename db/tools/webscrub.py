"""Take the game out of the public explorer's dataset (used by webexport.py).

The knowledge base exists to feed a game, and it keeps everything for that purpose. The public website, though,
presents the research about the war on its own terms, so this step:

- **Leaves out** the game-only material: the prior-art games topic (t08) and everything only it uses, the design
  notes, the game-feed tables (`sim_*`), and the "game/sim relevance" sections of the research texts.
- **Renames** the game framing: "diorama" becomes "dossier area", "chapter" becomes "dossier", "win band" becomes
  "range", and "playable area" becomes "map area".
- **Drops** claims that are still about designing the game after renaming ("In the game, …", "for a commercial
  game", …), with their observations and edges, plus anything only they used: entities, sources, metrics, kinds
  and glossary terms.
- **Checks** what remains: `leftovers()` lists any game wording that isn't on the short whitelist of genuine war
  facts (US–Ukrainian tabletop war games, the "Ghost of Kyiv" clip being DCS World game footage, Spiderweb's
  "long game").

Works on the dump's tagged JSON rows; record ids are `{"$rid": [table, key]}`.
"""
from __future__ import annotations

import re

DROP_TABLES = {"design_note", "sim_loadout", "sim_param", "sim_series", "sim_terrain_profile"}
DROP_TOPICS = {"t08"}                                   # Prior-art games & wargames
# Fields that are not shown on the site or are internal bookkeeping.
DROP_FIELDS = {"legacy_ids", "geometry_source"}
DROP_EXT = {"topic_tags", "document", "publisher_raw", "date_raw"}
# Text that is file paths or identifiers, not prose: never rewritten or scanned.
LITERAL_FIELDS = {"url", "file", "key", "code", "id", "local_id", "osm", "alt_urls", "geocode_query"}

GAME = re.compile(r"\b(game|games|gameplay|in-game|gamif\w*|player|players|diorama|dioramas|win conditions?|"
                  r"win bands?|wargam\w*|playable|chapter|chapters)\b", re.I)
# Genuine facts about the war that use these words; they stay.
WHITELIST = re.compile(r"(war ?games?\b|wargam(es?|ing)\b(?=.{0,80}(counteroffensive|Zaporizhzhia|Ukrain|2022|2023))|"
                       r"(?<=the )game DCS World|DCS World game|long game)", re.I)
# Design talk that marks a claim as being about the game itself.
DESIGN = re.compile(r"\b((in|for|into|to) (the|a|our) game|game[- ]ready|game assets|game HUD|ship\w* (it |them )?"
                    r"(in|with) the game|commercial game|playable|win bands?|win conditions?|in-game|player)\b", re.I)

RENAMES = [
    (re.compile(r"\bgame map\b"), "map"), (re.compile(r"\bGame map\b"), "Map"),
    (re.compile(r"\bThe diorama: ", re.I), ""),                        # section "3. The diorama: geography, …"
    (re.compile(r"\bplayable (area|box|map box|map)\b", re.I), r"map \1"),
    (re.compile(r"\bmap map box\b", re.I), "map box"),
    (re.compile(r"\ba win band\b"), "an outcome range"), (re.compile(r"\bwin bands?\b"), "outcome range"),
    (re.compile(r"\bWin bands?\b"), "Outcome range"),
    (re.compile(r"\bdioramas\b"), "dossier areas"), (re.compile(r"\bDioramas\b"), "Dossier areas"),
    (re.compile(r"\bdiorama\b"), "dossier area"), (re.compile(r"\bDiorama\b"), "Dossier area"),
    (re.compile(r"\bchapters\b"), "dossiers"), (re.compile(r"\bChapters\b"), "Dossiers"),
    (re.compile(r"\bchapter\b"), "dossier"), (re.compile(r"\bChapter\b"), "Dossier"),
]


# Record ids that carry the game framing in their key, renamed everywhere they appear.
RENAME_IDS = {("metric", "game_map_extent"): "map_extent", ("metric", "game_map_area"): "map_area",
              ("place", "bakhmut_diorama"): "bakhmut_dossier_area"}


def rename_ids(v):
    if isinstance(v, dict):
        if "$rid" in v:
            t, k = v["$rid"]
            return {"$rid": [t, RENAME_IDS.get((t, k), k)]} if isinstance(k, str) else v
        return {k: rename_ids(x) for k, x in v.items()}
    if isinstance(v, list):
        return [rename_ids(x) for x in v]
    return v


def rename(s: str) -> str:
    for pat, rep in RENAMES:
        s = pat.sub(rep, s)
    return s


def has_game(s: str) -> bool:
    return bool(GAME.search(WHITELIST.sub("", s)))


def rid(v) -> tuple | None:
    if isinstance(v, dict) and "$rid" in v:
        t, k = v["$rid"]
        return (t, tuple(k) if isinstance(k, list) else k)
    return None


def refs(v, out: set):
    """Every record id inside a value."""
    r = rid(v)
    if r:
        out.add(r)
    elif isinstance(v, dict):
        for k, x in v.items():
            if k != "id":
                refs(x, out)
    elif isinstance(v, list):
        for x in v:
            refs(x, out)
    return out


# Names and titles are what they are (a publication called "Game of drones", the DCS World "video game" kind,
# an author's company), so entity names and source titles are not scanned.
NAME_FIELDS = {("labels", "en"), ("aliases",), ("title",)}


def strings(v, path=()):
    """(field path, text) for every prose string in a row."""
    if isinstance(v, dict):
        if any(k.startswith("$") for k in v):
            return
        for k, x in v.items():
            if k not in LITERAL_FIELDS:
                yield from strings(x, path + (k,))
    elif isinstance(v, list):
        for x in v:
            yield from strings(x, path)
    elif isinstance(v, str):
        yield path, v


def rewrite(v, field=None):
    if isinstance(v, dict):
        if any(k.startswith("$") for k in v):
            return v
        return {k: (x if k in LITERAL_FIELDS else rewrite(x, k)) for k, x in v.items()}
    if isinstance(v, list):
        return [rewrite(x, field) for x in v]
    if isinstance(v, str):
        return rename(v)
    return v


def topics_of(row) -> set:
    return {rid(t)[1] for t in row.get("topics", []) if rid(t)} | ({rid(row["topic"])[1]} if rid(row.get("topic")) else set())


def is_game_section(section: str) -> bool:
    return bool(re.search(r"game|sim relevance|design lessons|prior[- ]art", section or "", re.I))


ENTITY_TABLES = {"actor", "system", "work", "event", "place", "source", "media", "term", "metric", "kind", "unit",
                 "era", "topic"}


def scrub(tables: dict[str, list[dict]], relations: set[str]) -> tuple[dict[str, list[dict]], dict]:
    report = {}
    tables = {t: rows for t, rows in tables.items() if t not in DROP_TABLES}
    tables["topic"] = [r for r in tables.get("topic", []) if rid(r["id"])[1] not in DROP_TOPICS]

    # 1. Claims: drop the game-only topic, rename the framing, then drop what is still about the game.
    dropped = set()
    kept_claims = []
    for r in tables["claim"]:
        cid = rid(r["id"])
        r = rewrite(r)
        text = r.get("text", "")
        if topics_of(r) & DROP_TOPICS or DESIGN.search(text) or has_game(text):
            dropped.add(cid)
        else:
            kept_claims.append(r)
    for r in kept_claims:                               # extraction notes: "OUTCOME; win condition: …"
        note = r.get("confidence_note")
        if isinstance(note, str):
            note = re.sub(r"^OUTCOME\s*[;:,.]?\s*", "", note).strip()
            if not note or has_game(note):
                r.pop("confidence_note", None)
            else:
                r["confidence_note"] = note[:1].upper() + note[1:]
    tables["claim"] = kept_claims
    report["claims_dropped"] = len(dropped)

    # 2. Rows that hang off claims; edges to dropped claims; game sections of the research texts.
    tables["observation"] = [r for r in tables.get("observation", []) if rid(r.get("claim")) not in dropped]
    passages_dropped = set()
    kept = []
    for r in tables.get("passage", []):
        r2 = rewrite(r)
        if topics_of(r) & DROP_TOPICS or is_game_section(r.get("section", "")) or has_game(r2.get("markdown", "")):
            passages_dropped.add(rid(r["id"]))
        else:
            kept.append(r2)
    tables["passage"] = kept
    tables["question"] = [rewrite(r) for r in tables.get("question", [])
                          if not topics_of(r) & DROP_TOPICS and not has_game(rename(r.get("text", "")))]
    gone = dropped | passages_dropped
    for t in relations:
        if t in tables:
            tables[t] = [r for r in tables[t] if rid(r.get("in")) not in gone and rid(r.get("out")) not in gone]
    if "counters" in tables:
        for r in tables["counters"]:
            r["evidence"] = [e for e in r.get("evidence", []) if rid(e) not in dropped]
        tables["counters"] = [r for r in tables["counters"] if r["evidence"]]

    # 3. Entities: keep what anything kept refers to; drop unreferenced ones with game wording.
    tables["term"] = [r for r in tables.get("term", []) if not has_game(str(r.get("group", "")))]
    entity_rows = {t: tables.get(t, []) for t in ENTITY_TABLES}
    used: set = set()
    for t, rows in tables.items():
        if t not in ENTITY_TABLES:
            for r in rows:
                refs(r, used)
    for _ in range(6):                                  # entities refer to entities (parent, publisher, kind)
        before = len(used)
        for t, rows in entity_rows.items():
            for r in rows:
                if rid(r["id"]) in used:
                    refs(r, used)
        if len(used) == before:
            break
    for t, rows in entity_rows.items():
        keep = []
        for r in rows:
            r2 = rewrite(r)
            game_words = any(has_game(s) for _, s in strings(r2))
            if t == "media" and str(r.get("ext", {}).get("document", "")).startswith("research/08"):
                continue
            if rid(r["id"]) in used or not game_words or t in ("era", "unit", "topic"):
                keep.append(r2)
        tables[t] = keep
    report["entities_dropped"] = {t: len(entity_rows[t]) - len(tables[t]) for t in entity_rows if len(entity_rows[t]) != len(tables[t])}

    # 4. Strip internal fields, and blank prose fields on kept entities that still talk about the game.
    for t, rows in tables.items():
        for r in rows:
            for f in DROP_FIELDS:
                r.pop(f, None)
            if isinstance(r.get("ext"), dict):
                for f in DROP_EXT:
                    r["ext"].pop(f, None)
            if t in ENTITY_TABLES:
                for f in ("notes", "note", "summary", "description"):
                    if isinstance(r.get(f), str) and has_game(r[f]):
                        del r[f]
            if t in relations | {"observation"}:
                for f in ("rationale", "note", "effect"):
                    if isinstance(r.get(f), str) and has_game(r[f]):
                        del r[f]
    report["passages_dropped"] = len(passages_dropped)
    tables = {t: [rename_ids(r) for r in rows] for t, rows in tables.items()}
    return tables, report


def leftovers(tables: dict[str, list[dict]]) -> list[str]:
    """Game wording still in the dataset (should be empty)."""
    out = []
    for t, rows in tables.items():
        for r in rows:
            for path, s in strings(r):
                if t in ENTITY_TABLES and path in NAME_FIELDS:
                    continue
                if has_game(s):
                    out.append(f"{t}.{'.'.join(path)} {rid(r.get('id'))}: {s[:140]}")
    return out
