<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Visual and structural evidence that this project works, ships, and is real.*

> This is not a demo repo. This is production software. The artifacts below prove it.

## Architecture

```mermaid
flowchart LR
    User[User] --> CF[Cloudflare Tunnel]
    CF --> AR[approuter :8080]
    AR --> CB[cochranblock :8081]
    AR --> OD[oakilydokily :3000]
    AR --> RR[rogue-repo :3001]
    CB --> Sled[(sled DB)]
    CB --> Assets[Embedded Assets]
```

## Build Output

| Metric | Value |
|--------|-------|
| Binary size (x86) | 15MB (release, opt-level='s', LTO, strip) |
| Binary size (ARM) | 8.2MB |
| Infrastructure cost | $10/month |
| External services | Cloudflare tunnel (free tier) |
| Database | Embedded sled + SQLite — no external DB |
| Cloud dependencies | Zero |
| Public repos | 14 (12 Unlicense, 2 proprietary) |
| Certification | SDVOSB pending · SAM.gov pending registration · eMMA vendor SUP1095449 · CSB approved |
| Functions | 52 |
| Types | 7 |
| Lines of code | 2,905 |
| Direct dependencies | 42 |
| Routes | 27 (20 pages + 4 meta + 3 API) |
| Release profile | opt-level='s', lto=true, codegen-units=1, panic='abort', strip=true |
| GPU nodes | lf: RTX 3070 8GB · gd: RTX 3050 Ti 4GB |
| QA Round 1 | PASS — zero errors, zero warnings, zero debug prints, zero AI slop, all routes 200 |
| QA Round 2 | PASS — clean build, clippy -D warnings, zero uncommitted changes |

## Screenshots

| View | Artifact |
|------|----------|
| Homepage | ![Index](screenshots/index.png) |
| Products | ![Products](screenshots/products.png) |
| Deploy (Tech Intake) | ![Deploy](screenshots/deploy.png) |
| About | ![About](screenshots/about.png) |
| Book a Call | ![Book](screenshots/book.png) |

## How to Verify

```bash
# Clone, build, run. That's it.
cargo build --release -p cochranblock
ls -lh target/release/cochranblock   # <10MB
./target/release/cochranblock         # localhost:8081
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
