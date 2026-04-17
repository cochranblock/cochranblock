# DARPA I2O White Paper
## HR001126S0001 — Information Innovation Office, Office-Wide BAA

**Title:** Sovereign AI Agent Architecture for Zero-Cloud Mission Operations

**Proposer:** The Cochran Block, LLC
**PI:** Michael Cochran — Army 17C (Cyber Operations), JCAC 2014, USCYBERCOM J38 JMOC-E dev lead
**CAGE:** 1CQ66 | **UEI:** W7X3HAQL9CF9 | **NAICS:** 541511, 541512

---

### 1. Problem Statement

Current AI-assisted operations require cloud connectivity, API keys, and third-party model hosting. In contested, denied, or disconnected environments, these dependencies become single points of failure. Analysts in SCIFs, forward-deployed units, and air-gapped networks cannot use AI tools that phone home.

### 2. Technical Innovation

We have built and deployed a working on-device AI agent system (Kova) that eliminates all cloud dependencies. The entire AI inference stack — model loading, tool execution, context management, and distributed orchestration — compiles into a single binary under 30MB.

**Named techniques developed under this effort:**

**Compression Mapping (P13)** — All code symbols (functions, types, fields) are tokenized to compressed identifiers (f0-fN, t0-tN, s0-sN). This reduces AI context consumption by 75%, enabling longer autonomous work sessions within fixed token budgets. 368/368 symbols compressed in the production system.

**Agent Tool Loop** — The LLM calls 13 tools (file read/write/edit, bash execution, glob, grep, code review) in an autonomous loop until the task is complete. Context auto-compacts at 80% budget via LLM-powered summarization — the agent summarizes its own prior work to free tokens without losing key decisions.

**C2 Swarm Mesh** — Distributed orchestration across 4+ worker nodes via SSH with tokenized command protocol. Nodes operate independently during network partition and resync when connectivity restores. The tmuxisfree fleet controller manages 28 concurrent AI agent sessions with rate-limit-aware "Sponge Mesh" broadcast — exponential backoff retry across the fleet.

**Load-Balanced Gov Scout** — 8 federal APIs (SAM.gov, USASpending, SBIR, Federal Register, Grants.gov, Regulations.gov, CALC+, Contract Awards) queried with rate-limit rotation and sled dedup. No single API is a bottleneck. 713 results per scout run with typed schemas (Bid/Award/Signal/Rate) and zstd-compressed sled cache.

**NanoSign Model Integrity** — 36-byte BLAKE3 signature embedded in every AI model file. Verified at load time before inference. Unsigned or tampered models are rejected. Zero-infrastructure supply chain security for AI at the tactical edge.

### 3. Proof of Concept — Live Production Systems

All techniques are implemented, tested, and running:

- **Kova:** 314 tests, 96 source files, 16.9M param diffusion model, on-device LLM inference, agentic tool loop. Single binary.
- **cochranblock.org:** Production multi-domain web platform. 15MB binary, $10/month total infrastructure. 75fps render, 164ms first paint, zero layout shift.
- **whobelooking:** Federal contract intelligence system. 8 APIs, headless Chrome scraper, 451 opportunities scraped and scored. Single binary.
- **whyyoulying:** Labor fraud detection. 8 DoDI 5505.02 rules, GAGAS referral export, 67 unit + 12 integration tests. 621KB binary.
- **15 open-source repositories** publicly auditable at github.com/cochranblock

### 4. Technical Approach

**Phase 0 (current):** Working prototypes deployed across 15 repositories. Timeline of Invention with commit-level provenance for every technique.

**Phase I (12 months, $1M):**
1. Harden Kova agent loop for IL4/IL5 environments — FIPS 140-3 crypto module, CAC/PIV auth
2. Package C2 Swarm Mesh for multi-node SCIF deployment — nodes sync via sneakernet when air-gapped
3. Integrate NanoSign into DoD model supply chain — verify model provenance before any inference
4. Field trial with operational unit in DIL exercise environment

**Phase II (24 months, $3M):**
1. Cross-domain solution compatibility — CDS guard integration for model transfer between classification levels
2. Training curriculum for agency adoption
3. Transition plan for PEO/PM integration

### 5. Differentiators

- Every technique is named, benchmarked, and has commit-level provenance in a public Timeline of Invention
- All code is Unlicense (public domain) — zero procurement friction, zero vendor lock-in
- Memory-safe Rust — aligned with CISA Secure-by-Design mandate, EO 14028, NIST SP 800-218
- PI has 13 years defense/enterprise experience including USCYBERCOM J38 Congressional NDAA study

### 6. Contact

Michael Cochran, Owner & PI
The Cochran Block, LLC
mcochran@cochranblock.org | 443-900-7903
cochranblock.org/govdocs — Full capability statement
github.com/cochranblock — All source code
