# RFI Response: Innovation Networks and Supply Chains

| | |
|---|---|
| Submitted by | Cochran Block (UEI W7X3HAQL9CF9, CAGE 1CQ66, SDVOSB pending, MD CSB approved) |
| Founder of Record | Michael Cochran |
| Re | SBA RFI on Supply Chain Gaps and Entrepreneur Assistance, FR Doc 2026-08553 |

## Introduction: A Worked Example, Not a Proposal

Cochran Block is a one-person, zero-cloud, public-domain software company. As of this filing it ships more than two dozen production Rust repositories, all under the Unlicense, all built using a documented human-piloted, AI-assisted workflow. The entire company runs on a single laptop and a $10/month tunnel. There is no AWS bill, no Kubernetes cluster, no DevOps team, and no offshore dependency in the build path.

This response treats Cochran Block as a worked example that bears directly on every question SBA has asked. Each section below cites named, public, auditable repositories rather than abstract claims. The founder's prior cleared-cyber experience (Two Six Technologies, 2022 to 2024, hands-on across the Joint Cyber Warfighting Architecture components JCC2, JCAP, and UDP) is the basis for the national-security framing. The repositories are the basis for everything else.

The argument across the five questions is consistent: the supply-chain gaps SBA cares about are real, they are technology gaps the United States can close, the gaps that small businesses are uniquely able to fill share a common shape (sovereign, low-overhead, vertically integrated single-binary tooling), the workforce model that produces this kind of software is already running, and the data needed to track it can be embedded in the artifact itself.

---

## Q1. Existing or Projected Supply Chain Gaps

### Q1a. Technologies Needed and Integration Challenges

Five concrete gaps, each with a corresponding Cochran Block artifact already addressing it:

1. **GPU compute sovereignty.** CUDA-only software locks the United States into a single foreign-fab vendor for AI inference. `any-gpu` is a Rust tensor engine over wgpu that runs on AMD, NVIDIA, Intel, and Apple silicon from one codebase. Integration challenge: WGSL compute shaders and kernel-fusion gaps versus hand-tuned CUDA. Tractable, not blocked.
2. **Local-first agentic inference.** Cloud-API dependence ties cleared workloads to commercial SaaS terms of service and foreign egress. `kova` is a local-first augment engine, dual-mode (local GGUF or Anthropic SSE), shipped as a single binary under 10 MB, designed for SSH-swarm orchestration on commodity hardware.
3. **Sovereign edge mesh comms.** No domestic, public-domain, sub-GHz cognitive mesh stack exists for cleared edge deployments. `ghost-fabric` is a Rust CLI plus Android client for LoRa 915 MHz mesh node management with subsystem traits for radio, mesh, inference, and sensor.
4. **Air-gap workflow automation.** n8n requires Node, which has no place in a cleared air-gap. `r8r` is a Rust port that builds to a single 8 MB static binary with 585 nodes registered and a code-generation pipeline that walks the upstream and emits Rust skeletons.
5. **Sovereign configuration management.** Ansible requires Python and YAML. `runsible` is a 1-for-1 Rust reimagining with TOML as native syntax and an automatic YAML-to-TOML converter for portability of existing playbooks.
6. **Sovereign web hosting.** Cloud-hosted static sites depend on a foreign or commercial CDN for availability. `pocket-server` is a Rust web server that runs on an Android handset (or any machine) and serves a live site with a kiosk dashboard reporting uptime, requests, bytes, and power draw. The hosting bill is the cost of a phone charger.
7. **Sovereign AI orchestration.** LangChain is a Python framework with a long transitive dependency tree. `tmux-harness` orchestrates a fleet of agentic Claude Code processes through tmux panes with no Python and no cloud, turning any commodity workstation into a multi-agent dev environment.
8. **Sovereign payment rail.** Card-present and card-not-present payments depend on a small set of foreign-influenced acquirers and processors. `rogue-repo` is a sovereign software-storefront and ISO 8583 payment engine in pure Rust, with AES-256-GCM PAN encryption, Argon2id auth, a working PostgreSQL ledger, and a TCP wire-protocol client ready for bank or switch connection. The federal need for a domestic, auditable, public-domain payment stack is unmet today.
9. **Sovereign on-device classification across architectures.** Cloud-AI moderation and screening services route sensitive audio and text through commercial endpoints. `call-shield` is an on-device call-screening classifier whose build matrix targets twelve architectures including RISC-V, IBM POWER (government mainframe relevance), seven ARM and x86 variants, plus Android, iOS, and offline-installable PWA. One classifier, every cleared platform.
10. **Per-entity micro-model swarms instead of foundation-model dependency.** Foundation-model APIs concentrate risk in a handful of vendors. `dndaimodel` is a Candle-based per-entity micro-model swarm with deterministic world tables, demonstrating that tiny purpose-trained models on commodity CPU produce useful, auditable output without a foundation-model API contract.

### Q1b. Anticipated Timeframe

All five artifacts above are at v0.1 to v0.6 today. Hardening to FY27 procurement readiness is a 12-to-18-month horizon under SBIR Phase II scale. The bottleneck is not invention. The bottleneck is that the team is one person.

### Q1c. Gaps Small Businesses Are Uniquely Positioned to Fill

The single-binary, zero-cloud, public-domain delivery model is a small-business-only product. A prime contractor cannot ship it. The cost structure that makes a prime competitive on a cost-plus contract makes it impossible to deliver software whose entire infrastructure footprint is $10/month. Cochran Block is shipping that delivery model today across more than two dozen repositories from a single laptop. This is not a thesis. It is operational.

A second class of small-business-only product is the SaaS-replacement local-first binary. Two examples in the Cochran Block portfolio: `free-payroll-system` (a public-domain payroll binary that replaces a per-employee SaaS subscription with a single Rust binary, no cloud, forever) and `atsisbroken` (a local-first replacement for the SaaS applicant-tracking-system intermediary that pipes job applications through a foreign cloud). In both cases the small business advantage is structural: the product cannot exist as a SaaS because there is nothing to charge a recurring fee for. Only a small business is willing to ship the cheaper, sovereign alternative.

The gap small businesses fill is the gap between what the government is allowed to buy under existing acquisition vehicles and what the technology actually permits. The federal acquisition system was designed when shipping software meant shipping an FTE army. It has not caught up to the world in which one founder plus AI can stand up a working capability in weeks. SBA's Innovation Network Programs are the correct vehicle to close that gap.

### Q1d. Workforce Challenges and Models That Work

The standard cleared-software workforce model (W-2 engineer with TS/SCI plus polygraph, billed at $300+ per hour through a prime) does not scale to the breadth of the supply chain gaps SBA is asking about. There are not enough cleared engineers, the pipeline is not growing fast enough, and the existing supply is already saturated by primes.

Cochran Block runs a different model and it produces auditable output. The model is one cleared founder using AI as a co-pilot with every commit attributed. The framework that documents this is `provenance-docs`, which embeds two artifacts in the repository: a Timeline of Invention (dated, commit-linked, with a mandatory AI-Role field on every entry) and a Proof of Artifacts (architecture, build metrics, screenshots, verification commands). The standalone `header-writer` binary injects the Unlicense header and contributor list pre- and post-AI, making correct attribution a five-second pre-commit step. A majority of the production repositories include a programmatic validator that runs the structural checks under a triple-execution gate. `tmux-harness` operationalizes the workforce model itself: a single founder runs a fleet of agentic Claude Code processes across tmux panes with project-level isolation. The model is real, it is shipping software, and it is cited here so that SBA can verify it independently.

This addresses Q1d directly. The proven curricula model that solves the cleared-cyber workforce gap is human-direction plus AI-generation plus commit-level provenance, with the provenance documented in a way that DFARS 252.227-7014 and EO 14028 auditors can actually read.

---

## Q2. Existing Businesses That Could Pivot to Fill Gaps

Four categories of pivot, each with public evidence:

1. **Open-source Rust shops can pivot into cleared-cyber tooling** if a provenance framework lets them prove what their AI assistants generated and what their humans directed. The `provenance-docs` repository specifies and demonstrates that framework. The pivot path is: existing Rust shop adopts the Timeline-of-Invention plus Proof-of-Artifacts pattern, runs the validator binary on their own repos, and arrives at SBIR Phase I eligibility for cleared work.

2. **Compliance and fraud-reduction services can pivot into deep-tech supply-chain assurance.** `ireducefraud` is a positioning of the same code-and-architecture skill set toward compliance counsel as the buyer. `whyyoulying` is a complementary product targeting DoD IG and FBI fraud investigators on labor-category fraud and ghost-billing patterns mapped to DoDI 5505.02 and 5505.03 and the Attorney General Guidelines. The same skill that builds a sovereign mesh stack also builds the audit harness around it.

3. **Civic and pro-se software can pivot into government-process tooling.** `illbethejudgeofthat` is a Google-Takeout-to-court-exhibit pipeline. The same evidence-pipeline pattern is directly applicable to FOIA processing, IG investigations, and OCR complaint workflows.

4. **SaaS-replacement local-first software can pivot into broad small-business uplift.** `free-payroll-system`, `atsisbroken`, and `pocket-server` each replace a recurring SaaS bill (payroll processor, applicant-tracking system, web-hosting subscription) with a one-time-build Rust binary. Any small business that already writes good local-first software can adopt this delivery pattern. The pivot is not invention. It is repositioning.

The general point: small businesses that already write good open-source software in memory-safe languages are one provenance framework and one set of acquisition-aware docs away from being SBIR-Phase-I-ready. SBA can subsidize the docs without subsidizing the software, which is the more leveraged use of FAST and GAFC funding.

---

## Q3. Highly Specialized Suppliers Across Multiple Industries

Three Cochran Block artifacts that are specialized and that already serve more than one industry:

- **`provenance-docs`.** Specialized: AI-attribution framework for software supply chains. Industries served: federal acquisition (CDRL mapping, SBIR phase plan), private-sector compliance (EO 14028 and NIST SP 800-218 mapping in the `govdocs/` directory), patent law (Thaler v. Vidal attribution gap), and open-source ecosystems (Unlicense, public domain). One artifact, four buyer types.
- **`forge-engine`.** Specialized: 2D-first Rust game engine with NanoSign provenance baked in from commit zero. Industries served: indie games, training simulators, defense visualization, and storefront-of-sovereign-software. The provenance hook (NanoSign, 36 bytes appended to any model file via BLAKE3 hashing) is itself reusable across every cleared model-distribution channel.
- **`kova`.** Specialized: local-first agentic engine. Industries served: cleared cyber operations, embedded edge, single-tenant enterprise, and offline research. One binary, four deployment surfaces.
- **`deglaze`.** Specialized: WebAssembly JS-glue budget auditor that maps every emitted JavaScript function to the WASM spec proposal that would eliminate it. Industries served: any small business shipping a Rust-to-WASM binary, federal acquirers requiring a software bill of materials for browser-delivered code, and standards-body advocacy. The supply-chain audit unit is the function, not the package.
- **`rogue-repo`.** Specialized: ISO 8583 payment engine plus a PWA software storefront. Industries served: domestic payment processing, software-distribution sovereignty (binaries delivered without an app-store gatekeeper), and indie-game commerce. One binary, three regulated markets.

If solved at SBIR Phase II scale, each of these artifacts becomes a domestic small-business alternative to a foreign or single-vendor incumbent. That is the supply-chain gap closure SBA is asking about.

---

## Q4. Commercialization Challenges and Curricula That Work

The commercialization barrier for nat-sec-relevant deep tech, observed firsthand, is not invention. It is the documentation tax. A cleared-cyber prime can absorb the cost of a Defense Acquisition Workforce-trained program manager, a contracts office, and a CDRL author. A solo founder cannot. The existing federal documentation expectations were written assuming the prime contractor cost structure.

The curricula and resource model that has been demonstrated to overcome this:

1. **Provenance-Docs Whitepaper (`provenance-docs/WHITEPAPER.md`).** Includes a CDRL mapping (which contract data requirements list items the framework satisfies) and an SBIR phase plan (Phase I scope, Phase II scope, Phase III commercialization path). It is public domain. It is reusable verbatim by any small business writing an OII application.
2. **Header-Writer Tooling (`header-writer`).** A pre-and-post-AI script that injects the Unlicense header and contributor list into every source file. Five seconds per repository, run from a Cargo command. The cost of correct attribution becomes negligible.
3. **Triple-Sims Quality Gate (`triple_sims_all_work_no_self_licking_ice_cream.plan.md`).** A one-page rule for declaring work done. Run the gate three times, fail on first deviation, no flake-tolerance. This kills the most common small-business commercialization failure: shipping software that almost works.
4. **Cochran Block Protocol Stack (`forge-engine/protocols/`).** A working in-repository archive of numbered engineering protocols (P12 AI Slop Eradication, P13 Compression Mapping, P15 Anti-Circular Dependencies, P26 Moonshot Frame, P27 Diamond Rust Binary, P28 Plasma Timing Discipline, P29 Dual-Laser Separation) demonstrating that a small-business software-engineering doctrine can be codified, archived, and inherited across repositories. This is the protocol equivalent of a CDRL library and is reusable verbatim by any GAFC awardee.
4. **Founder's Cleared-Cyber Experience.** The author's hands-on JCWA component experience (JCC2, JCAP, UDP) at Two Six Technologies during 2022 to 2024 grounds every design decision in the actual operational environment those programs serve. This kind of in-domain cleared experience is rare in the small-business pool and SBA should consider weighting it explicitly in OII award criteria.

The curricula model SBA should consider sponsoring is not yet another generic accelerator. It is a documentation-standard template plus a validator binary plus a one-week founder workshop, delivered through GAFC, that takes an existing public-domain Rust project and brings it to SBIR-Phase-I documentation parity. The marginal cost is low, the leverage on the existing open-source pipeline is high, and the work product is auditable.

---

## Q5. Quantitative and Qualitative Data Sources

The supply-chain tracking data SBA needs already exists, in three forms, and Cochran Block ships tooling for all three:

1. **In-repository provenance.** The Timeline of Invention is machine-readable, commit-linked, and contains an AI-Role field. The `f30` validator binary in `provenance-docs/src/` enforces 13 structural checks plus hash, date, and cross-document consistency. This is supply-chain-of-software at the unit of the commit. Every claim about who built what is verifiable against a hash.
2. **Distribution-time provenance.** NanoSign (described in `provenance-docs/WHITEPAPER.md` Section 4.4) appends 36 bytes to any model file, providing self-verifying integrity via BLAKE3. This is supply-chain-of-model at the unit of the file. Every model artifact a small business ships can be verified by its consumer.
3. **Demand-side intelligence.** `whobelooking` is a visitor-intelligence pipeline that ingests Cloudflare, access-log, or CSV data and emits a PDF report identifying which organizations are silently evaluating a given site. Applied at scale, this is a supply-chain-of-attention tracker: which large organizations and primes are quietly studying which small-business capabilities. It has already produced one verifiable case study (Microsoft evaluation across 14 IPs over 8 days), documented in the repository.
4. **Browser-delivered code provenance.** `deglaze` audits the WebAssembly JS-glue layer that ships in every Rust-to-WASM binary, categorizes each emitted function, and maps it to the WASM spec proposal that would eliminate it. This produces a function-level supply-chain bill of materials for any browser-delivered binary, suitable for federal acquirers under EO 14028 transparency requirements.

For SBA's purposes, items 1 and 2 are the low-hanging fruit. Both are public domain, both are running today, and both can be adopted by any FAST or GAFC awardee at zero licensing cost.

---

## Closing: Community, Partnerships, and Outcomes

Cochran Block is one founder, more than two dozen public-domain repositories, and a $10/month infrastructure footprint, with prior cleared-cyber operational experience on the JCWA component set. The community of economic activity is the Maryland and Mid-Atlantic cleared-cyber corridor, where the historic industries are defense systems integration and national-intelligence software services. The specific action needed to strengthen those industries is to expand the small-business pool that can credibly compete on cleared-software work without first becoming a prime subcontractor. Every claim in this response is independently verifiable at https://github.com/cochranblock and at https://cochranblock.org.

The partnerships needed within this innovation network are not formal teaming agreements. They are documentation and validation pipelines: a GAFC-funded provenance reviewer, a FAST-funded SBIR proposal-writing pairing for AI-piloted Rust shops, and a RIC-supported regional clearance-and-acquisition liaison who connects small-business builders to cleared end customers without primes as the only path. The anticipated outcomes are measurable: the number of small-business Rust binaries that pass SBIR Phase I documentation review, the number of cleared end customers that adopt a small-business binary as a sovereign alternative to a prime-delivered SaaS, and the dollar value of recurring SaaS spend retired by sovereign single-binary replacement. Capital follows. Jobs follow. The order matters.

If SBA's Innovation Network Programs intend to fund the small-business activities best positioned to close national-competitiveness supply-chain gaps, the most leveraged use of FY26 funding is to underwrite the documentation, attribution, and provenance overhead that currently locks small Rust-and-AI shops out of SBIR participation. The technology is already being built. The artifact this RFI is asking about already exists. The remaining work is to make it acquisition-legible at scale.

Respectfully submitted, **Michael Cochran**, Founder, Cochran Block. mcochran@cochranblock.org. UEI W7X3HAQL9CF9, CAGE 1CQ66, EIN 41-3835237. SDVOSB (pending), MD CSB (approved).
