<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized. Top = most important. Max 20. Tags: `[build]` `[test]` `[docs]` `[feature]` `[fix]` `[research]`

---

1. `[build]` CAGE code from DLA — blocks all 11 SBIR submissions and federal contracting. Nothing to do but wait. Check SAM.gov weekly.
2. `[feature]` Dynamic repo count from velocity cache — replace hardcoded `repo_count = 15` in f92/openbooks. Depends on: `/api/velocity` already returns live data, just wire it.
3. `[feature]` `/changelog` page — auto-generate from `git log`, show shipping velocity to visitors. No hardcoded entries. The guest analysis flagged this as missing.
4. `[feature]` `/support` page — SLA tiers, response times, bus-factor answer. Guest analysis: "haven't proven the business claim (trust you for 5 years)."
5. `[feature]` `/security` page — upgrade `/.well-known/security.txt` link target from `/about` to a dedicated security policy. Disclosure process, crypto inventory, NanoSign reference.
6. `[build]` Google Play Store listing for pixel-forge — store assets uploaded, AAB built (6.1 MB). Submit for review. Depends on: [pixel-forge](https://github.com/cochranblock/pixel-forge).
7. `[build]` IRONHIVE cluster online — start Ollama on lf/gd/st, re-establish SSH tunnels for ports 3003-3006. Fix NVIDIA driver on st. bt still needs HDMI fix. Depends on: [kova](https://github.com/cochranblock/kova) cluster commands.
8. `[feature]` `/source` page live metrics — tokei/cloc line counts, `ls -la target/release/` binary sizes, test counts computed at build time via `build.rs`. No hardcoded numbers that go stale. The site IS the proof of artifacts.
9. `[test]` Add any-gpu to HTTP test assertions — `products_anygpu_link` test verifying the product card renders and links to GitHub.
10. `[docs]` Security audit disclosure — no third-party audit exists. Either commission one or document the self-audit methodology. Guest analysis flagged this for government buyers.
11. `[feature]` Rogue Repo integration — add `/rogue-repo` route or subdomain via approuter. Depends on: [rogue-repo](https://github.com/cochranblock/rogue-repo), [approuter](https://github.com/cochranblock/approuter).
12. `[feature]` Ronin Sites integration — multi-tenant shop platform behind approuter. Depends on: [rogue-repo](https://github.com/cochranblock/rogue-repo) payment engine, [approuter](https://github.com/cochranblock/approuter).
13. `[build]` Pocket Server MVP — phone-as-web-server, Android deployment. Depends on: [approuter](https://github.com/cochranblock/approuter), [kova](https://github.com/cochranblock/kova).
14. `[build]` Ghost Fabric MVP — LoRa mesh edge nodes with embedded AI. Depends on: [kova](https://github.com/cochranblock/kova).
15. `[fix]` NVIDIA driver on st (kova-elite-support) — `nvidia-smi` fails. Needs driver reinstall or kernel module rebuild. Blocks IRONHIVE 4-node cluster.
16. `[fix]` bt (kova-thick-beast) HDMI — unreachable since HDMI issue. Needs physical access or serial console.
17. `[research]` SBIR Phase I proposals — 11 agencies prepped (DoD, NSF, DHS/CISA, NIST, NIH, NASA, DOE, USDA, EPA, DOT, NOAA). All blocked on CAGE. Prep submission packages now so they're ready day-one.
18. `[docs]` Compression map for cochranblock — `docs/compression_map.md` documenting f2/t0/C7 identifiers. Guest analysis: "token compression is unexplained, hostile to contributors."
19. `[test]` NanoSign assertions — add test that `/govdocs` page contains "NanoSign" and "BLAKE3". Add test that `/api/summary` returns `innovations` field.
20. `[research]` Multi-instance failover story — guest analysis: "one binary, one process, if it crashes the site is down." Document the approuter health-check + auto-restart architecture or build it.
