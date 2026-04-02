# VR&E Self-Employment Business Plan — Executive Summary & Training Justification

**Veteran:** Michael Cochran
**Entity:** The Cochran Block, LLC
**Disability Rating:** 30% Service-Connected
**Post-9/11 GI Bill:** 23 months remaining (electing P9/11SA for Baltimore BAH)
**Maryland CSB:** Approved
**SDVOSB:** Pending
**SAM.gov:** Registered
**Maryland eMMA:** SUP1095449

---

## Executive Summary

The Cochran Block, LLC is a Maryland-based Certified Small Business (CSB) specializing in memory-safe software architecture for federal agencies. The company develops and maintains **Exopack**, an open-source Rust-based CI/CD toolchain that reduces government software deployment sizes by up to 99.5% while enforcing the memory-safety mandates established by CISA Directive 23-01, Executive Order 14028, and the 2026 DoD Secure-by-Design requirements.

**The Business Model:** Cochran Block releases all software into the Public Domain under The Unlicense. This zero-procurement-friction model eliminates ITAR/EAR licensing concerns, sole-source justification requirements, and vendor lock-in — the three primary barriers to federal software adoption. Revenue is generated exclusively through professional services: implementation consulting ($225/hour), air-gapped DevSecOps architecture, and environment-specific hardening for DoD/IC/federal deployments.

**Commercial Viability Indicators:**
- 16 active public repositories on GitHub, all Unlicense
- Continuous development verified by automated 30-minute commit tracking dashboard
- First paying client (oakilydokily.com) under active contract
- Production infrastructure: 4 worker nodes, GPU compute (RTX 3070 8GB, RTX 3050 Ti 4GB), automated deployment pipeline
- Binary sizes from 48 KB to 51.5 MB across product line — every claim verifiable from public source code
- Existing web presence generating organic traffic at cochranblock.org (single 8.2 MB Rust binary, $10/month infrastructure cost)

**Federal Market Alignment:**
- CISA "Secure by Design" pledge (2024-2026): mandates memory-safe languages for all new federal software
- EO 14028 SBOM requirements: Cochran Block ships machine-readable Software Bills of Materials with every release
- NIST SP 800-218 (SSDF): development practices mapped to all four task areas (PS, PW, RV, PO)
- CMMC Level 1-2: compliance documentation maintained per-project in standardized `govdocs/` directories
- FedRAMP: on-premises deployment model eliminates cloud authorization boundary complexity

---

## Services & Training Plan

### Current Service Offerings

| Service | Rate | Description |
|---------|------|-------------|
| Exopack Implementation | $225/hr | Deploy memory-safe CI/CD pipeline in federal environment |
| Air-Gapped Architecture | $225/hr | Configure zero-cloud Rust deployments for SCIF/disconnected networks |
| Binary Hardening | $225/hr | Optimize release binaries (LTO, strip, size profiling) for constrained environments |
| Fractional CTO | $3,500/mo | Ongoing architecture leadership for federal software modernization |
| Zero-Cloud Deployment | $3,500 one-time | Replace cloud infrastructure with single-binary Rust architecture |

### Why Lab-Based Workforce Training (Category II) Is Required

The Cochran Block's federal market entry requires **validated performance data** that can only be generated in enterprise-grade computing environments. Specifically:

**1. AI/ML Model Validation at Scale**
Exopack's binary optimization pipeline includes AI model quantization (f32 to f16), on-device inference via custom Mixture-of-Experts architectures, and memory-safe tensor operations via the Candle framework. Federal procurement officers require performance benchmarks generated on hardware comparable to their deployment targets — not consumer laptops. University lab environments (e.g., UMBC Center for Applied AI, Johns Hopkins APL) provide:
- Multi-GPU clusters for parallel stress testing
- Standardized benchmarking infrastructure recognized by federal evaluators
- Published performance results that constitute "past performance by proxy" for SAM.gov proposals

**2. FIPS 140-3 Crypto Validation Environment**
Federal deployments require FIPS-validated cryptographic modules. The transition from development-grade crypto (ring/RustCrypto) to FIPS-certified implementations requires testing against the NIST Cryptographic Algorithm Validation Program (CAVP) test vectors on controlled lab equipment. University cryptography labs provide the validated testing environments and expert guidance necessary for this certification path.

**3. Air-Gapped/Edge Computing Stress Testing**
Government edge deployments (tactical, shipboard, SCIF) operate on resource-constrained hardware without internet connectivity. Cochran Block's zero-cloud architecture is purpose-built for these environments, but validation requires:
- Isolated network segments simulating air-gapped conditions
- ARM/RISC-V development boards matching DoD edge compute targets
- Sustained load testing (72+ hour runs) that exceed home-office power and cooling capacity

### Training Institution Requirements

The requested lab-based workforce training should provide:

| Requirement | Justification |
|-------------|---------------|
| Enterprise GPU cluster access | AI model validation at federal deployment scale |
| Isolated network lab | Air-gapped deployment testing and STIG compliance verification |
| FIPS testing environment | Cryptographic module validation pathway |
| Faculty advisor with federal/DoD experience | Technical mentorship and academic validation |
| Published benchmarking capability | Generate citable performance data for SAM.gov proposals |
| Certificate or credential upon completion | Tangible qualification for federal RFP technical volumes |

### Candidate Institutions (Baltimore Region)

1. **UMBC Center for Applied AI** — GPU clusters, AI research focus, DoD-adjacent faculty
2. **Johns Hopkins Applied Physics Laboratory** — Federal research heritage, security clearance infrastructure
3. **University of Maryland, College Park — Maryland Cybersecurity Center (MC2)** — NIST partnership, FIPS expertise

### Measurable Outcomes (12-Month Training Period)

| Milestone | Timeline | Deliverable |
|-----------|----------|-------------|
| Lab environment access established | Month 1 | Signed MOU with institution |
| Exopack AI benchmarks published | Month 3 | Citable performance report |
| FIPS crypto validation path documented | Month 6 | CAVP test vector results |
| Air-gapped deployment validated | Month 6 | Edge computing reference architecture |
| First SBIR/STTR proposal submitted | Month 9 | Technical volume with lab-validated data |
| First federal contract awarded or in negotiation | Month 12 | SAM.gov contract action |

### Proof of Discipline and Readiness

The Veteran maintains a custom-built dashboard that tracks and verifies continuous Git commits at 30-minute intervals across all 16 active repositories. This system provides:
- **Objective evidence of sustained productivity** — not self-reported, machine-verified
- **Public audit trail** — every commit timestamped, hashed, and published to GitHub
- **Proof of Artifacts documentation** — each repository maintains a `PROOF_OF_ARTIFACTS.md` with verifiable build metrics, test results, and binary sizes
- **Timeline of Invention** — each repository maintains a `TIMELINE_OF_INVENTION.md` with dated, hash-verified commit history establishing intellectual property provenance

This level of documentation rigor exceeds standard industry practice and demonstrates the discipline, consistency, and technical capability required for successful self-employment under VR&E Category II.

---

**Prepared for:** VA Vocational Rehabilitation & Employment Counselor, Baltimore Regional Office
**Date:** March 2026
**Contact:** mcochran@cochranblock.org | cochranblock.org
