# bavovniatko: research base

This is cited source material for a Bevy wargame and simulation of the Russo-Ukrainian war. The player
is always the Ukrainian side. It is phase 1: collection, a broad survey. It is not game design yet.

- **Collected:** 2026-09-28 by nine parallel research agents, one per topic, then merged and checked.
- **Language:** English-language sources only, with a Ukrainian accent. Ukrainian outlets that publish
  in English are preferred and paired with Western analysis and OSINT. Places use Ukrainian spellings.
  Russian capabilities are described only as Ukrainian or Western sources report them. There is no
  Russian state media.
- **Links only.** No media or datasets were downloaded.

## Files

| # | file | what's inside |
|---|---|---|
| 00 | [00-timeline-eras.md](00-timeline-eras.md) | Phases and battles per era, each side's resources and aid, territory, losses, peace talks |
| 01 | [01-drones-uncrewed.md](01-drones-uncrewed.md) | Drone type catalogue (recon, FPV, fibre, bombers, Lancet, Shahed/jet Geran, interceptors, UGVs), production, kill-zone depth, e-points |
| 02 | [02-ew-countermeasures.md](02-ew-countermeasures.md) | Chronological measure → countermeasure chain, jamming, GNSS spoofing, nets, cages, the Starlink whitelist, the speed of adaptation |
| 03 | [03-fires-air.md](03-fires-air.md) | Artillery and shell ratios, glide bombs, helicopter lob fire, tactical aviation, air defence, missiles, sensor-to-shooter times |
| 04 | [04-ground-tactics-manpower.md](04-ground-tactics-manpower.md) | Assault and defence tactics per era, North Korean troops, fortifications and mines, rotations, mobilisation, AWOL, casevac, casualties |
| 05 | [05-isr-c2-logistics.md](05-isr-c2-logistics.md) | Battlefield transparency, kill-zone depth by era, Delta/Kropyva/Starlink, logistics under drones, UGVs, deep strike, deception |
| 06 | [06-terrain-geodata.md](06-terrain-geodata.md) | Shelterbelt (lisosmuha) geometry, fields, landforms, settlements, seasons, battle damage, an open-geodata catalogue with licences, procgen literature |
| 07 | [07-maps-ui-conventions.md](07-maps-ui-conventions.md) | DeepStateMap in depth, other front maps compared, APP-6/2525 symbology, glimpses of Ukrainian military UIs, data access |
| 08 | [08-prior-art-games.md](08-prior-art-games.md) | Commercial, drone and training-sim, board and professional wargames: what they model and miss, plus ethics |
| — | [glossary.md](glossary.md) | Frontline slang (transliterated Ukrainian), Russian terms as reported, systems, programmes, terrain and wargaming terms |
| — | [sources.yaml](sources.yaml) | The merged, deduplicated index of every source. Regenerate it with `python3 research/tools/merge_sources.py`, which also checks every `[src:id]` citation |
| — | `sources/NN-*.yaml` | Each topic's own source list, as its agent wrote it |

Every topic file has the same skeleton: overview → per-era notes → catalogue/figures tables →
"Russia's side as reported" (00–05) → **Game/sim relevance** → Terms → Open questions/gaps → Sources.
Claims are cited inline as `[src:<id>]`, and each id resolves in `sources.yaml` (directly or through `aliases`).

## Eras

These IDs are canonical across all files and the future game data. The boundaries are working boundaries;
file 00 §8 proposes splits.

| id | span | what defines it |
|---|---|---|
| `e1-invasion` | Feb–Apr 2022 | Kyiv, Chernihiv, Sumy, Kharkiv, Mariupol; Russian BTG columns; Javelin/NLAW/TB2 |
| `e2-donbas-artillery` | May–Aug 2022 | Sievierodonetsk/Lysychansk; Russian artillery superiority (about 10:1); HIMARS arrives |
| `e3-counteroffensives-22` | Sep–Nov 2022 | Kharkiv and Kherson counteroffensives; Russian mobilisation; the Shahed campaign begins |
| `e4-bakhmut` | Dec 2022–May 2023 | Wagner and convict "meat assaults"; Vuhledar armour losses; Lancet |
| `e5-counteroffensive-23` | Jun–Nov 2023 | Zaporizhzhia push into minefields and the Surovikin line; Western armour; Kakhovka dam; Krynky |
| `e6-avdiivka-attrition` | Oct 2023–Jul 2024 | Avdiivka; shell famine while US aid stalled; mass KAB glide bombs; FPV boom; Vovchansk |
| `e7-kursk-pokrovsk` | Aug 2024–Mar 2025 | Kursk incursion; North Korean troops; Pokrovsk/Kurakhove; fibre-optic FPV appears |
| `e8-drone-kill-zone` | Mar 2025–Jan 2026 | Kill zone deepens to 10–15+ km; infiltration by 2–4-man groups; Rubikon; Spiderweb; interceptors against Shahed |
| `e9-counteroffensive-26` | Feb 2026–present | Starlink whitelist cuts off Russia (early Feb); Ukrainian counteroffensive from 11 Feb; Operation Vivaldi near Lyman; jet Gerans; UGVs at scale |

## Source schema (`sources.yaml`)

```yaml
- id: rusi-2025-watling-third-year        # publisher-year-author-slug
  aliases: [rusi-2025-watling-reynolds-third-year]   # other ids that agents used for the same source
  title: "..."
  url: https://...
  alt_urls: [https://...]                 # e.g. the PDF next to the landing page
  publisher: RUSI
  authors: [Jack Watling, Nick Reynolds]
  published: 2025-02                      # YYYY-MM-DD | YYYY-MM | YYYY | unknown
  accessed: 2026-09-28
  type: think-tank                        # think-tank | news | osint-dataset | map | official | academic | video
                                          # | podcast | game | dataset | book | encyclopedia | review | standard | software
  origin: western                         # ukrainian | western | international | other
  reliability: high                       # high | medium | low; the reason is in notes
  verified: fetched                       # fetched (an agent read the page) | search-only (confirmed by search results only)
  eras: [e6-avdiivka-attrition, e7-kursk-pokrovsk]
  topics: [drones, ew]
  found_in: [01-drones-uncrewed, 02-ew-countermeasures]
  notes: "One-line summary and why it matters."
```

**Reliability scale:**
- **high:** primary data, official statistics from the named party, peer-reviewed work, or established think tanks with fieldwork (RUSI, CSIS, Kiel).
- **medium:** reputable journalism, including most Ukrainian outlets, which relay official claims.
- **low:** single-source, promotional or unverified.

Official claims by either side are tagged as claims in the text, whatever the outlet's rating.

## Coverage

**304 unique sources** (342 entries across the topic files before deduplication). Every URL was re-checked on 2026-09-28: 295 return 200, 8 return 403 to scripts but are known publisher pages (Wiley DOIs, Forbes, NYT, BoardGameGeek, Critical Threats), and 1 is unreachable (`ffm-oleshky-shelterbelts`, already marked search-only).

By origin: 142 Ukrainian, 79 Western, 79 international, 4 other. By reliability: 117 high, 182 medium, 5 low.

| topic | sources | fetched | search-only | e1 | e2 | e3 | e4 | e5 | e6 | e7 | e8 | e9 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 00-timeline-eras | 36 | 36 | 0 | 7 | 6 | 9 | 6 | 5 | 10 | 10 | 19 | 21 |
| 01-drones-uncrewed | 32 | 32 | 0 | 2 | 2 | 2 | 3 | 1 | 3 | 7 | 19 | 13 |
| 02-ew-countermeasures | 39 | 38 | 1 | 4 | 3 | 3 | 3 | 5 | 8 | 7 | 15 | 15 |
| 03-fires-air | 33 | 33 | 0 | 5 | 4 | 4 | 4 | 7 | 10 | 11 | 16 | 12 |
| 04-ground-tactics-manpower | 41 | 40 | 1 | 3 | 3 | 4 | 5 | 4 | 6 | 12 | 15 | 21 |
| 05-isr-c2-logistics | 44 | 44 | 0 | 3 | 3 | 5 | 2 | 3 | 3 | 6 | 12 | 25 |
| 06-terrain-geodata | 45 | 44 | 1 | 3 | 3 | 3 | 3 | 7 | 6 | 4 | 6 | 5 |
| 07-maps-ui-conventions | 29 | 27 | 2 | 11 | 7 | 10 | 7 | 6 | 8 | 11 | 17 | 15 |
| 08-prior-art-games | 43 | 41 | 2 | 10 | 2 | 2 | 2 | 1 | 3 | 1 | 10 | 4 |
| **merged** | **304** | | | **39** | **26** | **36** | **27** | **31** | **51** | **60** | **121** | **115** |

The era columns count the sources tagged with that era. Coverage is heaviest for e8–e9 (2025–26), where the war changed fastest and training data is thinnest. It is lightest for e2 and e4.

## Cross-topic notes and conflicts

- **Kill-zone depth** is the most important single parameter, and it grows by era:
  - e1–e2: near zero.
  - Late 2024: dense to about 3 km, thinning to about 15 km, taskable to 40 km (RUSI fieldwork).
  - 2025: 10–15 km lethal (the Drone Line target).
  - Mid-2026: 15–30 km, depending on sector and commander (04, 05).

  No dataset exists for it; the figures are commanders' estimates.
- **Starlink cutoff date:** 1 Feb (ISW via EMP), 2–5 Feb (Kyiv Independent, OSW, SpaceX), and United24's "Dec 2025", which looks wrong (02, 05, 00).
  The gains it enabled are reported as 10–12 km of depth or 200+ km² in about 5 days, which are different units.
  Whether the cutoff *caused* the 11 Feb counteroffensive or merely helped it is disputed (00 §8).
- **Jet Gerans:**
  - About 180 launched in 2025, about 1,400 cumulative by early June 2026, and 15–20% of launches by April 2026 (00).
  - Monthly launches went from about 350 (June) to about 2,800 (August 2026) (01).
  - Interception is about 60%, against 90%+ for piston Shaheds (01, 02).

  These figures are consistent and describe a steep ramp. HUR's production claim of about 3,000 Geran-4/5 a month is unverified.
- **Drone lethality:** RUSI says drones cause 60–70% of damaged Russian systems, and Ukrainian officials say 70–80% of casualties. These measure different things. RUSI also says 60–80% of FPVs fail, while OSW's accuracy figures (30% → 70%) use a different definition (01, 03).
- **Shell ratio** (Ukraine : Russia): about 1:10 in e2, 1:10–1:12 during the early-2024 famine, and about 1:1.8 by July 2025 (00, 03). Ukrainian official figures for Russian daily fire (27–44k) are far above Western open-source estimates (10–15k).
- **Glide bombs:** 5,171 in January 2026 rising to 8,266 in June 2026 (Air Force data via Kyiv Post). Figures for 2025 production conflict: 70k (RUSI) versus 120k (HUR) (03).
- **Casualties** are contested everywhere. Mediazona/BBC count about 259k Russian dead by name, a hard lower bound. The Ukrainian General Staff claims about 1.5m Russians killed or wounded, and CSIS and UK estimates fall in between. Ukrainian losses are opaque (00, 04).
- **The grey zone changed meaning.** On DeepStateMap it used to be a thin no-man's-land. In 2026 it was redefined as "areas of Russian infiltration" with intermixed positions, and ISW added an infiltration layer in December 2025. The contested depth grew from about 1 km (Jan 2025) to 5–12 km (07). This mirrors the tactical shift to small-group infiltration (04).
- **Map data is not free to ship.** DeepState has no open licence (its web app's JSON endpoints are undocumented), ISW geodata requires written consent, and ACLED needs a paid licence for commercial use. milsymbol (MIT) is safe. File 07 recommends self-authored era polygons, with permission requested before bundling anyone's geometry.
- **Licensing matters for the procedural generator.** OSM and Overture derivatives are share-alike (ODbL). The only Ukraine field-boundary dataset (NASA Harvest) is non-commercial and no-derivatives. CC BY layers (WorldCover, Dynamic World, Hansen) are safer to bake in (06).

## Gaps worth a second, deeper pass

The web-search budget ran out partway for most agents, so these are the known holes:

1. **ISW/CTP daily assessments.** Most returned 403, and 2026 sector detail relies on Ukrainian outlets and Wikipedia. Worth adding manually, especially for Feb 2026 and Operation Vivaldi.
2. **Weapon arrival dates and quantities** from official US and European fact sheets (these returned 403). Needed for exact era loadouts.
3. **Territory time series.** DeepState monthly km² statistics for 2022–2026 would give one consistent frontline-change metric.
4. **Russian-side quantities before late 2024:** Shahed volumes 2022–24, Russian FPV production, Lancet after March 2023, and Krasukha/Tobol/trench-jammer specs.
5. **Fires in 2026:** the role of artillery and glide bombs in the counteroffensive and Vivaldi, Ukrainian helicopter lob fire, and EU shell output.
6. **Manpower:** Ukrainian mobilisation-law changes in 2025–26, medical ratios, remote mining and minefield density.
7. **Terrain calibration:** real field-block sizes and belt spacing measured from OSM for sample areas (Pokrovsk, Robotyne, Kupiansk), balkas and river valleys, and village morphology. The typical block size of 0.5 × 1–2 km is a *derived estimate*.
8. **Maps not yet covered:** Suriyak, Andrew Perpetua and AMK Mapping; Liveuamap icon semantics; Delta UI screenshots; DeepState monthly km² figures for 2022–25.
9. **Games not yet verified:** Regiments, Flashpoint Campaigns, Steel Division, Men of War/Gates of Hell, Gray Zone Warfare, CSIS and Dstl wargames.
10. **Possible era splits:** e6 at Apr/May 2024 (after the US supplemental), e8 at about Aug 2025, and e9 at about Jun 2026 (the jet-Geran surge and Vivaldi).
11. **The 2026 US–Iran war** and its effect on Patriot and ATACMS supply to Ukraine was mentioned in passing but needs sourcing.

Each topic file's "Open questions/gaps" section has the full per-topic list.
