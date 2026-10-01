# Entity curation: brief

Staged claims name things in free text (mentions, claimants, observation places, counter systems).
Curation turns those names into a canonical registry, `db/vocab/entities/<type>.yaml`, which the
loader uses to resolve every name to exactly one record.

## Input

`db/staging/entities/proposed.yaml` (from `db/tools/entities.py propose`). It lists, per type, clusters
of names that are identical after normalisation. Each cluster has its variants, count, the kinds the
extractors suggested, topics, example claim ids, and `existing` for actors already in the database
(publishers). To see a name in context: `grep -h 'NAME' db/staging/claims/*.jsonl | head -3`.

## Output: one YAML list per type

```yaml
- id: system:geran_2            # <type>:<snake_case>, stable, meaningful, no dates
  type: system                  # actor | place | event | system | work
  kind: long_range_drone        # from db/vocab/kinds.yaml for that type
  en: "Shahed-136 / Geran-2"    # canonical English label (Ukrainian spellings for places: Kyiv, Kharkiv)
  uk: "Шахед-136 / Герань-2"    # optional: Ukrainian (or original Cyrillic) form
  aliases: ["Shahed", "Shahed-136", "Geran-2", "Geran"]   # EVERY staged variant that means this entity
  parent: system:shahed_family  # optional; must be another registry id (variants → family, unit → formation, town → oblast)
  sides: [ru]                   # systems: users (ua, ru, western, other)
  side: ua                      # actors: ua | ru | western | international | other
  note: "Iranian-designed one-way attack drone, Russian licence production at Alabuga."
  # places only:
  geocode_query: "Pokrovsk, Donetsk Oblast, Ukraine"   # what to send to OSM Nominatim
  countrycodes: "ua"            # optional, default "ua,ru,by,md"
  no_geocode: true              # vague areas ("the south", "the front line", "Donbas axis" as a notion)
  bbox: [36.1, 47.6, 36.3, 47.75]   # sample boxes etc. given in the text: [west, south, east, north]
```

## Rules

1. **Coverage.** `db/.venv/bin/python db/tools/entities.py unmatched --type <type>` must end with `0 unresolved names`.
2. **Validity.** `db/.venv/bin/python db/tools/entities.py validate --type <type>` must report 0 errors. That means
   unique ids, valid kinds, existing parents, and no name mapping to two ids within a type.
3. **Merge only true identity.** Examples: "HUR" = "Defence Intelligence of Ukraine" = "Ukrainian military
   intelligence (HUR)"; "Shahed" = "Geran-2". Keep **variants** as separate entities linked by `parent`, e.g.
   Geran-3 (jet) is not Geran-2, and Lancet-3 has parent Lancet. Keep a **generic** entity for vague mentions
   ("drones", "a Ukrainian soldier", "Russian forces") so that nothing is lost. Mark it `note: "generic"`.
4. **Actors.** Reuse the `existing` id for publishers already in the database (e.g. actor:rusi,
   actor:kyiv_independent). People get `kind: person`, and `note` gives their role and date (e.g. "Commander-in-Chief
   of the Armed Forces of Ukraine from 21 Jul 2026"). Units and formations get `kind: military_unit`, with
   `parent` pointing to their higher formation where the text says so.
5. **Places.** Use Ukrainian spellings. Give a precise `geocode_query` for every real place: settlement plus oblast
   plus country, or river, forest or road names. Set `no_geocode: true` for notional areas. Set `bbox` for the
   sample boxes; their claim `notes` state the bounding box.
6. Don't edit staging files, other types' registry files, or anything else. No web access is needed.
