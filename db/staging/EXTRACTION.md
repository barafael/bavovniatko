# Claim extraction: brief for agents and humans

This is how research text becomes staged records for the knowledge base. It applies to the first
import of `research/0*.md` and to all later research intake. Loaders read only what passes
`db/tools/stage.py check`.

## Your inputs and outputs

- **Input.** The worksheet for your topic, produced by `db/tools/stage.py worksheet NN`. It is the
  topic file split into numbered blocks, each headed `<<< NN/OOOO | role | section >>>`. The key
  `NN/OOOO` is the `passage` you cite.
- **Output.** JSON Lines appended through the validator:
  `db/.venv/bin/python db/tools/stage.py append NN <file.jsonl>`. Write each batch (one section or
  one large block) to a scratch file and append it. Rejected lines are **not** written: fix them and
  append them again. Never edit `db/staging/claims/NN.jsonl` by hand.
- **Done** means that `db/.venv/bin/python db/tools/stage.py check NN` reports **0 errors and 0
  passages with uncovered citations**.
- **Read-only.** Don't touch `research/`, the database or any other file. No web access is needed,
  so don't fetch anything. Extraction restates what the text says; it is not new research.

## What to extract

Extract from every block **except** headings and the sections *Sources*, *Terms*, *Open questions/gaps*
and *Game/sim relevance*. Those sections are already imported deterministically as sources, terms,
questions and design notes. Preamble lines are normally skipped unless they carry cited claims.

Tables count as text. Every data row yields one or more claims, including catalogue rows (system
specs, games, datasets) and key-figure rows. Uncited prose that still states something substantive,
such as a synthesis in an overview, becomes a claim that cites the sources the same block cites for
that point. Skip pure signposting like "see 07 §2.7".

## Claim records (`"type": "claim"`)

| field | rule |
|---|---|
| `local_id` | `NN-OOOO-SS`: the passage key plus a 2–3 digit sequence within that passage (`03-0042-01`, `03-0042-02`, …). Unique and stable. |
| `passage` | `NN/OOOO` of the block the claim comes from |
| `text` | **One atomic assertion** in one or two plain English sentences, self-contained. Resolve "it"/"the unit"/"this" to names. Carry the time and place if the text gives them. Keep hedges ("about", "reportedly", "at least") and attribution ("HUR says"). Use Ukrainian spellings (Kyiv, Kharkiv, Donbas) and the file's terminology. Add nothing the text doesn't say. |
| `kind` | `figure` (a number is the point), `fact`, `event` (something happened at a time), `assessment` (analysis or judgement), `forecast` (expectation or plan), `definition` |
| `epistemic` | `observed` (visually confirmed or documented, e.g. Oryx or geolocated), `reported` (journalistic reporting of events), `claimed` (a party's own claim: officials, commanders, makers, either side), `estimated` (analyst or intelligence estimates), `derived` (the research file derived it, marked "derived"/"our estimate"), `computed` (computed from data: OSM, DeepState geometry) |
| `status` | `disputed` if the text presents it as contested; omit otherwise |
| `claimants` | Who asserts it: `{name, kind, side, role}`. Kinds are listed below. Name the claimant the text names ("Syrskyi", "HUR", "RUSI", "the 7th Air Assault Corps"). If an outlet simply reports an event, omit claimants. The outlet is the source, not the claimant. Roles: `claimant` (default), `estimator`, `analyst`, `reporter`. |
| `cites` | Exactly the `[src:…]` ids attached to *this* statement in the text, not every id in the block. `{source, locator?, quote?, support?}`. Give `quote` only if the text quotes. `support` is `direct` (default), `secondary` (the source relays someone else) or `background`. |
| `time` | `{from, to?, precision}` with dates as `YYYY`, `YYYY-MM` or `YYYY-MM-DD`, and precision `day`, `month`, `quarter` or `year`. This is the time the claim is *about*, not its publication date. |
| `eras` | The era ids the claim is about: `e1`…`e9`, or the sub-phases `e6a e6b e8a e8b e9a e9b` when clearly within one. The era section headings in the worksheet tell you. |
| `sides` | Whose forces or actions the claim concerns: `ua`, `ru`, `western`, `other` |
| `mentions` | Named entities: `{name, type, kind?, role?}`, where `type` is `place`, `event`, `system`, `actor`, `work` or `metric`. Use the most specific canonical name: "Geran-3" not "jet drone", "Pokrovsk" not "the city", "Operation Vivaldi", "Rubikon", "Delta". Kinds are listed below. Roles: `subject` (default), `location`, `instrument`, `target`, `measure`, `mention`. |
| `observations` | One per number the claim asserts. The rules are in the next section. |
| `notes` | Optional. Anything a reviewer should know, such as an ambiguity in the source text |

### Observations (numbers)

`{metric, value | low+high, value_text, qualifier, unit, time?, side?, place?, subject?, method?, note?}`

- **`metric`** is a slug from `db/vocab/metrics.yaml`. If none fits, invent a snake_case slug and add
  `"metric_new": {"label": "...", "dimension": "..."}` on that observation. Reuse your own new
  slugs consistently. Prefer a specific metric (`kab_launches`, `interception_rate`) over
  `count_generic` or `share_generic`.
- **`unit`** is an id from `db/vocab/units.yaml`: `count, count_per_day, count_per_month, count_per_year,
  people, m, km, km2, ha, km2_per_month, km2_per_day, percent, ratio, usd, usd_m, usd_bn, eur_bn, uah_bn,
  rounds_per_day, casualties_per_km2, day, h, min, s, month, year, km_per_h, km_per_km2, per_100km2,
  per_km2, points, kg, t, degree, other`, and the rest of the file.
- **`value`** is a number. For "about 8,266", use `value: 8266, qualifier: "approx"`. For a range, use
  `low`, `high` and `qualifier: "range"`. For "at least"/"over", use `qualifier: "at_least"`.
  Percentages are 0–100. `value_text` is the figure exactly as written ("~20,000 (>32,000)", "1:10").
- **Ratios.** For "Ukraine : Russia 1:10", use `metric: fire_ratio, unit: ratio, value: 0.1,
  value_text: "1:10"`. Always record which way round in `note`.
- **`subject`** says what is counted when the metric is generic, e.g. "Sting interceptors".
  `place` is a place name.
- **`method`**: `reported` (default), `estimated`, `derived`, `computed`.
- **No invented precision.** If the text says "hundreds", use `value_text` with no value, and don't
  put digits in `value_text` without a value.

## Counter links (`"type": "counter"`)

This is the measure/countermeasure graph, mainly from topic 02 with some from 01 and 03. For each
countermeasure the text says defeats or degrades a measure, write
`{type: "counter", local_id: "NN-xSS", counter, counter_kind?, measure, measure_kind?, first_observed?,
lag_days?, effect?, rationale, evidence: [claim local_ids]}`. For example: counter "fibre-optic FPV",
measure "RF jamming of drone control links", effect "immune to RF jamming", evidence
["02-0014-03"]. The evidence claims must also be staged.

## Kinds (from `db/vocab/kinds.yaml`)

- **actor:** person, outlet, think_tank, government_body, armed_force, military_unit, intelligence_agency,
  company, game_studio, ngo, academic_institution, international_org, platform, political_party, organisation
- **place:** country, oblast, raion, hromada, settlement, city_district, river, reservoir, forest, road,
  railway, bridge, front_sector, axis, landform, facility, airfield, border_crossing, region, sample_box, sea
- **event:** battle, operation, offensive, strike, truce, negotiation, policy, appointment, technical,
  aid_decision, production, incident, milestone
- **system:** recon_drone, fpv_drone, bomber_drone, loitering_munition, long_range_drone, interceptor_drone,
  ugv, naval_drone, drone_component, tube_artillery, rocket_artillery, glide_bomb, missile, air_defence,
  aircraft, helicopter, armour, vehicle, ew_system, gnss_countermeasure, passive_defence, c2_software,
  communications, sensor, mine, fortification, munition, small_arms, technique, programme, training_sim
- **work:** video_game, board_game, wargame, training_sim, map_product, dataset_product, software_product, mod

## Example

```json
{"type":"claim","local_id":"03-0010-04","passage":"03/0010","text":"The Ukrainian Air Force counted 8,266 Russian glide bombs (KABs) in June 2026.","kind":"figure","epistemic":"claimed","claimants":[{"name":"Ukrainian Air Force","kind":"armed_force","side":"ua"}],"cites":[{"source":"kyivpost-2026-korshak-glide-bombs","support":"secondary"}],"time":{"from":"2026-06","precision":"month"},"eras":["e9b"],"sides":["ru"],"mentions":[{"name":"KAB","type":"system","kind":"glide_bomb"},{"name":"Ukrainian Air Force","type":"actor","kind":"armed_force","role":"mention"}],"observations":[{"metric":"kab_launches","value":8266,"value_text":"8,266","qualifier":"exact","unit":"count_per_month","time":{"from":"2026-06","precision":"month"},"side":"ru"}]}
```
