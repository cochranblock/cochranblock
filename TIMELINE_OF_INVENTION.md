<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

## How to Read This Document

Each entry follows this format:

- **Date**: When the work shipped (not when it was started)
- **What**: Concrete deliverable — binary, feature, fix, architecture change
- **Why**: Business or technical reason driving the decision
- **Commit**: Short hash(es) for traceability
- **AI Role**: What the AI did vs. what the human directed

This document exists because AI-assisted code has a trust problem. Anyone can generate 10,000 lines of spaghetti. This timeline proves that a human pilot directed every decision, verified every output, and shipped working software.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### Fish Tank Starfield (April 8, 2026)

**Invention:** A GPU-zero-cost animated starfield using a static mask over a looping background-position gradient — inspired by a $15 scrolling fish tank decoration from an Arizona garage sale.

**The Problem:** Every website with animated backgrounds uses oversized elements (200-400% of viewport) with `transform: translate3d` animations. This allocates massive GPU textures — 4x screen size in memory. Desktop GPUs handle it. Android phones crash, show white screens, drain batteries, and stutter.

**The Insight:** A child's fish tank decoration from the 1990s solves this. Two layers: a static piece of black cardboard with fish-shaped holes punched in it (the mask), and a single strip of colored paper sliding behind it on a motor (the animation). The strip is the same size as the window. The visual effect is identical to moving the entire ocean — but you're only moving a piece of paper.

**The Technique:**
1. `body::after` — static CSS mask with star-shaped holes (radial-gradient cutouts). Never moves. Zero GPU cost.
2. `body::before` — screen-sized element (100% x 100%, not 200%) with `background-size: 200% 200%` and `background-position` animation. The gradient is larger than the element but the element itself is viewport-sized. `background-position` changes are compositor-only — no layout, no paint, no GPU texture reallocation.

**Result:** Same drifting starfield visual. 75fps on headless Chrome. Zero Cumulative Layout Shift. 164ms first paint. Works on every Android phone tested. GPU memory usage: 1x viewport instead of 4x.

**Prior Art:** Parallax scrolling exists. CSS mask compositing exists. But combining a static punch-hole mask with a background-position loop as an alternative to oversized transform animations — specifically to eliminate GPU texture bloat on mobile — is novel. No CSS framework, tutorial, or Stack Overflow answer describes this pattern as of April 2026.

**Named:** Fish Tank Starfield
**Commit:** `11c115f`
**Benchmark:** `whobelooking perf https://cochranblock.org` — 75fps, 0.0000 CLS, 164ms FP
**Origin:** Arizona garage sale, circa 2006-2010. Michael Cochran.

---

## Entries

<!-- Add entries in reverse chronological order. Template:

### YYYY-MM-DD — [Short Title]

**What:** [Concrete deliverable]
**Why:** [Business/technical driver]
**Commit:** `abc1234`
**AI Role:** [What AI generated vs. what human directed/verified]
**Proof:** [Link to artifact, screenshot, or test output]

-->

### 2026-05-06 (later) — Native-Details Hamburger Menu + CDP-Driven Rendering Verification (chromiumoxide)

**What:** Closing the mobile-nav gap on the LET'S TEAM root.

- **Mobile hamburger menu** at `/` (`assets/lets-team.html`) — `<details class="nav-mobile">` with `<summary>` styled as a 40×40 cyan-bordered burger icon (3 stacked bars). When opened: `[open]` state morphs the bars into an X, expands a fixed-position drawer below the brand bar containing the full nav grouped into 4 labeled sections — **On This Page** (Engage / Architecture / Verticals / Registrations) / **Procurement** (Gov Docs / SBIR / VR&E / DCAA / Capability Statement / Resume) / **Receipts** (Open Books / Source / Stats / Binaries / Pulse) / **Site** (Products / Services / About / Contact / Book / Deploy / Grant / Arch / No Quarter) + Read the Doctrine →. CSS-only — no JavaScript. Uses native `<details>`/`<summary>` pattern which renders reliably across all browsers (the `<input type="checkbox">` + `<label>` toggle pattern was attempted first but fought chromium-headless rendering at the small viewport).
- **chromiumoxide CDP audit binary** — standalone Rust crate at `/tmp/burger-check/` that drives Chromium via the DevTools Protocol. Sets iPhone-like viewport (390×844, deviceScaleFactor=2, mobile=true, iOS Safari UA), navigates, then evaluates JS to query the actual computed DOM state of `.nav-mobile` and its `<summary>`. Settles whether elements are *rendering* — not just whether the headless CLI can capture them. Output: full computed-style dump (`display`, `visibility`, `position`, `bbox`, `z-index`, etc.) plus before/after screenshots of the closed/open hamburger states. Confirmed the burger renders at the correct position (40×40 box at 335.6, 11.2 — top-right of mobile viewport) with cyan border + 12% cyan fill, and the click-to-open animation works end-to-end.
- **Headless-CLI quirk documented** — `chromium --headless=new --screenshot` at small viewports (e.g. 390×844) failed to capture `position: fixed` elements layered over the cosmic backdrop (`html::before` + `html::after` radial gradients), even when computed style confirmed the element was rendering at the right place with full opacity. Switched to chromiumoxide CDP screenshots which use the same DevTools API real browsers use — captures correctly. Lesson: headless CLI screenshots are NOT the source of truth; CDP introspection + screenshots are.
- **Visual upgrade for the hamburger button** — bumped the summary from `1px solid var(--border-strong)` (translucent cyan, low contrast against dark backdrop) to `1.5px solid var(--accent)` + `background: rgba(0,217,255,0.12)` + drop-shadow, so the burger reads clearly against the cosmic backdrop on actual phone screens.

**Why:** The phone-portrait nav had `display: none` on `.nav-links` with no replacement, leaving phone visitors with only the brand at the top — they had to scroll to the 4-group footer for navigation. User caught this and asked for a "hamburger stack that makes sense." The four-group ordering (On This Page → Procurement → Receipts → Site) mirrors the buyer journey: scan-the-page anchors first, then the procurement-officer artifacts (cap statement, resume, gov docs), then evidence (openbooks, source, stats), then the rest of the site. Headless-CLI rendering kept producing empty captures despite correct CSS, so the chromiumoxide CDP audit was built to *prove* via computed-style introspection that the hamburger was actually rendering — separating "real bug" from "headless-tooling quirk."

**Commit:** Same uncommitted set as the prior 2026-05-06 entry plus:
- `assets/lets-team.html` extended with the `<details class="nav-mobile">` markup + `.nav-mobile-menu` 4-group content (~70 lines added) and the corresponding CSS (`.nav-mobile`, `.nav-mobile > summary`, `.nav-mobile-menu`, `.nav-mobile-menu .nav-group-head`, mobile media query) (~80 lines added).
- `/tmp/burger-check/` — standalone Rust crate using chromiumoxide 0.7 (tokio-runtime, no fetcher) for CDP-driven rendering verification.
- `screenshots/burger-cdp.png` (352 KB) — closed-state proof
- `screenshots/burger-cdp-open.png` (246 KB) — open-state proof, drawer expanded

**AI Role:** Claude implemented all CSS and HTML, wrote the chromiumoxide audit binary from scratch (correctly debugged the chromiumoxide 0.7 feature flags after a first wrong guess), and ran the verification. Human directed: catching the missing-mobile-nav gap, demanding the hamburger stack be ordered to "make sense," refusing to accept "I gave up on chromium-headless verification" as an answer ("that wasn't a fucking option"), and naming the right tool ("chromiumoxide rust it") that closed the loop.

**Proof:**
- Live: https://cochranblock.org/ at narrow viewport — 40×40 cyan-bordered ☰ at top-right.
- CDP-verified computed style: `.nav-mobile { display: block; position: fixed; top: 11.2px; right: 14.4px; z-index: 100; visibility: visible; opacity: 1; bbox 40×40 at (335.6, 11.2) }`.
- Screenshots: `screenshots/burger-cdp.png` (closed), `screenshots/burger-cdp-open.png` (open with drawer expanded showing all 4 groups).
- DOM count: 1 `<details>`, 1 `<summary>`, 1 `<div class="nav-mobile-menu">`, 25 `<a>` items.

### 2026-05-06 — LET'S TEAM Apex Root + Two-Act Manifesto Fold + Cap-Statement-Styled Resume + Trust-Chip Strip + HTML→PDF Render Pipeline + AI Orchestration Positioning

**What:** Major site reorganization centered on subcontractor-hire positioning while preserving doctrine moat.

- **Apex root replaced** — `cochranblock.org/` now serves `assets/lets-team.html` (formerly served the anti-founder manifesto). Hero: `LET'S TEAM.` Three-mode subhead: "We sub on your contract, we prime your SBIR, we teach our architecture under the Unlicense." Trust strip flex-wraps atomic credential chips so long values (UEI W7X3HAQL9CF9, EIN 41-3835237, etc.) never clip on phone portrait. 6 hero engagement-mode pills + 8-mode AS PRIME column (SBIR I/II/III, STTR, D2P2, JV, Mentor-Protégé, STRATFI/TACFI) and 3-mode AS SUB column (Teaming, Subcontract, IDIQ). Architecture Taught Under the Unlicense section (7 patterns, public-domain commitment). Verticals: Defense / Healthtech / Fed Civilian with FAR/DFARS/NIST/CMMC bullets. 2026 Procurement Posture banner naming CISA Secure-by-Design, NSA/CISA June 2025 CSI, DARPA TRACTOR, DoD CIO FY25-26 Software Mod Plan. Dual-Use Proof table with KNOXAI as commercialization anchor. Past Performance split into 3 sections: Corporate (Cochran Block) / Military Service (USCYBERCOM J38 JMOC-E + ARCYBER 103rd CMT + NMT) / Contractor Employment (Two Six + MaxisIQ, with real dates).
- **Two-Act Manifesto Fold** — `assets/manual.html` rewritten to fold the standalone manifesto (now Act I, "THE ANTI-FOUNDER") into the operations manual (Act II, "THE MANUAL") on a single scroll, joined by an amber-rule seam with dimmed diamond glyph. Sticky TOC rail kicks in at the seam. `assets/anti-founder.html` deleted. `/manual`, `manual.cochranblock.org/*`, `/anti-founder`, `/antifounder`, `/receipts`, `/eat-the-founder-software-market` all serve the folded asset; deep-linkers append `#doctrine` or `#manual` to land at either half.
- **Cap-statement-styled resume** at `/resume` (`assets/resume.html`) — banner reads `MICHAEL COCHRAN` (cyan/white split, Orbitron 26pt 900) so ATS keyword filters and corporate recruiters don't get tripped by an "ANTI-FOUNDER" header. Cyan/dark anti-founder aesthetic, JetBrains Mono, letter-format `@page`. Single-column print flow with `break-inside: avoid` on credential clusters keeps the doc to exactly 2 pages. Quantified achievement bullets per role (Hire Heroes USA / Terrance Ford rewrite incorporated). Phone (443) 900-7903, LinkedIn, GIAC GSEC, Offensive Operations Foundation Course (NSA/USCYBERCOM), `Inactive Top Secret / SCI · CI Polygraph`. Hannah Montana mode (separate ATS + branded variants) explored and rejected — single resume with name-led banner is the correct trade-off.
- **HTML → PDF render pipeline** — `scripts/build-resume-pdf.sh` uses chromium-headless `--print-to-pdf` against `assets/resume.html` (with `sed` rewrite of root-relative URLs to absolute `https://cochranblock.org/...` so PDF hyperlinks survive standalone). Site `/resume` and downloadable PDF now share a single source of truth. Print stylesheet hides the topbar, pdf-nudge banner, backrefs grid, and cosmic backdrop, leaving the clean letter-format doc.
- **PDF renames + back-compat** — `assets/capability-statement.pdf` → `assets/cochranblock-capability-statement.pdf`; `assets/resume.pdf` → `assets/michael-cochran-resume_may_2026.pdf`. `src/web/assets.rs` keeps the legacy bare names mapped to the new files; `/resume.pdf` 308-redirects. External links don't 404.
- **Custom AI orchestration over curated data** — added across Architecture Taught (7th pattern), Dual-Use Proof (new row), and all three verticals (Defense / Healthtech / Fed Civilian). Doctrine line: *"Data is the moat, not the model."* Researched and curated corpora paired with on-device inference = dual-use AI defensible without an OpenAI bill or FedRAMP cloud auth.
- **Trust-Chip Strip layout** — credentials display restructured from inline `<br>`-separated text to flex-wrap atomic `<span class="chip">` units. Each credential is its own self-contained pill that wraps cleanly without clipping at the viewport edge on phone portrait. Mobile rule: `font-size: 0.66rem; padding: 0.15rem 0.4rem; overflow-wrap: anywhere; word-break: break-word;`.
- **Cosmic backdrop on new pages** — lifted starDrift + per-page ambient gradient + shooting stars CSS from `assets/css/main.css` into `assets/lets-team.html` and `assets/resume.html` (resume backdrop wrapped in `@media screen` so PDF print stays clean). Visual identity now matches `/govdocs` and `/services`.
- **Backref expansion** — lets-team gets a 4-group footer grid (Engage / Gov / Receipts / Site, 30+ links) plus 3 dropdown nav menus (Gov / Tools / Site) plus inline cross-refs in the body. 46 backref links to other site sections (was 0). Resume gets a 4-group `.backrefs` strip in the footer (Engage / Gov / Receipts / Doctrine, screen-only — hidden in print).
- **Mobile + tablet portrait fixes** — `clamp(1.7rem, 10vw, 3.4rem)` banner on phone (was 14vw and overflowing), past-performance + dual-use tables card-collapse `<720px`, defensive `overflow-wrap: anywhere`/`min-width: 0` guards, hero pills `0.62rem` so 6 fit 3+3 on 390px, footer 4-col → 2-col `<1024px`, verticals 3-col → 2-col → 1-col, third vert grid-column reset at `<820px` (was leaking `span 2` from tablet rule).
- **Test scaffolding** — `src/tests/http.rs` got 14 new tests (`lets_team_root_serves_buyer_page`, `lets_team_root_engagement_modes`, `lets_team_architecture_taught_unlicense`, `lets_team_dual_use_proof_table`, `lets_team_past_performance_three_sections`, `lets_team_2026_procurement_posture`, `lets_team_alias_routes_200`, `manual_serves_folded_doctrine_plus_ops`, `manual_recursive_aliases_200`, `legacy_manifesto_paths_serve_folded_manual`, `removed_anti_founder_asset_404`, `resume_html_capability_styled`, `resume_html_six_job_split`, `resume_pdf_still_at_assets`) and 4 updated tests (`index_business`, `home_ctas`, `hero_product_status`, `buttons_hero_ctas_200`) for the new routing.
- **Multi-Viewport Screenshot Pipeline** — `scripts/screenshots.sh` drives chromium-headless across phone-portrait (390x844), phone-landscape (844x390), tablet-portrait (768x1024), tablet-landscape (1024x768), and desktop (1280x800) for `/`, `/resume`, `/manual`, `/govdocs`, `/services`. UI/UX simulation in CI-style; 25 captures per pass.
- **Routing additions** — `/lets-team`, `/team`, `/teaming` all alias to apex root buyer page. `/resume` route + `f_resume_html` handler. Manual subdomain dispatch in `f2_root` swapped from manifesto to folded manual.
- **Operating agreement** — left untouched but audited for SDVOSB/SBIR/teaming/JV/MPP/dual-use coverage. Already aligned. One pending edit (SDVOSB row: `Application in progress` → `Final Review`) deferred per user.

**Why:** The doctrine is the moat for direct-pursuit channels but the wrong opener for prime BD scouts, contracting officers, and ATS-routed recruiters. Splitting persona — `LET'S TEAM.` apex root for buyers, folded manual for the doctrine-curious, name-led resume for the ATS gauntlet — preserves the brand while surviving the resume pipeline. `MICHAEL COCHRAN` as the resume banner avoids the "anti-X" pattern-match that gets resumes filtered. Trust-chip restructure was triggered by user reporting "the box under JV/Mentor-Protégé looks weird on portrait cell" — was credential text clipping at the viewport edge on phone. HTML→PDF pipeline ensures the artifact downloaded by a contracting officer matches the page they just read, not a stale ReportLab-rendered version diverging quietly. AI-orchestration-over-curated-data positioning was added at user request — Michael has shipped multiple instances and "data is the moat, not the model" is the dual-use credibility story SBIR evaluators want.

**Commit:** Uncommitted at time of writing (deployed live to gd via direct binary scp + hot-reload). Files changed:
- `assets/lets-team.html` (new, 51 KB)
- `assets/resume.html` (new, 34 KB, banner: MICHAEL COCHRAN)
- `assets/manual.html` (rewrite — folded manifesto + seam + ops)
- `assets/anti-founder.html` (deleted — folded into manual.html)
- `assets/cochranblock-capability-statement.pdf` (renamed from capability-statement.pdf)
- `assets/michael-cochran-resume_may_2026.pdf` (renamed from resume.pdf, regenerated from HTML)
- `src/web/pages.rs` (new handlers: `f_resume_html`, `f_lets_team`; `f2_root` updated; `f_anti_founder` now serves folded manual)
- `src/web/router.rs` (`/resume`, `/lets-team`, `/team`, `/teaming` routes; legacy PDF redirects)
- `src/web/assets.rs` (new asset keys, legacy aliases preserved)
- `src/web/whyme.rs` (resume PDF link updated)
- `src/tests/http.rs` (14 new + 4 updated tests, ~400 lines added)
- `scripts/build-resume-pdf.sh` (new)
- `scripts/screenshots.sh` (new)
- `screenshots/prod-*.png` (14 multi-viewport captures of production)

**AI Role:** Claude (this agent) implemented all HTML/CSS/Rust edits, ran the chromium-headless render + screenshot pipelines, and drove the deploy. Human directed: the brand-vs-pipeline tension recognition that drove the MICHAEL COCHRAN banner switch, the "Hannah Montana this shit" exploration and subsequent rejection, the AI-orchestration-over-curated-data positioning, the trust-chip-on-cell observation, identification of mobile portrait failures, the SDVOSB Final Review status calibration, and the cosmic-backdrop-must-stay-throughout aesthetic constraint.

**Proof:**
- Live: https://cochranblock.org/, https://cochranblock.org/resume, https://manual.cochranblock.org/
- PDF: https://cochranblock.org/assets/michael-cochran-resume_may_2026.pdf (290 KB, 2 pages, generated from /resume HTML)
- Screenshots: `screenshots/prod-{lets-team,resume,manual,govdocs,services}-{phone,tablet,desktop}-{portrait,landscape}.png` (14 captures)
- Cloudflare cache purged across all 10 domains under the approuter account post-deploy.

### 2026-04-09 — /inventions Page + Truth Audit + Defense Contractor Benchmarks + Mobile Polish

**What:** Added /inventions page cataloguing 7 inventions (no known prior art), 4 original techniques, and 18 production engineering contributions — all with commit provenance. Truth audit updated all site metrics to live deployment benchmarks (9.9MB binary, 116ms TTFB, 176ms first paint, 72fps, 0.0000 CLS, 131 DOM elements). /speed page replaced Wix comparison with defense contractor benchmarks — cochranblock.org vs Booz Allen, Leidos, SAIC, CACI. Capability statement updated with EIN, ACH, 4 crates.io crates, 7 named inventions, CSS v4. Mobile fixes: nav menu default state, search overlap on <480px, horizontal table scroll, Facebook webview. SAM.gov status updated to Active (CAGE 1CQ66). Fish Tank Starfield invented and documented in Human Revelations section. JCAC date corrected to 2014.
**Why:** The /inventions page is the IP portfolio — proof that human-directed AI development produces original work, not generated spaghetti. Truth audit ensures every number on the site matches live deployment. Defense contractor benchmarks position a $10/month laptop against billion-dollar cloud infrastructure. Mobile fixes triggered by real user reports (Facebook webview on Android).
**Commit:** `ff54f91` (inventions), `1471258` (truth audit), `fe0dfbf` (/speed benchmarks), `569b8df` (capability statement), `99bed7a` (mobile tables), `efc7a69` (nav search), `c61736d` (nav menu), `11c115f` (fish tank), `4556efd` (human revelations), `2cf5781` (android GPU), `a6233ed` (SAM active), `fe29702` (JCAC fix), `1fc51cf` (fish tank date fix)
**AI Role:** AI implemented all code, page layouts, and CSS fixes. Human invented Fish Tank Starfield, classified inventions vs techniques, directed truth audit scope, chose defense contractor comparisons, and identified mobile bugs from user reports.
**Proof:** cochranblock.org/inventions, cochranblock.org/speed, 14 commits in one session

### 2026-04-08 — Fish Tank Starfield + Android GPU Fix + whobelooking Contract Scout

**What:** Invented the Fish Tank Starfield technique — static CSS mask with background-position loop replaces oversized transform animations. Eliminated Android GPU crashes. Also built whobelooking contract scout: 8 federal APIs (SAM.gov, USASpending, SBIR, Federal Register, Grants.gov, Regulations.gov, CALC+, Contract Awards), typed schemas (Bid/Award/Signal/Rate), zstd+sled cache, headless Chrome scraper, perf benchmark tool. Scraped 451 SAM.gov opportunities. Updated SAM.gov status to Active, CAGE 1CQ66.
**Why:** Facebook user reported Android browser issues with cochranblock.org. Root cause: 200% oversized body::before with transform animation — 4x GPU memory. Fix inspired by childhood memory of a scrolling fish tank decoration from an Arizona garage sale. The same session produced a full federal contract discovery pipeline.
**Commit:** `11c115f` (fish tank), `f08cad3` (original negative space), `2cf5781` (mobile GPU fix), whobelooking commits
**AI Role:** AI implemented all code changes. Human invented the fish tank pattern from a childhood memory, directed the architecture for whobelooking, and identified which contract opportunities to pursue.
**Proof:** `whobelooking perf https://cochranblock.org` — 75fps, 0.0000 CLS, 164ms first paint. `whobelooking scout` — 713 results from 8 APIs.

### 2026-04-02 — any-gpu Added as 15th Repo, Full Docs Audit

**What:** Added any-gpu (GPU tensor engine via wgpu — AMD/NVIDIA/Intel/Apple from one codebase) to products page, codeskillz metadata, tinybinaries leaderboard, velocity API, llms.txt, and search index. Updated all repo counts from 14→15 and Unlicense counts from 12→13 across ~50 site references. Full documentation audit: PROOF_OF_ARTIFACTS.md metrics updated (92 functions, 10 types, 5,980 LOC, 50 routes), architecture diagram corrected, README binary sizes fixed. CSS-only nav redesign shipped. Deployed to gd (x86 production).
**Commit:** `e6de722` (any-gpu addition), subsequent commits for docs audit
**AI Role:** AI executed all site updates, metric counting, and cross-referencing. Human directed product addition and audit scope.

### 2026-03-28 — Native Search Engine + Federal Compliance Docs (SBOM, SSDF, CMMC, Security)

**What:** Added /search — native Rust full-text search with 16-entry compile-time index. GET /search?q= returns matching pages with highlighted snippets, sub-millisecond. No external search service, no JS library — 60 lines of Rust. Added SBOM (38 deps with versions/licenses/purpose), SSDF compliance matrix (PS/PW/RV/PO mapped to actual practices), CMMC Level 1-2 (11 practices across 8 domains), and Security posture (crypto primitives table with FIPS status, attack surface analysis) to /govdocs.
**Why:** Search is the flex — a full-text search engine inside a 15MB binary. Federal compliance docs make /govdocs a one-stop procurement package.
**Commit:** `52b025a`, `c08c568`
**AI Role:** AI built the search index and compliance matrices. Human directed which pages to index and verified all SSDF/CMMC mappings.
**Proof:** cochranblock.org/search, cochranblock.org/govdocs

### 2026-03-28 — /source Page + /vre Business Plan + Capability Statement Enhancement

**What:** Added /source — site serves its own Rust source code via `include_str!`, baked into the binary. Added /vre — full VR&E Category II self-employment business plan as a web page with live cross-references to /tinybinaries, /codeskillz, /services, /govdocs. Enhanced /govdocs capability statement with core competencies, past performance (5 entries), differentiators framed as procurement benefits, CAGE code pending, 518210 NAICS code. Added "Read the Source" CTA to hero.
**Why:** /source is the ultimate proof — the site IS the demo. /vre lets a VA counselor verify every claim by clicking through the site. Capability statement upgrade for federal matchmaking events.
**Commit:** `2584d57`, `9361a7c`, `f48f31b`
**AI Role:** AI built all three features. Human directed the VR&E content, capability statement structure, and source page concept.
**Proof:** cochranblock.org/source, cochranblock.org/vre, cochranblock.org/govdocs#capability

### 2026-03-27 — /tinybinaries Page + Binary Size Optimization

**What:** Added /tinybinaries page with binary size leaderboard (10 projects, 48KB to 51.5MB) and efficiency table (KB per function). Changed release profile from `opt-level=3` to `opt-level='s'` — binaries dropped 16-17% (x86: 18MB→15MB, ARM: 9.9MB→8.2MB). Updated all size claims site-wide.
**Why:** Prove the tiny binary claims with real measured data. Smaller binaries = smaller attack surface = better federal procurement story.
**Commit:** `453c7b7`, `42de147`
**AI Role:** AI benchmarked opt-level options and built the page. Human directed the optimization strategy and approved the page layout.
**Proof:** cochranblock.org/tinybinaries

### 2026-03-27 — QA Rounds 1 & 2 + Clippy Clean

**What:** Full QA audit — clean compile, zero warnings, zero debug prints, zero AI slop words, all 17 routes verified 200. Clippy with `-D warnings` — fixed collapsible-if patterns in webhook handlers and frontmatter stripper.
**Why:** Production QA gate before shipping to real users. Every claim must be verifiable.
**Commit:** `e60d08e`, `2e8299c`
**AI Role:** AI ran the audit and fixed clippy warnings. Human directed the QA process and approved the results.

### 2026-03-27 — Truth Audit + CSB/SDVOSB Distinction + GPU Infrastructure

**What:** Audited every fact on the site. Fixed "14 Unlicense repos" → "14 open source repos" (12 Unlicense, 2 without). CSB (Certified Small Business) marked Approved — separated from SDVOSB (still Pending). Added GPU infrastructure: lf RTX 3070 8GB, gd RTX 3050 Ti 4GB. Doubled starfield to 42 stars.
**Why:** A site that claims to prove everything must be 100% accurate. CSB and SDVOSB are different certifications.
**Commit:** `2e8299c`
**AI Role:** AI audited all pages, verified binary sizes, counted repos, checked licenses. Human corrected the CSB/SDVOSB distinction.

### 2026-03-27 — Docs Sweep — README, CODEOWNERS, OWNERS.yaml

**What:** Rewrote README.md with full 24-route table and correct build command. Updated GITHUB_BANNER.md (8→14 repos). Fixed PROOF_OF_ARTIFACTS.md (11→14 repos, SAM.gov status). Replaced kova-specific CODEOWNERS and OWNERS.yaml with cochranblock-specific subsystems.
**Why:** Docs referenced kova files that don't exist in this repo. Stale paths, wrong repo counts.
**Commit:** `5b62dbb`
**AI Role:** AI identified stale references and rewrote docs. Human approved all changes.

### 2026-03-27 — SEO: Per-Page Meta, Canonical URLs, llms.txt, security.txt, FAQ Schema

**What:** Added per-page meta descriptions and og:description (7 pages). Added `<link rel="canonical">` to every page. Created /llms.txt (AI crawler context), /.well-known/security.txt (RFC 9116), /humans.txt (team + tools). Added FAQPage JSON-LD schema with 7 questions. Updated 6 GitHub repos with cochranblock.org backlinks.
**Why:** AI crawlers, search engines, and federal scanners all read these signals. Per-page SEO means each page ranks for its own terms.
**Commit:** `ccd9896`, `05a7047`
**AI Role:** AI built all SEO infrastructure. Human directed which pages needed descriptions and approved FAQ content.

### 2026-03-27 — SBIR Technical Approaches for 11 Federal Agencies + Bid Tracker

**What:** Added SBIR/STTR technical approach sections to /govdocs for DoD, NSF, DHS, NASA, DOE, USDA, EPA, DOT, NIST, NIH, and NOAA — 11 agency-specific proposals with Phase I/II objectives, past performance, and commercialization. Added color-coded upcoming bids tracker table (orange=blocked, cyan=ready, teal=open, purple=submitted, green=won).
**Why:** SAM.gov pending. When it clears, proposals need to be ready to submit immediately. The compressed solicitation window (April-August 2026) means no time to draft from scratch.
**Commit:** `87abae3`, `f417f61`, `4dc762e`, `0183dc1`, `72760b2`
**AI Role:** AI researched solicitation timelines, drafted agency-specific proposals. Human directed the technical approach and approved all claims.
**Proof:** cochranblock.org/govdocs

### 2026-03-27 — SAM.gov Status Update + 30-Day Speed Flex

**What:** Updated SAM.gov status from "Registered" to "Pending Registration" across all pages. Added flex line on about page: "LLC formed, 14 repos open-sourced, site live, eMMA registered, SAM.gov filed, first partnership signed — all in under 30 days."
**Why:** Accuracy. SAM.gov registration is still pending, not active. The speed flex shows execution velocity.
**Commit:** `898c34c`, `a58cd50`
**AI Role:** AI updated all references. Human directed the status change and approved the flex copy.

### 2026-03-27 — Visual Polish: Starfield, Glassmorphism, Animations, Per-Page Color

**What:** Added drifting starfield (42 CSS radial-gradient dots, 100s drift cycle), two shooting stars (20s/30s cycles), glassmorphism on cards (backdrop-filter blur), staggered fadeInUp hero animations, per-page ambient color gradients (8 pages), glow effects on buttons/inputs/nav. Rocket launch button on /deploy/confirmed with 3-2-1 countdown.
**Why:** The site sells technology — it should look like technology. Professional but atmospheric.
**Commit:** `cec512d`, `1698e76`, `b48d5ec`, `e897c4c`, `719bf95`, `c90b338`, `91dd2a1`, `7c25cf1`
**AI Role:** AI implemented all CSS effects and the rocket JS. Human directed the aesthetic — "space theme, professional, not distracting."

### 2026-03-26 — Ghost Fabric Edge Intelligence + Cost Analysis Rewrite

**What:** Added Ghost Fabric edge intelligence section to /mathskillz with Python vs Rust deployment math. Rewrote cost analysis page with specific AWS scenario, 37signals validation, always-visible summary table, and trust/compliance section.
**Why:** Need public cost justification for VR&E self-employment track approval, government procurement officers, and LinkedIn sharing.
**Commit:** See `git log --oneline -5`
**AI Role:** AI researched marketing best practices and drafted copy. Human directed all positioning and approved numbers.
**Proof:** cochranblock.org/mathskillz

### 2026-03-25 — eMMA + SAM.gov + VR&E Pipeline

**What:** Registered as Maryland eMMA vendor (SUP1095449), submitted CSB certification, filed W-9, submitted VA VR&E Chapter 31 self-employment track, submitted LiftFund business loan application.
**Why:** Building procurement pipeline. SDVOSB + eMMA + SAM = federal and state contract eligibility.
**Commit:** Non-code business milestone.
**AI Role:** AI assisted with form completion, UNSPSC code research, W-9 PDF generation. Human made all business decisions and submissions.

### 2026-03-25 — Veteran-Owned Branding + SDVOSB Badges

**What:** Added "Veteran-Owned" to hero status line, "SDVOSB Pending" to footer, updated about page founder bio with Army 17C and SDVOSB status. Added /mathskillz to nav.
**Why:** SDVOSB certification pending. Defense contractors and govt buyers search for these signals.
**Commit:** `bcf5b01`
**AI Role:** AI placed badges in correct locations. Human directed which credentials to display.

### 2026-03-25 — /mathskillz Cost Analysis Page

**What:** New page at /mathskillz with cloud-vs-bare-metal cost breakdown, ROI calculations, and pricing justification.
**Why:** Public math for sales conversations, VR&E counselor review, and LinkedIn sharing.
**Commit:** `88099c8`
**AI Role:** AI drafted initial version. Human directed all pricing and positioning.

### 2026-03-24 — PWA + Services + Downloads + Social Cards

**What:** Added PWA manifest for mobile install, /services pricing page ($3,500 base, $225/hr), /downloads page with binary downloads, og:image social card, diamond logo.
**Why:** Complete the sales funnel — pricing, downloads, social sharing, mobile install.
**Commit:** `b57bc18`, `34beb3e`, `86af3be`, `b6638b8`, `c4d021c`
**AI Role:** AI built pages and generated PWA assets. Human directed pricing, copy, and branding.

### 2026-03-23 — Zero-Cloud Rebrand & Binary Compression

**What:** Resized embedded product images (15.6MB → 2.6MB), dropping the release binary from 21MB to <10MB. Rewrote homepage hero to break the fourth wall — tells visitors they're looking at the <10MB binary right now.
**Why:** The binary size IS the pitch. A Fractional CTO selling zero-cloud architecture needs the live demo to prove the claim at first glance.
**Commit:** See HEAD
**AI Role:** AI executed image compression and copy drafts. Human directed the brand positioning, approved final copy, verified binary size.
**Proof:** `ls -lh target/release/cochranblock` → 9.8MB

### 2026-03-22 — Pixel Forge Product Card

**What:** Added Pixel Forge to open source product section with logo and description.
**Commit:** `1d087c5`
**AI Role:** AI generated card HTML. Human directed placement and copy.

### 2026-03-20 — Sovereign Intelligence Identity Alignment

**What:** Aligned site identity with business card branding.
**Commit:** `8c47091`
**AI Role:** Human directed all brand decisions. AI executed code changes.

### 2026-03-18 — CRT Terminal → Proper Intake Form

**What:** Replaced novelty CRT terminal deploy page with a real Zero-Cloud Tech Intake Form for lead generation.
**Why:** Business pivot from "cool demo" to "convert founders into Fractional CTO clients."
**Commit:** `eed6ba0`
**AI Role:** AI built form HTML/validation. Human designed the form fields and conversion flow.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
<!-- COCHRANBLOCK-BRAND-FOOTER:START - generated by cochranblock/scripts/brand-stamp.sh -->

---

<sub>&#9656; **THE COCHRAN BLOCK, LLC** &#183; CAGE `1CQ66` &#183; UEI `W7X3HAQL9CF9` &#183; UNLICENSE &#183; [cochranblock.org](https://cochranblock.org)</sub>
<!-- COCHRANBLOCK-BRAND-FOOTER:END -->
