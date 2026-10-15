# bavovniatko

A game and simulation of the Russo-Ukrainian war, played as Ukraine, and the cited research it is built on. The
goal is to convey the war's chaos: drones, glide bombs, total surveillance, forces unable to mass, and the cycle
of measure and countermeasure.

The repository has two halves.

## [game/](game/README.md): the game

Self-running dioramas of single historical episodes, built in Bevy. The day plays out by itself along its
historical course, and the player, a drone operator and fire controller, matters at the margin. Two dioramas
exist, one minefield seen from both sides: **Vuhledar, 6 February 2023** and **Mala Tokmachka, 8 June 2023**.

```sh
cd game
cargo run --release              # Vuhledar
cargo run --release -- robotyne  # Mala Tokmachka
```

| path | contents |
|---|---|
| [`game/design/`](game/design/) | the design: the chapters and the campaign, the units, the plans in progress ([roadmap](game/design/roadmap.md)) |
| `game/src/`, `game/assets/`, `game/tools/` | the game, its baked vehicle models, the tool that bakes them |
| `game/scripts/` | headless shots, clips, the showcase film, the same-day and balance checks |
| `game/data/` | the game feed exported from the knowledge base |

## [db/](db/README.md): the knowledge base

A SurrealDB graph of about 10,800 cited claims, with their claimants, places, numbers and relations (supports,
contradicts, …). It is the source of truth for everything the game takes from history.

| path | contents |
|---|---|
| [`db/research/`](db/research/README.md) | the research base generated from the graph: topic surveys, and a dossier for each chapter |
| [`db/web/`](db/web/README.md) | a static explorer that runs the whole knowledge base in your browser: **https://barafael.github.io/bavovniatko/** |
| `db/schema/`, `db/vocab/`, `db/dump/`, `db/tools/`, … | the schema, the vocabularies, the committed data and the tooling |

## Licence

- **Code:** MIT ([LICENSE-MIT](LICENSE-MIT)) or Apache-2.0 ([LICENSE-APACHE](LICENSE-APACHE)), at your option.
- **Content and data:** CC BY 4.0, with these exceptions:
  - OpenStreetMap-derived place geometry is ODbL 1.0, © OpenStreetMap contributors;
  - quotations remain their authors';
  - the vehicle models keep their own licences ([credits](game/assets/vehicles/CREDITS.md)).

  See [LICENSE-CONTENT.md](LICENSE-CONTENT.md).
- **The explorer's bundled software** carries its own licences. SurrealDB is under the Business Source License 1.1.
  The site lists every licence under `licenses/`.

The data concerns an ongoing war, and many figures are contested. Claims record what sources say, not established
facts.
