# bavovniatko

Research and tooling for a game and simulation of the Russo-Ukrainian war, played as Ukraine. The goal is to
convey the war's chaos: drones, glide bombs, total surveillance, forces unable to mass, and the cycle of measure and
countermeasure.

- **[research/](research/README.md):** a cited research base, with topic surveys and chapter dossiers for the game's
  dioramas. It is generated from the knowledge base.
- **[db/](db/README.md):** the knowledge base, a SurrealDB graph of about 10,800 cited claims with their claimants,
  places, numbers and relations (supports, contradicts, …).
- **[web/](web/README.md):** a static explorer that runs the whole knowledge base in your browser:
  **https://barafael.github.io/bavovniatko/**
- **[design/](design/):** game design notes. **[game-data/](game-data/):** the game feed exported from the knowledge base.

## Licence

- **Code:** MIT ([LICENSE-MIT](LICENSE-MIT)) or Apache-2.0 ([LICENSE-APACHE](LICENSE-APACHE)), at your option.
- **Content and data:** CC BY 4.0, with these exceptions:
  - OpenStreetMap-derived place geometry is ODbL 1.0, © OpenStreetMap contributors;
  - quotations remain their authors'.

  See [LICENSE-CONTENT.md](LICENSE-CONTENT.md).
- **The explorer's bundled software** carries its own licences. SurrealDB is under the Business Source License 1.1.
  The site lists every licence under `licenses/`.

The data concerns an ongoing war, and many figures are contested. Claims record what sources say, not established
facts.
