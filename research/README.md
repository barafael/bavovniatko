# bavovniatko: research base

This is cited source material for a Bevy wargame and simulation of the Russo-Ukrainian war. The player
is always the Ukrainian side. It is the research phase: collected sources, not game design yet.

- **Collected** in two passes by nine parallel research agents, one per topic, then merged and checked:
  a broad survey (2026-09-28), then a deeper pass on the gaps it left (2026-09-28 to 09-30). The deeper
  pass folded its findings into the existing files rather than appending to them.
- **Language:** English-language sources only, with a Ukrainian accent. Ukrainian outlets that publish
  in English are preferred and paired with Western analysis and OSINT. Places use Ukrainian spellings.
  Russian capabilities are described only as Ukrainian or Western sources report them. There is no
  Russian state media.
- **Generated from the knowledge base.** Since 2026-09-30 the SurrealDB claim graph in [`db/`](../db/README.md)
  is the source of truth: 7,100 atomic claims with their sources, claimants, eras, places and observations, and
  typed relations between them. The topic files, the glossary, the source indexes and
  [contradictions.md](contradictions.md) are regenerated from it by `db/tools/render.py`. Edit the data, not
  the files. This README is still written by hand.
- **Links only.** Nothing is committed except text. Some figures were *computed* from public data
  (OSM Overpass for 06, DeepState's map history for 07), and each such figure is labelled with its
  method and query date.

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
| — | [chapters/](chapters/) | **Chapter dossiers** for the game's dioramas (see [design/chapters.md](../design/chapters.md)): the historical outcome to reproduce, a dated timeline, the geography, the forces, set pieces, decision points and media |
| — | [contradictions.md](contradictions.md) | Every pair of claims that cannot both be true as stated, with claimants, sources and the explanation (generated) |
| — | [sources.yaml](sources.yaml) | The deduplicated index of every source (generated from the knowledge base) |
| — | `sources/NN-*.yaml`, `sources/chapters/*.yaml` | The sources cited by each topic file and chapter dossier (generated) |

Every topic file has the same skeleton: overview → per-era notes → catalogue/figures tables →
"Russia's side as reported" (00–05) → **Game/sim relevance** → Terms → Open questions/gaps → Sources.
Claims are cited inline as `[src:<id>]`, and each id resolves in `sources.yaml` (directly or through `aliases`).

## Chapter dossiers

| dossier | chapter | window |
|---|---|---|
| [c02](chapters/c02-snake-island-moskva.md) | Snake Island → the sinking of the Moskva | 24 Feb – 7 Jul 2022 |
| [c03](chapters/c03-azovstal.md) | Mariupol and the Azovstal defence, with the helicopter air bridge | Feb – 20 May 2022 |
| [c04](chapters/c04-kherson-antonivskyi.md) | Kherson and the HIMARS campaign against the Dnipro crossings | Jul – 11 Nov 2022 |
| [c05](chapters/c05-kerch-bridge.md) | The Kerch bridge: truck bomb, sea drones, underwater charges | Oct 2022 – 2026 |
| [c06](chapters/c06-kharkiv-offensive.md) | The Kharkiv counteroffensive | 29 Aug – early Oct 2022 |
| [c08](chapters/c08-vuhledar.md) | Vuhledar: Russian mechanised assaults into minefields | Nov 2022 – Mar 2023 |
| [c10](chapters/c10-krynky.md) | The Krynky bridgehead | Oct 2023 – Jul 2024 |
| [c12](chapters/c12-a50-hunt.md) | Hunting the A-50 | Jan – Feb 2024 |
| [c13](chapters/c13-black-sea-drone-war.md) | The Black Sea sea-drone war | Oct 2022 – May 2025 |
| [c17](chapters/c17-kupiansk-pipeline.md) | The Kupiansk gas-pipeline infiltration | 2025 – early 2026 |
| [v01](chapters/v01-brovary-ambush.md) | Vignette: the Brovary ambush | 9–10 Mar 2022 |
| [v02](chapters/v02-stepove-bradley.md) | Vignette: Bradley vs T-90M near Stepove | 11–12 Jan 2024 |

Each dossier's claims are in the knowledge base. The outcome claims carry an `OUTCOME` flag, which is the
chapter's win condition: `SELECT key, text FROM claim WHERE topics CONTAINS topic:c03 AND ext.outcome = true`.

## Eras

These IDs are canonical across all files and the future game data. File 00 §3 describes sub-phases
inside e6, e8 and e9 (e.g. *e6a/e6b*) in prose. The IDs themselves stay fixed.

| id | span | what defines it |
|---|---|---|
| `e1-invasion` | Feb–Apr 2022 | Kyiv, Chernihiv, Sumy, Kharkiv, Mariupol; Russian BTG columns; Javelin/NLAW/TB2 |
| `e2-donbas-artillery` | May–Aug 2022 | Sievierodonetsk/Lysychansk; Russian artillery superiority (about 10:1); HIMARS arrives |
| `e3-counteroffensives-22` | Sep–Nov 2022 | Kharkiv and Kherson counteroffensives; Russian mobilisation; the Shahed campaign begins |
| `e4-bakhmut` | Dec 2022–May 2023 | Wagner and convict "meat assaults"; Vuhledar armour losses; Lancet |
| `e5-counteroffensive-23` | Jun–Nov 2023 | Zaporizhzhia push into minefields and the Surovikin line; Western armour; Kakhovka dam; Krynky |
| `e6-avdiivka-attrition` | Oct 2023–Jul 2024 | Avdiivka; shell famine while US aid stalled; mass KAB glide bombs; FPV boom; Vovchansk. Splits at the Apr 2024 US supplemental (e6a aid drought / e6b post-supplemental) |
| `e7-kursk-pokrovsk` | Aug 2024–Mar 2025 | Kursk incursion; North Korean troops; Pokrovsk/Kurakhove; fibre-optic FPV appears |
| `e8-drone-kill-zone` | Mar 2025–Jan 2026 | Kill zone deepens to 10–15+ km; infiltration by 2–4-man groups; Rubikon; Spiderweb; interceptors against Shahed. Splits at about Aug 2025 (e8a peak Russian summer gains / e8b Pokrovsk–Myrnohrad, fall of Huliaipole) |
| `e9-counteroffensive-26` | late Jan 2026–present | Starlink whitelist cuts off Russia (29 Jan–5 Feb); Ukrainian southern counteroffensive (from 29 Jan); Russian net gains near zero; Operation Vivaldi near Lyman. Splits at about Jun/Jul 2026 (e9b: jet-Geran surge, Patriot shortage from the Iran war, new defence minister and commander-in-chief) |

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

**583 unique sources** (677 entries across the topic files before deduplication); 579 were fetched and read, and 4 are confirmed by search results only. 43 are encyclopedia entries, used mainly to find the primary sources.

On 2026-09-30 every URL was re-checked. 568 return 200. 14 refuse scripted access (403/406/429) but are real publisher pages, DOIs or API endpoints. 1 is unreachable (`ffm-oleshky-shelterbelts`, which is already marked search-only).

By origin: 344 Ukrainian, 131 Western, 93 international, 15 other. By reliability: 170 high, 397 medium, 16 low.

| topic | sources | e1 | e2 | e3 | e4 | e5 | e6 | e7 | e8 | e9 |
|---|---|---|---|---|---|---|---|---|---|---|
| 00-timeline-eras | 82 | 11 | 11 | 11 | 10 | 11 | 16 | 17 | 29 | 55 |
| 01-drones-uncrewed | 76 | 4 | 2 | 5 | 6 | 5 | 8 | 11 | 40 | 41 |
| 02-ew-countermeasures | 80 | 8 | 4 | 4 | 5 | 7 | 13 | 14 | 24 | 45 |
| 03-fires-air | 77 | 7 | 4 | 4 | 4 | 12 | 14 | 16 | 35 | 44 |
| 04-ground-tactics-manpower | 73 | 4 | 3 | 4 | 5 | 8 | 10 | 17 | 29 | 44 |
| 05-isr-c2-logistics | 92 | 3 | 3 | 6 | 2 | 3 | 3 | 6 | 20 | 71 |
| 06-terrain-geodata | 65 | 6 | 6 | 6 | 5 | 14 | 10 | 7 | 10 | 9 |
| 07-maps-ui-conventions | 55 | 13 | 9 | 12 | 9 | 11 | 15 | 22 | 30 | 24 |
| 08-prior-art-games | 77 | 13 | 2 | 2 | 2 | 1 | 3 | 1 | 16 | 12 |
| **merged** | **583** | **59** | **36** | **47** | **39** | **59** | **82** | **98** | **205** | **282** |

The era columns count sources tagged with that era. Coverage is heaviest for e8–e9 (2025–26), where the war changed fastest and training data is thinnest. It is lightest for e2 and e4.

## Cross-topic notes and conflicts

These are the figures the simulation depends on most, reconciled across the topic files. The topic files
have the claimants and full ranges.

- **Kill-zone depth** is the most important single parameter, and it grows by era:
  - e1–e2: near zero.
  - Late 2024: dense to about 3 km, thinning to about 15 km, taskable to 40 km (RUSI fieldwork).
  - 2025: 10–15 km lethal (the Drone Line target).
  - July 2026: 20–25 km on active axes, with 30 km expected by year-end (7th Air Assault Corps). The range across commanders is 15–30 km (01, 04, 05).

  These are still commanders' estimates. Delta/Mission Control holds strike-by-depth data, but it isn't public.
- **The Starlink cutoff was phased, not dated to one day** (05, 00, 02):
  - 29 Jan 2026: the MoD contacts SpaceX.
  - Late Jan–1 Feb: a speed cap of about 75–90 km/h. This is ISW's "1 Feb".
  - 2 Feb: the Cabinet creates the whitelist.
  - 5 Feb: enforcement.

  United24's "Dec 2025" is an error. According to Biletsky via ISW, Russian drone effectiveness fell 20–40% within two weeks. By mid-March Russian drones were again striking 40–50 km deep over mesh relays, and by summer C2 was "gradually restored". The Russian "Kupol" jams Ukraine's Starlink rather than replacing Russia's.
- **The counteroffensive started 29 Jan 2026, not 11 Feb.** That is the Air Assault Forces' own date; ISW first observed counterattacks on 9 Feb, and the 11 Feb figure traces back only to Wikipedia. Starlink helped but did not cause it (00 §3 e9). ISW measures near-zero net Russian gains from March 2026 on.
- **The territory series is now consistent** (07 §2.7). It is computed from DeepState's per-version map geometry and matches DeepState's published monthly figures in most months:
  - Net Russian gains of about 380–390 km² a month in e7–e8, and about 78 a month in e9.
  - 2025 total: 4,336 km².
  - July 2026: 36 km² (Radio Svoboda's 27 is off).
  - The grey zone tripled to about 1,600–1,800 km², and DeepState redefined it in 2026 as "areas of Russian infiltration".
  - DeepState publishes Ukrainian gains late for operational security, so Vivaldi's May–Jun recaptures appear in Aug–Sep.
  - Vivaldi's area is disputed: the corps claims 125+ km², mappers estimate 200–240 km².
- **Russian cost per km² rose steeply** (00, 04): about 53 casualties per km² gained in autumn 2024, 85 in 2025, and 290 in 2026.
- **Jet Gerans:**
  - About 180 launched in 2025. By June 2026, about 1,400 had been launched in total.
  - Launches went from about 350 a month (June 2026) to about 2,800 (August).
  - Interception is 55% (September, Euromaidan Press) to 57% (Zelensky, 28 Sep), against 70% overall and 90%+ for piston Shaheds.
  - HUR claims about 3,000 Geran-4/5 are produced a month.
  - Mobile fire groups can't catch jet Gerans (Ihnat).

  The Air Force stopped publishing monthly Shahed counts in May 2026, so later series are reconstructions (01, 02, 03).
- **Drone lethality:** RUSI says drones cause 60–70% of damaged Russian systems, and Ukrainian officials say 70–80% of casualties; these measure different things. FPV success is about 10–15% without guidance and a claimed 70–80% with terminal guidance (CSIS). RUSI's 2024 figure is a 60–80% failure rate. The Unmanned Systems Forces claim 360k targets hit in 1.7M sorties (01, 03).
- **Shell ratio and daily fire:**
  - The Ukraine : Russia ratio was about 1:10 in e2, 1:10–1:12 during the early-2024 famine, and about 1:1.8 by July 2025.
  - For Russian daily fire, the Western open-source estimate is 10–15k rounds (Modern War Institute) and the Ukrainian official figure is about 27k (2025). Ukrainian daily fire is about 2–7k rounds (03).
- **Glide bombs** follow a monthly Air Force/MoD series (03):
  - About 3,200 (May 2024).
  - 5,171–5,700 (Jan 2026).
  - 8,266 (Jun 2026).
  - 8,766 (Aug 2026).

  Avdiivka took about 60 a day in February 2024. For 2025 production, RUSI says 70k UMPK were ordered and HUR says 120k were planned.
- **Air defence under the Iran war:**
  - The 2026 US–Iran war used up to 1,430 Patriot rounds in 39 days, draining Ukraine's supply.
  - Ballistic intercept rates range from 10.2% (CSIS) to about 40% (June) and 15% (July, 29 of 195) (MoD).
  - Ukraine's ATACMS stock is "severely depleted" (00, 03).
- **Command change:** Defence minister Fedorov was dismissed in mid-July 2026 (Khmara acting from 16 Jul), and Drapatyi replaced Syrskyi as Commander-in-Chief on 21 Jul. The game should know who is in charge when, because most official figures in these files come from these officials (00, 04, 05).
- **Manpower** (04):
  - HUR says Russia recruited about 403k in 2025. Ukrainian intelligence gives about 232k recruited against about 240k lost in Jan–Jul 2026.
  - Ukraine mobilises about 30k a month.
  - Fixed-term contracts opened 15 Jun 2026 (about 2,000 signed in month one).
- **Casualties** are contested everywhere (00, 04):
  - Russian dead: Mediazona/BBC count about 259k by name, a hard lower bound. UK intelligence estimates about 500k killed (May 2026).
  - Russian killed and wounded: the Ukrainian General Staff claims about 1.53M.
  - Ukrainian dead range from 55k (Zelensky, Feb 2026) to 100–140k (CSIS).
- **Terrain is now measured, not estimated** (06 §2.7). OSM and Sentinel-2 were measured across five boxes of about 10 × 10 km.
  - **Huliaipole E**, the cleanest grid: main belts are 549 m apart (OSM) or 547 m (Sentinel-2), with cross-belts 1.1–1.3 km apart. That gives field blocks of about **0.55 × 1.2 km**.
  - **The other boxes** are coarser and more irregular, with main belts about 0.9–1.2 km apart.
  - **Crop fields:** median 38–65 ha.
  - **Belt damage:** by 2023, 21.5% of front-line shelterbelts were damaged, with up to 57% loss of function in hotspots (Matsala 2024).
- **Data you can't simply ship** (06, 07):
  - DeepState has no open licence (its web app's JSON endpoints are undocumented; ask before any bulk use).
  - ISW geodata requires written consent, and ACLED needs a paid commercial licence.
  - GeoConfirmed's data is free for research, journalism and analysis, but it has no commercial clause.
  - Liveuamap requires a reference.
  - OSM and Overture derivatives are share-alike (ODbL). NASA Harvest's field boundaries are non-commercial and no-derivatives.
  - Safe to ship: CC BY layers (WorldCover, Dynamic World, Hansen) and milsymbol (MIT). File 07 recommends self-drawn control polygons per era.

## Remaining gaps

Both passes ran out of web-search budget. Pass 2 worked around this with the Wayback Machine for ISW,
site search APIs and public data endpoints. These gaps remain; each topic's "Open questions/gaps"
section says what was tried.

1. **No independent Ukrainian detection-to-strike latency for 2025–26**, and no measured kill-zone density (05). Both matter for the core simulation loop.
2. **Loss and failure rates:** FPV loss rates for 2026 (02); UGV loss and casevac success rates (04, 05); kill shares of Shahed kills by weapon type over time (02, 03).
3. **Fires in 2026:** artillery, HIMARS and glide bombs in the counteroffensive and Vivaldi are unquantified because of "maximum information silence". The glide-bomb monthly series still misses most of 2024–25. Artillery's own share of casualties is missing (03).
4. **Russian production before 2025:** FPV output for 2023–24; Lancet strike counts after March 2023 (WarSpotting blocks scripts) (01).
5. **Russian EW specs:** Krasukha-2 in Ukraine, Pole-21, trench-jammer families, and Russian terminals after the cutoff (02).
6. **Minefield density per km** for 2023 vs 2024–26; the Ukrainian contract reform's legal fate; Ukrainian casualties (04).
7. **Terrain:** balka dimensions (measurable from Copernicus GLO-30), Soviet belt design norms, crater density after 2022, and a village-damage time series. The five OSM boxes are a small sample (06).
8. **Maps:** the NYT map (blocked everywhere), Delta's visual design, DeepState's terms of use, and the legends of UA Control Map and Liveuamap (07).
9. **Weapon dates:** NLAW's first delivery, the first combat use of Abrams and HIMARS, and Patriot's first intercept; the March 2022 peak occupied share (00).
10. **Games:** CSIS and Dstl drone wargames; first-party confirmation of the commercial FPV sims used in Ukrainian drone schools; some developer countries (08).
