# Chapter research: brief

Each game chapter is a **diorama**: one bounded historical episode that runs by itself if the player does nothing,
and is won by reaching **the outcome that actually happened** (see `design/chapters.md`). A chapter dossier
collects what the designers need to script that: the true outcome, a dated timeline, the geography, the forces,
the set pieces and the real decision points. Every statement must be cited.

The dossiers enter the knowledge base through the usual intake pipeline: passages, claims, entities, relations
(`db/README.md`). So write them in the same cited-markdown style as `research/0*.md`.

## You write exactly two files

- `research/chapters/<slug>.md`: the dossier.
- `db/staging/sources/<slug>.yaml`: the **new** sources you cite, in the format of `research/sources.yaml` (below).
  Sources already in the knowledge base are cited by their existing id and **not** repeated here.

Touch nothing else. Scratch work goes in
`/tmp/claude-1000/-home-rafael-bavovniatko/757aa319-9f33-441d-a07e-ce09ebf2f33f/scratchpad/chapters/<slug>/`.

## Dossier structure

Cite inline as `[src:<id>]`. Use these sections, numbered exactly like this:

```
# <Chapter title>
<one-line scope: dates, place>

## 1. Summary and historical outcome
   What happened, in 5–10 sentences. Then **the outcome the player must reproduce**, stated concretely
   (dates, places held or lost, losses, ships sunk, km²). Where the outcome is contested, give the range and name
   the claimants.
## 2. Timeline
   A dated table: | date (time if known) | event | side | source |. This is the self-running script, so be dense.
   Aim for 25–60 rows.
## 3. The diorama: geography, terrain, weather
   The bounded map. Give key locations with coordinates (lat, lon) where a source or OSM gives them, plus
   distances, terrain, structures, water, roads and bridges. Note the weather and light conditions on key days.
## 4. Forces
   An order of battle per side: units, commanders, strength, equipment, and how they changed over the window.
## 5. Systems and tactics in use
   Which weapons and techniques mattered, how they were employed, and what countered them.
## 6. Set pieces
   The iconic moments a diorama should stage, each a short paragraph.
## 7. Decision points
   The moments where choices mattered: what was decided, by whom, what the alternatives were and what followed,
   according to sources. **No invented counterfactuals.** Only what sources say or debate.
## 8. Media
   Notable public media: photos, footage, maps, satellite imagery. Give a table of | what | kind | publisher | date |
   url | licence or terms if known |. Links only; download nothing.
## 9. Russia's side as reported
## 10. Game/sim relevance
   3–8 bullets on mechanics, win band and pacing.
## 11. Open questions/gaps
## 12. Sources
   A list of ids, each with a one-line note.
```

## Sourcing rules (as in pass 1 and 2)

- **Language and perspective.** English-language sources, with a Ukrainian accent: Ukrainian outlets in English
  (Kyiv Independent, Kyiv Post, Ukrainska Pravda EN, Militarnyi, Defense Express, Euromaidan Press, United24,
  Ukrinform, Hromadske, official UA statements), paired with Western analysis (RUSI, ISW/CTP, CSIS, OSW,
  War on the Rocks, the Naval News and USNI type for naval topics) and OSINT (Oryx, GeoConfirmed, satellite
  imagery reporting).
- **No Russian state media.** Russian claims are reported as Ukrainian or Western sources report them.
- **Ukrainian spellings** (Kyiv, Kharkiv, Mykolaiv, Zmiinyi Island / Snake Island).
- **Provenance.** NEVER invent URLs, figures, names or coordinates. Fetch key sources to confirm them. Use
  `verified: search-only` only when a page blocks fetching but search results clearly confirm it.
- **Contested figures.** Give ranges and name the claimant.
- **Existing sources first.** Before adding a source, check whether the knowledge base already has it:
  `cd /home/rafael/bavovniatko && db/.venv/bin/python db/tools/q.py "SELECT id, title FROM source WHERE string::contains(url, '<domain/path fragment>')"`.
  You can also read what the knowledge base already knows about your chapter:
  `db/.venv/bin/python db/tools/q.py "SELECT key, text, ->cites->source.id AS src FROM claim WHERE string::contains(string::lowercase(text), '<term>')"`.
- **Privacy.** Never put personal data (names, emails) in User-Agent strings or requests. Use a generic browser UA.
- **Blocked pages.** If WebFetch returns 403, try `curl -sL --compressed -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/126 Safari/537.36"`,
  or the Wayback Machine at `https://web.archive.org/web/2026id_/<url>`.

## Source entry format (`db/staging/sources/<slug>.yaml`)

```yaml
- id: kyivindependent-2022-smith-moskva-sinking     # publisher-year-author-slug, lowercase, unique
  title: "…"
  url: https://…
  publisher: Kyiv Independent
  authors: [Jane Smith]
  published: "2022-04-15"          # YYYY-MM-DD | YYYY-MM | YYYY | unknown
  accessed: "2026-10-01"
  type: news                        # think-tank | news | osint-dataset | map | official | academic | video | podcast | game | dataset | book | encyclopedia | review
  origin: ukrainian                 # ukrainian | western | international | other
  reliability: medium               # high | medium | low (reason in notes)
  verified: fetched                 # fetched | search-only
  eras: [e1-invasion]               # era codes from research/README.md
  topics: [naval, chapter]
  found_in: [<slug>]
  notes: "One-line summary and why it matters."
```

Validate your YAML parses: `python3 -c "import yaml,sys; yaml.safe_load(open(sys.argv[1]))" <file>`. Every
`[src:id]` in your dossier must be either in your YAML or already in the knowledge base. Check with:

```
cd /home/rafael/bavovniatko && db/.venv/bin/python - <slug> <<'EOF'
import sys, re, yaml, json, subprocess
s = sys.argv[1]
mine = {e['id'] for e in yaml.safe_load(open(f'db/staging/sources/{s}.yaml'))}
cited = set(c.strip() for m in re.findall(r'\[src:([^\]]+)\]', open(f'research/chapters/{s}.md').read()) for c in re.split(r'[,;]\s*(?:src:)?', m) if c.strip())
q = "SELECT VALUE legacy_ids FROM source"
kb = {i for l in json.loads(subprocess.run(['db/.venv/bin/python','db/tools/q.py',q],capture_output=True,text=True).stdout) for i in l}
print('unresolved:', sorted(cited - mine - kb)); print('uncited new:', sorted(mine - cited)); print('dup of kb:', sorted(mine & kb))
EOF
```
It must print empty lists.

## Final reply (under 150 words)

The outcome you established, the timeline length, the source count (new and reused), the biggest uncertainties,
and anything another chapter should know.
