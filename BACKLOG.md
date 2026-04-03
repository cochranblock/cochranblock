<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized. Top = most important. Max 20. Tags: `[build]` `[test]` `[docs]` `[feature]` `[fix]` `[research]`

---

1. `[build]` CAGE code from DLA — blocks all 11 SBIR submissions and federal contracting. Nothing to do but wait. Check SAM.gov weekly.
2. `[feature]` `/support` page — SLA tiers, response times, bus-factor answer. Guest analysis: "haven't proven the business claim (trust you for 5 years)."
3. `[feature]` `/security` page — upgrade `/.well-known/security.txt` link target from `/about` to a dedicated security policy. Disclosure process, crypto inventory, NanoSign reference.
4. `[build]` Google Play Store listing for pixel-forge — store assets uploaded, AAB built (6.1 MB). Submit for review. Depends on: [pixel-forge](https://github.com/cochranblock/pixel-forge).
5. `[build]` IRONHIVE cluster online — start Ollama on lf/gd/st, re-establish SSH tunnels for ports 3003-3006. Fix NVIDIA driver on st. bt still needs HDMI fix. Depends on: [kova](https://github.com/cochranblock/kova) cluster commands.
6. `[feature]` `/source` page live metrics — tokei/cloc line counts, `ls -la target/release/` binary sizes, test counts computed at build time via `build.rs`. No hardcoded numbers that go stale. The site IS the proof of artifacts.
7. `[test]` Add any-gpu to HTTP test assertions — `products_anygpu_link` test verifying the product card renders and links to GitHub.
8. `[docs]` Security audit disclosure — no third-party audit exists. Either commission one or document the self-audit methodology. Guest analysis flagged this for government buyers.
9. `[feature]` Rogue Repo integration — add `/rogue-repo` route or subdomain via approuter. Depends on: [rogue-repo](https://github.com/cochranblock/rogue-repo), [approuter](https://github.com/cochranblock/approuter).
10. `[feature]` Ronin Sites integration — multi-tenant shop platform behind approuter. Depends on: [rogue-repo](https://github.com/cochranblock/rogue-repo) payment engine, [approuter](https://github.com/cochranblock/approuter).
11. `[build]` Pocket Server MVP — phone-as-web-server, Android deployment. Depends on: [approuter](https://github.com/cochranblock/approuter), [kova](https://github.com/cochranblock/kova).
12. `[build]` Ghost Fabric MVP — LoRa mesh edge nodes with embedded AI. Depends on: [kova](https://github.com/cochranblock/kova).
13. `[fix]` NVIDIA driver on st (kova-elite-support) — `nvidia-smi` fails. Needs driver reinstall or kernel module rebuild. Blocks IRONHIVE 4-node cluster.
14. `[fix]` bt (kova-thick-beast) HDMI — unreachable since HDMI issue. Needs physical access or serial console.
15. `[research]` SBIR Phase I proposals — 11 agencies prepped (DoD, NSF, DHS/CISA, NIST, NIH, NASA, DOE, USDA, EPA, DOT, NOAA). All blocked on CAGE. Prep submission packages now so they're ready day-one.
16. `[docs]` Compression map for cochranblock — `docs/compression_map.md` documenting f2/t0/C7 identifiers. Guest analysis: "token compression is unexplained, hostile to contributors."
17. `[test]` NanoSign assertions — add test that `/govdocs` page contains "NanoSign" and "BLAKE3". Add test that `/api/summary` returns `innovations` field.
18. `[research]` Multi-instance failover story — guest analysis: "one binary, one process, if it crashes the site is down." Document the approuter health-check + auto-restart architecture or build it.
19. `[fix]` `/api/stats` hardcoded numbers — `repos` and `binary_size_arm` in f73 are literals. Wire to REPOS.len() and embed actual binary size at build time.
20. `[fix]` `/api/summary` stale counts — `total_tests`, `total_rust_loc`, `total_rs_files` in f89 are hardcoded. Either compute at build time via build.rs or remove them.
