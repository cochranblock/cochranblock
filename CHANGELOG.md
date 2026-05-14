# Changelog

All notable releases of `cochranblock` (the single-binary website + LLC ops engine).

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow SemVer-ish — pre-1.0, minor bumps may include breaking site-route or DB-format changes. Dates are commit dates (US Eastern).

This file was assembled retroactively on 2026-05-14 from `git log` to bookmark milestones that had author-named version commits but no tags (v0.2.0, v0.3.0, v0.5.0), the existing pushed tag (v0.6.0), and the post-v0.6.0 work that spanned five thematic waves through May 2026.

---

## [v0.10.0] — 2026-05-14 — Nav Consistency

The dropdown nav from the LET'S TEAM apex (`/`) now ships site-wide. Every other page used to land users on the older checkbox-hack hamburger (which, per the May 6 ToI entry, "fought chromium-headless rendering at the small viewport"). That's gone.

### Changed
- `assets/css/main.css`: replaced `.nav-check`/`.nav-toggle`/`.nav-toggle-bar` rules with the `.nav-mobile` / `.nav-mobile-menu` / `.nav-mobile-menu .nav-group-head` block ported from `assets/lets-team.html`. Added CSS variables `--border-strong` and `--amber` to `:root`. Mobile `@media (max-width: 768px)` block now hides `.nav-links` and `.nav-search` (both reachable via the drawer) and activates `.nav-mobile`.
- `src/web/pages.rs` `C7` constant: swapped `<input class="nav-check"> + <label class="nav-toggle">` for `<details class="nav-mobile">` with a three-group drawer (Procurement / Receipts / Site) matching the apex pattern. Search box retained in the inline desktop nav, hidden on phone breakpoints.

### Fixed
- 9 additional stale "SDVOSB submitted" strings in `src/web/pages.rs` (JSON-LD ORG schema, govdoc note, two search-index bodies, founder bio, capability statement section). Missed by the 2026-05-13 sweep.
- `C8` footer constant: "SDVOSB · Submitted" → "SDVOSB · Certified 2026-05-12".

### Added
- `CHANGELOG.md` (this file).
- `docs/releases/v*.md` — per-release body content for the GitHub Releases UI.

---

## [v0.9.0] — 2026-05-13 — SDVOSB Certified

SBA VetCert issued the Service-Disabled Veteran-Owned Small Business certification on 2026-05-12 (expires 2029-05-12). Sweep across the entire public surface — site, capability statement, federal procurement docs, embedded JSON-LD, brand badge, the org-profile README at `github.com/cochranblock`.

### Changed
- `assets/brand/header-snippet.md` — shields.io badge flipped from yellow `SDVOSB Pending` to green `SDVOSB Certified`.
- `assets/capability-statement.html` — registrations table + sole-source bullet (removed the "once approved" conditional).
- `assets/lets-team.html` — meta description, hero sub, trust-chip strip, registrations table, footer note, foot strip.
- `assets/resume.html`, `assets/manual.html`, `assets/railgun-rosetta.html` — Authorizations / selling pitch / footer.
- `src/web/pages.rs` — whyme bio, bid-status table row (`bid-blocked` → `bid-ready`), JSON-LD FAQ "Is CochranBlock veteran-owned?" answer.
- `assets/52-days.html`, `assets/amendment-003.html`, `assets/supplement-msu-2026-04.html` — dated procurement records (status field within them is a live reference for evaluators, not a historical snapshot).
- `PROOF_OF_ARTIFACTS.md` certification row.
- `src/tests/http.rs` `hero_product_status` test — assertion flipped from "Final Review" to "SDVOSB" + "Certified" pair.

### Fixed
- JSON-LD FAQ "Does CochranBlock work with government agencies?" answer claimed SAM.gov registration was pending — it's been Active for months. Now matches every other page on the site.

### Wording locked
`SDVOSB Certified 2026-05-12 · expires 2029-05-12 (SBA VetCert)` — applied consistently across all surfaces.

---

## [v0.8.0] — 2026-05-07 — redb Consolidation, RSA Chain Elimination

Architectural cleanup: collapse the dual sled + SQLite storage layer into a single embedded ACID database. Removes 5 Dependabot advisories at the dependency-tree level by eliminating crates that transitively pulled `rsa`.

### Changed
- `db`: consolidated `sled + sqlx` → `redb` (single-file ACID, no `rsa` chain in the dep graph).
- `deps`: dropped `sqlx` macros + migrate features — eliminates `rsa` from graph entirely.
- `deps`: cleared all then-open Dependabot advisories — dropped unused `lers`, bumped `aws-lc-sys` + `quinn-proto` + `rustls-webpki` + `reqwest`.
- `deploy`: cargo-audit gate fixed (exit code, not regex) — previous grep-based gate matched the wrong line prefix and silently passed real advisories.

### Build impact
- Binary size and cold-start unchanged (redb is comparable in footprint to sled).
- Single-file DB makes backups + filesystem snapshot strategies one-step instead of two.

---

## [v0.7.0] — 2026-05-06 — LET'S TEAM Apex Root + Native-Details Hamburger

The April spring stack landed: `/sovereignty`, `/manifesto`, `/operations`, `/onboarding`, `/no-quarter`, `/knox` + `/knoxai` redirects, the gated `/john` brief, the Potato Index, KNOXAI product-first restructure, the operator roster, the pitch deck (multiple iterations against John Szeder's frameworks), `/openbooks` (local git, no GitHub API), three Operating Agreement amendments, the capability statement HTML→PDF refresh, and the `/changelog` route fed live from the GitHub commit API. Closed out with the LET'S TEAM apex root on `/` — a folded manifesto + MICHAEL COCHRAN resume + cap-statement-styled trust-chip strip — using the native `<details>`/`<summary>` hamburger that this v0.10.0 just generalized.

### Added (selective — full list via `git log v0.6.0..v0.7.0`)
- `/sovereignty` — six-proof audit page with live server facts.
- `/manifesto` — visceral mission declaration in founder voice.
- `/operations` — signed Operating Agreement + Manifesto.
- `/onboarding` — handbook for operators.
- `/no-quarter` + `/hunt` + `/receipts` + `/mission` aliases — mission doctrine, full harm-class matrix.
- `/knox` + `/knoxai` — KNOXAI mystery page, operator registration endpoint at `/knox/apply` (with hazmat suit entrance challenge), host-aware routing on `knox.cochranblock.org`.
- `/john` — direct brief, no gate.
- `/openbooks` — local git, no GitHub API; full transparency book.
- `/changelog` — live commit feed from GitHub API.
- `/pulse` — minute-resolution threat + biz intel, 60s cache, auto-refresh.
- `/security` — security blitz page with buzzword router.
- The Potato Index v1/v2/v3 (`/stats`) — measure websites in potatoes; ESP32-NOW micro scaling model; grocery-store math for regular people.
- 1,526-websites-on-one-laptop vs. cloud pricing comparison.
- Pitch deck v1 → v2 — finance-first pivot [P26 MOONSHOT], Docker Hub model, dual scenario projections with regulatory catalyst.
- `/manual` + `/railgun-rosetta` routes + origin-side visit logging + HSTS preload.
- LET'S TEAM apex root + folded manifesto + MICHAEL COCHRAN resume + native-details hamburger (this release's namesake).
- Operating Agreement Amendments 001 (Chicken and Egg Bypass Protocol + P31 Unbreaking Software), 002 (MMLC), 003 (10-slide pitch deck).

### Changed
- All Rights Reserved sweep — killed Unlicense references sitewide (the LLC retains commercial IP; per-contract Unlicense flip on government systems still applies).
- Capability statement: anti-founder restyle (cyan/dark) + healthtech target.
- Capability statement HTML → PDF refresh pipeline.
- Truthiness sweep — fix all stale stats: SDVOSB submitted, binary sizes, LOC, tests (note: full SDVOSB sweep didn't land until v0.9.0).
- Mobile nav fixes — hide nav search on mobile (<480px) to stop it overlapping the brand logo; mobile nav menu correctly starts closed (was open by default because desktop `display:flex` was overriding mobile `display:none`).
- crates.io counter: 22 → 32 (`pinned approuter-client + exopack for publish`).

### Build / infra
- Release profile: `opt-level` 3 → 's' (16-17% smaller binaries).
- Android: 6.1 MB AAB — cochranblock.org as a native Android app via JNI bridge (server loads as shared lib, works on emulator).
- P27 Full Diamond: `opt-level 's' → 3`, `lto: true → "fat"`.

---

## [v0.6.0] — 2026-03-24 — Downloads Page (pre-existing tag)

Existing tag at `c4d021c1fe`. This was the originally-pushed v0.6.0 tag — left in place to preserve consumer references. The "v0.6.0" wording first appears in `git log` at commit `f3d3297d54` (2026-03-20, "installable offline app — cargo install cochranblock opens browser"), four days before the tag landed on the downloads-page commit; the tag points at the downloads-page commit, not the version-bump commit.

### Added
- `/downloads` — binary downloads for macOS ARM + Linux x86_64.
- Installable offline app via `cargo install cochranblock` — opens the browser on launch.
- Federal compliance documentation (SBOM, SSDF NIST 800-218, CMMC Level 1-2, supply chain audit).
- `/source` route + capability statement enhancement.
- `/tinybinaries` page + binary size optimization.
- Native search engine.
- Visual regression orchestrator (Sim 4) — full pipeline through exopack.

### Changed
- Cargo.toml `version` field bumped to `0.6.0`.

---

## [v0.5.0] — 2026-03-19 — Holographic Deploy Terminal (retroactive tag)

Author-named version commit `e27f9ce243`. No tag at the time. Tag added retroactively 2026-05-14.

### Added
- Holographic deploy terminal interface.
- Pre-render screenshot pipeline.

### Changed
- Cargo.toml `version` field bumped to `0.5.0` (v0.4.0 was skipped historically — not a typo; the commit jumps `0.3.0 → 0.5.0`).

---

## [v0.3.0] — 2026-03-18 — Version Bump (retroactive tag)

Author-named version commit `40a024dd0d`. No tag at the time. Tag added retroactively 2026-05-14.

### Changed
- Cargo.toml `version` field bumped to `0.3.0`.
- Edition 2024 declared.

---

## [v0.2.0] — 2026-03-11 — Foundational Founders (retroactive tag)

Author-named version commit `7446432613`. No tag at the time. Tag added retroactively 2026-05-14.

### Added
- Foundational Founders attribution model — every file header carries the contributor list (`header-writer` injection rule).
- `Unlicense` sweep across the repository (public-domain release at the file-header level).
- GitHub Actions CI workflow.
- exopack test binary `cochranblock-test` wired into CI (xvfb + chromium + DISPLAY for visual regression).
- `mattbusel` added as permanent XFactor Contributor.
- LinkedIn / social-preview logo in README.

### Changed
- Site-only repo flatten — removed `approuter`, `oakilydokily`, `rogue-repo`, `kova`, `whyyoulying`, `wowasticker` from the monorepo; each got its own repo. Railway abandoned.
- `portfolio` → `cochranblock` rename throughout (env vars `COCHRANBLOCK_*`, DB filename `cochranblock.db`, user-agent string).

---

## [v0.1.0] — 2026-03-10 — Initial Monorepo (retroactive tag)

Initial commit `0064e5e098`. No tag at the time. Tag added retroactively 2026-05-14.

### Added
- Initial monorepo containing `approuter`, `cochranblock`, `oakilydokily`, `rogue-repo`, `kova` as a Cargo workspace.
- `setup-via-api.sh` — automate GitHub + Railway provisioning via API tokens.
- `NEXT_STEPS.md` — initial Railway setup roadmap.

### Why retro-tag this
Anchor the SemVer line at a real first-commit point so `v0.2.0` ("Foundational Founders") has a predecessor it can express a diff against. Pre-flatten state — most of the code that landed here moved out to sibling repos at v0.2.0.

---

[v0.10.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.10.0
[v0.9.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.9.0
[v0.8.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.8.0
[v0.7.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.7.0
[v0.6.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.6.0
[v0.5.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.5.0
[v0.3.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.3.0
[v0.2.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.2.0
[v0.1.0]: https://github.com/cochranblock/cochranblock/releases/tag/v0.1.0
