# RFI Response: Strengthening Domestic Supply Chains and Critical Supplier Competition

| | |
|---|---|
| Submitted by | Cochran Block (UEI W7X3HAQL9CF9, CAGE 1CQ66, SDVOSB pending, MD CSB approved) |
| Founder of Record | Michael Cochran |
| Re | SBA RFI on Scaling Critical Suppliers in Domestic Supply Chains, FR Doc 2026-08554 |
| Companion | A separate response to the companion RFI 2026-08553 (Innovation Networks and Supply Chains) is submitted under the same cover. |

## Introduction: Supplier-Scaling Infrastructure, Not the Supplier

Cochran Block is not a domestic manufacturer of physical goods. It is a one-person, public-domain Rust software shop with prior cleared-cyber operational experience on the Joint Cyber Warfighting Architecture component set (JCC2, JCAP, UDP) at Two Six Technologies during 2022 to 2024. As of this filing it ships more than two dozen production Unlicense Rust repositories on a $10/month infrastructure footprint.

This response therefore reframes the RFI's premise. The question SBA asks is "what existing domestic small-business suppliers can scale production capacity in 1-3 months given a capital infusion." The premise of this response is that **the bottleneck for most cleared and acquisition-relevant small-business suppliers is not production capacity, it is documentation, attribution, and execution-risk surface area.** Cochran Block builds the supplier-scaling infrastructure that compresses those bottlenecks, and that infrastructure can be deployed to dozens of existing domestic small businesses inside the 1-to-3-month execution window SBA has scoped. Where direct supplier examples are useful, the response cites Cochran Block's own portfolio. Where the more leveraged answer is tooling for other suppliers, it cites that.

Every claim is independently verifiable at https://github.com/cochranblock and https://cochranblock.org.

---

## Q1. Supply Chain Gaps and Choke Points

The supply-chain choke point most amenable to a 1-to-3-month intervention by SBA is not a physical good. It is the documentation-and-attribution layer that determines whether a domestic small-business supplier can sell into a cleared or federal-acquisition customer at all. Five concrete physical-and-digital gaps follow, with sector and quantification.

1. **Cross-vendor GPU compute supply** (defense, dual-use, AI research). CUDA-only software locks AI workloads into a single foreign-fab vendor. `any-gpu` is a Rust tensor engine over wgpu running on AMD, NVIDIA, Intel, and Apple silicon from one codebase. Constraint shape: software-portability gap, not capacity gap. Near-term milestone tractable: yes, hardening to Phase II in 1-3 months on an SBIR-Phase-I-equivalent capital infusion.
2. **Sovereign agentic-inference runtime** (cleared cyber, embedded edge). Cloud-API dependence ties cleared workloads to commercial SaaS terms of service. `kova` is a local-first augment engine, single binary under 10 MB, dual-mode local-or-remote, designed for SSH-swarm orchestration on commodity hardware. Constraint: documentation surface, not capability. Near-term milestone tractable: 1-3 months to FedRAMP-Low-equivalent attestation pack.
3. **Sub-GHz cognitive mesh comms for sovereign edge** (defense, critical infrastructure IoT, emergency response). No domestic, public-domain, sub-GHz cognitive mesh stack exists for cleared edge deployments. `ghost-fabric` is a Rust CLI plus Android client for LoRa 915 MHz mesh node management. Constraint: hardware-driver implementation gap. Near-term milestone tractable: 1-3 months to first hardware-driver integration on a named LoRa SoC, given a $50k-$150k component-and-engineering capital infusion.
4. **Air-gap workflow automation** (cleared cyber, classified facility ops). n8n requires Node, which has no place in an air-gap. `r8r` is a Rust port shipping as one 8 MB static binary with 585 nodes registered. Constraint: integration testing against existing classified workflows. Near-term milestone tractable: 1-3 months to a ten-node classified-pilot integration.
5. **Sovereign domestic payment rail** (commercial, dual-use). Card-present and card-not-present payments depend on a small set of foreign-influenced processors. `rogue-repo` is a sovereign software storefront and ISO 8583 payment engine in pure Rust with AES-256-GCM PAN encryption, Argon2id auth, a working PostgreSQL ledger, and a TCP wire-protocol client ready for bank-or-switch connection. Constraint: bank-side acquirer onboarding, which is a relationship gap rather than a code gap.

Each of the five is a single-point-of-failure or highly-concentrated-supply choke point that a U.S. small business can effectively address. Each is small enough that a single founder has already brought it to v0.1 and it is the next milestone, not the first one, that needs SBA help.

---

## Q2. Supplier Readiness to Scale

Three readiness tiers in the Cochran Block portfolio, presented with the surge-capacity question SBA asked:

- **Production-Ready (today).** `provenance-docs` (the documentation framework cited throughout this response), `header-writer` (post-AI Unlicense-and-attribution injector), `tmux-harness` (multi-agent tmux orchestration replacing LangChain), `kova` (local-first augment engine), `any-gpu` (cross-vendor tensor engine), `runsible` (Rust Ansible reimagining). All compile, all run, all documented at the Timeline-of-Invention plus Proof-of-Artifacts level. Surge capacity: each binary is a one-command install from a public Cargo registry; deployment time is minutes, not weeks.
- **Pilot-Ready (1-3 months).** `r8r` (Rust n8n for air-gap), `ghost-fabric` (sub-GHz mesh), `rogue-repo` (ISO 8583 payment), `forge-engine` (2D Rust game engine with NanoSign provenance), `whyyoulying` (DoD IG / FBI labor-category-fraud and ghost-billing detection per DoDI 5505.02 and 5505.03), `ireducefraud` (cyber security architect / fraud reduction for compliance counsel). Each requires one named-customer pilot to harden, which is an inside-the-RFI-window milestone.
- **Demand-Activated (3-6 months).** `pocket-server` (web server on a phone), `atsisbroken` (local-first ATS replacement for SaaS gatekeepers), `free-payroll-system` (single-binary payroll replacing per-employee SaaS), `whobelooking` (visitor-intelligence pipeline). Each is product-market-fit pending a named customer signing a memorandum of understanding.

Customer-demand evidence: `whobelooking` produced a verifiable case study of Microsoft evaluating cochranblock.org across 14 IPs over 8 days; the case study and its provenance are documented in the repository. This is the closest thing a one-founder shop can produce to a near-term contract pipeline before SBIR-funding-level diligence work begins.

---

## Q3. Constraints to Near-Term Scaling

The constraints SBA enumerated apply asymmetrically to a one-founder cleared-cyber Rust shop. In rough order of severity:

1. **Certification and qualification requirements for both commercial and government contracts.** SDVOSB application is pending at the VA. MD CSB is approved. CMMC Level 2 certification is the binding constraint for cleared-cyber pilot sales and is not currently accessible to a one-founder shop without a costly external assessor. SBA could measurably unblock dozens of small businesses by funding an OII-coordinated CMMC-readiness reviewer pool.
2. **Capital requirements for equipment, materials, and working capital.** For a software-only supplier the equipment line is small (development workstations, cleared-network access). Working capital is the bottleneck, specifically the cash runway needed to bridge the documentation phase between an awarded SBIR Phase I and the start of recurring contract revenue.
3. **Workforce availability and training.** The cleared-cyber engineer pool is saturated by primes. The Cochran Block model (one cleared founder plus AI co-pilot with full provenance attribution) is a workforce-multiplier model that does not require headcount growth. SBA can accelerate adoption of this model by funding a regional clearance-and-acquisition liaison through the RIC program.
4. **Demand uncertainty.** OII and the cleared-cyber acquisition base communicate demand through formal solicitations only. Small businesses have no signal on what is being evaluated. `whobelooking` partially closes this gap on the visitor-intelligence side. A demand-signal program inside OII, even at the level of a quarterly bulletin of priority capability gaps, would reduce uncertainty meaningfully.
5. **Infrastructure or equipment limitations.** Not binding for a software-only supplier. The Cochran Block deployment infrastructure is a $10/month Cloudflare tunnel and a single laptop.
6. **Price fluctuations and availability of upstream inputs.** Not binding for a public-domain Rust shop with stdlib-and-Tokio-class dependencies and zero foreign-binary inputs.
7. **Other risks adversely affecting near-term execution.** The single largest other risk for a one-founder shop is founder availability under legal and personal load. Mitigating this requires an SBA program that recognizes the bus-factor reality of a true SDVOSB micro-business and accepts a deliberate redundancy plan rather than penalizing it.

---

## Q4. Effective Interventions for Rapid Scaling

The interventions most likely to deliver near-term scaling for cleared-cyber and acquisition-relevant small-business suppliers, in order of leverage:

- **Financial support mechanisms deployable quickly.** A milestone-based micro-grant program (target $25k-$75k per supplier, 30-to-60-day disbursement) tied to a single named near-term outcome. Outcome examples: a CMMC-Level-2-readiness assessment package, a published SBIR Phase I Technical Volume, a first cleared-pilot integration. Existing FAST and GAFC funding can be repurposed at this granularity.
- **Demand commitments or procurement alignment.** OII publishing a quarterly priority-capability bulletin (even an unranked list of named gap areas) would let small businesses align effort to demand. The cost is one program analyst's time per quarter.
- **Technical assistance or validation support.** A GAFC-funded reviewer pool that performs a five-business-day documentation review on any small-business Rust binary, returning a punch list against the `provenance-docs` framework's CDRL mapping. This is the highest-leverage intervention named in this response. Cochran Block volunteers to seed the reviewer-pool curriculum from the existing `provenance-docs/WHITEPAPER.md` plus the Cochran Block Protocol Stack archive (P12 through P29 in `forge-engine/protocols/`).
- **Innovations and solutions facilitating near-term growth and efficiencies.** `header-writer`, `tmux-harness`, and the TRIPLE SIMS quality-gate plan are the three Cochran Block tools that most directly compress the documentation-and-validation cycle for any small-business Rust supplier. All three are public domain and available for adoption today at zero licensing cost.
- **Partnerships or coordinated actions.** Three asks: a GAFC-funded provenance reviewer (above), a FAST-funded SBIR proposal-writing pairing for AI-piloted Rust shops, and a RIC-supported regional clearance-and-acquisition liaison.
- **Role of service providers or intermediaries.** The most useful intermediary is not a primecontractor; it is a documentation-validation service that reduces execution risk on behalf of the small-business supplier. `provenance-docs` is an open-source instantiation of that service.

---

## Q5. Investment, Milestones, and Expected Outcomes

Capital request, framed for a one-founder shop with a real cost structure:

- **Estimated funding required for Cochran Block to deliver three named near-term milestones inside 1-3 months.** $150,000. Milestones: (1) `provenance-docs` v1.0 stable release plus an SBIR-Phase-I-aligned documentation review of three peer Rust shops' repositories; (2) `kova` plus `any-gpu` integrated into a cleared-pilot inference scenario with a named cleared customer; (3) `r8r` first ten-node air-gap pilot. Cost stack: founder time at full-time-equivalent for the three months ($90k), one external CMMC-readiness assessor engagement ($30k), workstation and cleared-network access ($15k), publication and travel for one OII engagement plus one cleared-customer demonstration ($15k).
- **Expected non-federal cost share or private investment.** Cochran Block is bootstrapped and has no outside investors. Private cost share is the founder's pre-existing personal investment in the equipment, the public-domain repositories, and the deployment infrastructure, conservatively valued at $200k of accumulated work-product and $50k of equipment and infrastructure. This counts as in-kind cost share against the $150k capital request.
- **Specific milestones achievable within 1-3 months.** The three above. Each is independently verifiable at https://github.com/cochranblock by checking the named repository's Timeline of Invention.
- **Anticipated increases in output and throughput.** Output is documentation throughput, not unit production. Anticipated increase: the documentation-review service throughput moves from one shop (Cochran Block itself) to four shops (Cochran Block plus three peer reviewees). This is a 4x increase inside the RFI execution window.
- **Expected improvements in cost, lead time, or supplier participation.** Cost: per-shop SBIR Phase I documentation cost falls from market rate (estimated $30-$60k per shop in consulting fees) to a few founder-hours of self-service against the published framework. Lead time: from the typical six-to-nine-month proposal-development window to a six-to-ten-week window once the framework and reviewer pool are in place. Supplier participation: every public-domain Rust shop currently locked out of SBIR by documentation overhead becomes eligible.

---

## Q6. Data and Measurement

The metrics SBA needs are already producible by Cochran Block tooling, and the framework for capturing them is open and public-domain:

- **Methods for estimating current and near-term production capacity growth.** For software suppliers, capacity equals throughput of validated binaries. The `f30` validator binary (in `provenance-docs/src/`) reports a per-repository structural-completeness score across 13 checks plus hash, date, and cross-document validation. Aggregated across an SBA-funded reviewer-pool cohort, this score is a leading indicator of how many shops will pass an SBIR Phase I documentation review.
- **Metrics for tracking short-term output increases.** Number of repositories passing the `f30` gate; number of SBIR Phase I Technical Volumes published; number of cleared-pilot integrations completed. All three are countable, dated, and auditable against the public Timeline-of-Invention record.
- **Indicators of reduced lead times or cost performance.** Median elapsed time from "SBIR-Phase-I-eligible artifact exists" to "SBIR-Phase-I documentation review passed." Without the framework: six to nine months. With the framework: target six to ten weeks. Self-reported by reviewees, audited against the SBA reviewer-pool log.
- **Metrics for measuring a specific supply chain's resilience or vulnerability.** Number of single-vendor dependencies eliminated per supplier (e.g., CUDA-only-to-cross-vendor moves, cloud-API-to-local moves, Node-to-Rust moves, Python-to-Rust moves). Each is a discrete, auditable substitution.
- **Metrics or indicators relating to improving national or economic security.** For each cleared-pilot integration: a domestic-only supply-chain bill of materials (SBOM) verified against EO 14028 and NIST SP 800-218 mappings already published in `provenance-docs/govdocs/`. The metric is the percentage of SBOM components with a U.S. principal place of business.

---

## Closing: Execution Posture and Capital Stack

Cochran Block is not asking SBA for a manufacturing-scaling capital infusion. It is asking SBA to underwrite the documentation and reviewer infrastructure that lets a fleet of existing public-domain Rust shops, including Cochran Block, become acquisition-eligible inside the 1-to-3-month window the RFI describes. The capital stack is small, the deliverables are countable, and the leverage on the broader open-source small-business ecosystem is high. The artifact this RFI is asking about already exists. The remaining work is to fund the reviewer side of the pipe.

Respectfully submitted, Michael Cochran, Founder, Cochran Block. mcochran@cochranblock.org. UEI W7X3HAQL9CF9, CAGE 1CQ66, EIN 41-3835237. SDVOSB (pending), MD CSB (approved).
