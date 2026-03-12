<!-- Unlicense — cochranblock.org -->
<!-- Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

<p align="center">
  <img src="https://raw.githubusercontent.com/cochranblock/cochranblock/main/assets/cochranblock-logo.svg" alt="CochranBlock" width="280">
</p>

# cochranblock

## Proof of Artifacts

*Wire diagrams, screenshots, and demos for quick review.*

### Wire / Architecture

```mermaid
flowchart LR
    User[User] --> Home["/"]
    User --> Services["/services"]
    User --> About["/about"]
    User --> Products["/products"]
    User --> Contact["/contact"]
    User --> Book["/book"]
    User --> Federal["/federal-partners"]
```

### Screenshots

| View | Description |
|------|-------------|
| ![Hero](docs/artifacts/screenshot-hero.png) | Hero section |
| ![Products](docs/artifacts/screenshot-products.png) | Products page |
| ![Rogue Repo](assets/img/rogue-repo.png) | Rogue Repo (Products) |
| ![Kova](assets/img/kova.png) | Kova (Products) |
| ![Ronin Sites](assets/img/ronin-sites.png) | Ronin Sites (Products) |
| ![Services](docs/artifacts/screenshot-services.png) | Services page |

### Demo

*Add `docs/artifacts/demo-hero.gif` for hero scroll or Products carousel.*

---

CochranBlock site (cochranblock.org) — Rust Axum server with embedded assets.

## Run

```bash
cargo run -p cochranblock
```

Then open http://localhost:8081 (default). Routes: `/`, `/services`, `/about`, `/contact`, `/book`, `/products`, `/federal-partners`.

## Tokenization

The source code uses **compact identifiers** (f0, t15, s0, etc.) per the Token-Optimized Code Representation whitepaper. See [../kova/docs/TOKENIZATION_IMPLEMENTATION.md](../kova/docs/TOKENIZATION_IMPLEMENTATION.md) and [../kova/docs/compression_map.md](../kova/docs/compression_map.md).

## Docs

- [docs/architecture_guide.md](docs/architecture_guide.md) — Full architecture
- [exopack/docs/testing_architecture.md](../exopack/docs/testing_architecture.md) — Two-binary test model
- [content/whitepaper_text.txt](content/whitepaper_text.txt) — Tokenization whitepaper
