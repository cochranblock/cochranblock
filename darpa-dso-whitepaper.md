# DARPA DSO White Paper
## HR001125S0013 — Defense Sciences Office, Office-Wide BAA

**Title:** Zero-Infrastructure Edge Intelligence: Compiled AI for Contested Environments

**Proposer:** The Cochran Block, LLC
**PI:** Michael Cochran — Army 17C (Cyber Operations), JCAC 2014, USCYBERCOM J38 JMOC-E dev lead
**CAGE:** 1CQ66 | **UEI:** W7X3HAQL9CF9 | **NAICS:** 541511, 541512

---

### 1. Problem Statement

Forward-deployed and edge computing environments cannot depend on cloud infrastructure, container orchestration, or network connectivity for AI-enabled operations. Current solutions require Docker, Kubernetes, cloud GPUs, and continuous internet — none of which exist at the tactical edge. The fundamental assumption that AI requires infrastructure is wrong.

### 2. Technical Innovation

We have built a complete AI inference and training pipeline that compiles into single binaries with zero external dependencies. No containers. No cloud. No package managers. Copy one file, run it, the mission continues.

**Named techniques developed under this effort:**

**Fish Tank Starfield** — A GPU-zero-cost rendering technique that achieves 75fps animated backgrounds using 1/4 the GPU memory of standard approaches. Static mask layer with background-position loop replaces oversized transform animations. Proven to eliminate Android GPU crashes on low-end devices. Origin: Arizona garage sale fish tank decoration, circa 2006-2010.

**Any-GPU Tensor Engine** — Vendor-agnostic GPU compute via WGSL shaders. One codebase runs on AMD (RX 5700 XT), NVIDIA (RTX 3070/3050 Ti), Intel, and Apple (M4). No CUDA dependency. 19 tensor operations, 62 cross-validated tests across 4 GPU architectures. Benchmarks: AMD 5.67ms, NVIDIA 3.03ms, Apple 3.36ms (512x512 matmul).

**Tiered MoE Cascade** — Three diffusion models at increasing fidelity (Cinder 1.09M params → Quench 5.83M → Anvil 16.9M) with Mixture-of-Experts routing. Lower tiers run on any hardware; upper tiers activate only when GPU capacity allows. Trained entirely on consumer GPUs ($0 cloud cost).

**Triple Sims** — Deterministic test validation: run the full test suite 3 times, all 3 must produce identical results. Eliminates flaky tests, race conditions, and non-deterministic behavior. If it passes once but fails twice, it's not reliable — it's lucky. Deployed across 6+ production projects.

**RSSI/Battery/Hop Mesh Routing (Ghost Fabric)** — LoRa 915MHz mesh network with multi-factor node scoring: signal strength (RSSI), remaining battery, contact freshness, and hop count. Self-healing topology — nodes route around failures automatically. Trait-based architecture allows hardware abstraction (mock for testing, real radio for deployment).

### 3. Proof of Concept

- **Pixel Forge:** Pure-Rust AI sprite generator. 3 diffusion models trained from scratch on consumer GPUs. NanoSign model integrity (BLAKE3). 77 tests. Running on lf node (RTX 3070).
- **any-gpu:** 19 GPU operations as WGSL compute shaders. Tested on 4 GPU architectures. 62 tests all passing. Public at github.com/cochranblock/any-gpu.
- **Ghost Fabric:** LoRa mesh node management with RSSI/battery/hop scoring. 35 tests. 459KB CLI binary. Android app scaffold (egui + NativeActivity).
- **exopack:** Triple Sims validation framework used across 6+ projects. Standards gate: 14-point Rust industry check.
- **cochranblock.org:** Fish Tank Starfield live in production. 75fps, 0.0000 CLS, 164ms first paint. Benchmarked via `whobelooking perf`.

### 4. Technical Approach

**Phase I (6 months, $500K):**
1. Integrate any-gpu autograd — backward ops for conv2d, group_norm, attention, AdamW optimizer. Enables GPU training on heterogeneous hardware fleet
2. Deploy Ghost Fabric mesh protocol on physical LoRa hardware (Heltec ESP32 + SX1276)
3. Demonstrate Pixel Forge training across mixed GPU fleet (AMD + NVIDIA in same training run via any-gpu)
4. Fish Tank rendering technique documented as reusable CSS pattern library for DoD web applications

**Phase II (18 months, $2M):**
1. Field trial: Ghost Fabric mesh deployed in DIL exercise environment with real LoRa radios
2. any-gpu training at scale: distributed training across 4+ heterogeneous nodes
3. FIPS 140-3 integration for all crypto primitives (currently AES-256-GCM, HKDF, Argon2id)
4. Transition plan: single-binary deployment kit for PEO adoption

### 5. Revolutionary vs. Evolutionary

Per the BAA requirement — this is not evolutionary:
- **No existing framework** compiles a full AI inference + training + mesh networking stack into a single binary
- **No existing system** trains diffusion models on AMD + NVIDIA + Apple GPUs from one codebase without CUDA
- **No known CSS technique** achieves animated starfield effects at 1/4 GPU memory via static mask + background-position loop
- **No test framework** requires triple-deterministic validation as a CI gate

Each technique has a named entry in a public Timeline of Invention with commit-level provenance.

### 6. Contact

Michael Cochran, Owner & PI
The Cochran Block, LLC
mcochran@cochranblock.org | 443-900-7903
cochranblock.org/govdocs — Full capability statement
github.com/cochranblock — All source code
