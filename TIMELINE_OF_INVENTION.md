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
