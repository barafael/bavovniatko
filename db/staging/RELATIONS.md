# Judging relations between claims: brief

The rules in `db/tools/relate.py candidates` find **candidate pairs** of claims that might bear on each
other: same metric and period, shared specific entities, or near-identical wording. Your job is to judge
each pair. The judgments become graph edges that the game and the research rely on, so precision matters
more than recall. **"none" is a perfectly good answer.**

## Input and output

- **Input:** `db/staging/relations/batches/bNNN.jsonl`. Each line is one pair:
  `{pair, rules, hints, detail, a: {id, text, kind, epistemic, claimants, sources, time, eras, topic}, b: {...}}`.
  The `hints` ("supports?", "contradicts?", "related?", "duplicates?") are only the rule's guess.
- **Output:** `db/staging/relations/judged/bNNN.jsonl` (same NNN), with exactly one line per pair:
  ```json
  {"pair": "…", "relation": "supports|weakens|contradicts|refines|supersedes|duplicates|none",
   "from": "claim:…", "to": "claim:…", "strength": 0.0-1.0, "rationale": "one sentence",
   "resolution": "optional, contradicts only", "independent": true}
  ```
  For `"relation": "none"`, `from`, `to` and `strength` may be omitted, but still give a short rationale.
  `independent` applies to `supports` only.
- **Check:** `db/.venv/bin/python db/tools/relate.py validate bNNN [bNNN …]` must print `OK`.
- **Sources:** you may read `research/*.md` for context. Edit no other files and don't use the web.

## Relations and direction

| relation | meaning | direction |
|---|---|---|
| `supports` | A makes B more likely. It corroborates B, independently or not. | from = the supporting claim, to = the supported claim. For mutual corroboration pick either. Set `independent: true` only when the two rest on different original evidence, not one outlet relaying another. |
| `weakens` | A makes B less likely or casts doubt on it, without outright conflict: a different method, a caveat, a lower-quality basis. | from weakens to |
| `contradicts` | A and B cannot both be true **as stated**: the same quantity, scope and time with incompatible values or assertions. | Symmetric; order doesn't matter. If the conflict is explicable (different claimants' methods, killed vs killed+wounded, a different period or area), say so in `resolution`. If the difference is *only* a different scope, it is not a contradiction; use `none` or `refines`. |
| `refines` | A narrows, qualifies, corrects or gives a more precise version of B. | from = the more specific or corrected claim, to = the general or earlier one |
| `supersedes` | A is a newer figure for the same series or state and replaces B (an updated count or a later status). | from = newer, to = older |
| `duplicates` | The same assertion extracted twice, e.g. from two topic files. | from = the copy, to = the canonical claim (the one with more sources, or the lower topic number) |
| `none` | No meaningful relation: different subjects, different quantities, or merely related context. | — |

## Strength

- **0.9–1.0:** unmistakable, e.g. the same figure from two independent sources, or a direct numerical conflict.
- **0.6–0.8:** clear.
- **0.3–0.5:** plausible but loose.

Don't create edges below 0.3; answer `none` instead.

## Typical cases

- **Two claimants give different casualty totals for the same period and scope:** `contradicts`. The resolution names
  the methods, e.g. "Mediazona counts named dead; the General Staff claims killed and wounded".
- **A monthly series point and a later revised figure for the same month:** `supersedes`.
- **A yearly total and a monthly figure:** `none`, unless one implies the other.
- **Fibre-optic FPV range of 20 km (2025) vs 40 km (2026):** these are different times of a growing capability, so
  `none` or `supersedes` (if it is clearly the same measure's update), not `contradicts`.
- **The same RUSI figure extracted in topic 01 and topic 03:** `duplicates`.
- **An assessment ("drones cause most losses") and a figure (70% of casualties by drones):** `supports`.
