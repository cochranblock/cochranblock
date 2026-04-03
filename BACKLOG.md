<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized. Top = most important. Max 20. Tags: `[build]` `[test]` `[docs]` `[feature]` `[fix]` `[research]`

---

1. `[fix]` `/analytics` zero-data fallback — when CF_TOKEN is absent or Cloudflare returns nothing, the page renders an empty table (0 requests, 0 visitors). A gov buyer sees a dead site. Add a fallback message: "Live data requires Cloudflare integration — contact us for a demo." One conditional in f90, one test assertion.
2. `[fix]` `/api/summary` and `/api/stats` stale hardcoded counts — `total_tests`, `total_rust_loc`, `total_rs_files` in f89 and `repos`/`binary_size_arm` in f73 are wrong right now. AI crawlers index these. Build.rs can count tests via `cargo test --list` output and embed the real numbers. The "code IS the resume" claim is violated by the resume itself.
3. `[fix]` Rate-limit the intake and community-grant POST endpoints — no-auth POST to SQLite with no throttle is a DoS surface. In-process per-IP rate limiter: DashMap<IpAddr, Vec<Instant>>, max 3 submits/hr/IP, 429 with Retry-After header. No new dependency needed. Protects the only mutable data store on the server.
4. `[build]` CAGE code from DLA — blocks all 11 SBIR submissions and federal contracting. Nothing to do but wait. Check SAM.gov weekly.
5. `[feature]` `/support` page — SLA tiers, response times, bus-factor answer. Guest analysis: "haven't proven the business claim (trust you for 5 years)."
6. `[feature]` `/security` page — upgrade `/.well-known/security.txt` link target from `/about` to a dedicated security policy. Disclosure process, crypto inventory, NanoSign reference.
7. `[build]` Google Play Store listing for pixel-forge — store assets uploaded, AAB built (6.1 MB). Submit for review. Depends on: [pixel-forge](https://github.com/cochranblock/pixel-forge).
8. `[build]` IRONHIVE cluster online — start Ollama on lf/gd/st, re-establish SSH tunnels for ports 3003-3006. Fix NVIDIA driver on st. bt still needs HDMI fix. Depends on: [kova](https://github.com/cochranblock/kova) cluster commands.
9. `[feature]` `/source` page live metrics — tokei/cloc line counts, `ls -la target/release/` binary sizes, test counts computed at build time via `build.rs`. No hardcoded numbers that go stale. The site IS the proof of artifacts.
10. `[test]` Add any-gpu to HTTP test assertions — `products_anygpu_link` test verifying the product card renders and links to GitHub.
11. `[docs]` Security audit disclosure — no third-party audit exists. Either commission one or document the self-audit methodology. Guest analysis flagged this for government buyers.
12. `[feature]` Rogue Repo integration — add `/rogue-repo` route or subdomain via approuter. Depends on: [rogue-repo](https://github.com/cochranblock/rogue-repo), [approuter](https://github.com/cochranblock/approuter).
13. `[feature]` Ronin Sites integration — multi-tenant shop platform behind approuter. Depends on: [rogue-repo](https://github.com/cochranblock/rogue-repo) payment engine, [approuter](https://github.com/cochranblock/approuter).
14. `[build]` Pocket Server MVP — phone-as-web-server, Android deployment. Depends on: [approuter](https://github.com/cochranblock/approuter), [kova](https://github.com/cochranblock/kova).
15. `[build]` Ghost Fabric MVP — LoRa mesh edge nodes with embedded AI. Depends on: [kova](https://github.com/cochranblock/kova).
16. `[fix]` NVIDIA driver on st (kova-elite-support) — `nvidia-smi` fails. Needs driver reinstall or kernel module rebuild. Blocks IRONHIVE 4-node cluster.
17. `[fix]` bt (kova-thick-beast) HDMI — unreachable since HDMI issue. Needs physical access or serial console.
18. `[research]` SBIR Phase I proposals — 11 agencies prepped (DoD, NSF, DHS/CISA, NIST, NIH, NASA, DOE, USDA, EPA, DOT, NOAA). All blocked on CAGE. Prep submission packages now so they're ready day-one.
19. `[research]` Multi-instance failover story — guest analysis: "one binary, one process, if it crashes the site is down." Document the approuter health-check + auto-restart architecture or build it.
20. `[fix]` Stored XSS pre-empt — form inputs (name, email, org, mission) stored unescaped in SQLite. No admin UI today so no active risk, but the auth/session infrastructure is already built. Escape on INSERT or assert no admin panel ships without sanitization on SELECT.
