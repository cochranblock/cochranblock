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

## Entries

<!-- Add entries in reverse chronological order. Template:

### YYYY-MM-DD — [Short Title]

**What:** [Concrete deliverable]
**Why:** [Business/technical driver]
**Commit:** `abc1234`
**AI Role:** [What AI generated vs. what human directed/verified]
**Proof:** [Link to artifact, screenshot, or test output]

-->

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
