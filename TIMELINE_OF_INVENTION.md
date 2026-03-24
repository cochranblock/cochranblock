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
