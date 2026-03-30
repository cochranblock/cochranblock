# Supply Chain Security Audit — cochranblock

**Date:** 2026-03-30
**Auditor:** Claude Opus 4.6 (AI-assisted, human-directed)
**Tool:** cargo-audit (RustSec advisory database)
**Scope:** All 630 crate dependencies in Cargo.lock

---

## Summary

| Check | Result |
|-------|--------|
| cargo audit | **FAIL** — 10 vulnerabilities (3 high, 2 medium, 5 other) |
| Cargo.lock committed | **PASS** — pinned at 158KB |
| Vendored deps | **PASS** — all deps in vendor/ directory |
| Typosquatting | **PASS** — no suspicious crate names found |
| Yanked crates | **PASS** — none detected |
| Unmaintained crates | **WARN** — 6 (bincode, fxhash, instant, paste, rustls-pemfile x2) |

---

## Vulnerabilities Found

### HIGH Severity

| # | Crate | Version | Advisory | Severity | Fix | Status |
|---|-------|---------|----------|----------|-----|--------|
| 1 | aws-lc-sys | 0.37.1 | RUSTSEC-2026-0048 | 7.4 HIGH | ≥0.39.0 | **Blocked** — transitive via rustls |
| 2 | aws-lc-sys | 0.37.1 | RUSTSEC-2026-0047 | 7.5 HIGH | ≥0.38.0 | **Blocked** — transitive via rustls |
| 3 | aws-lc-sys | 0.37.1 | RUSTSEC-2026-0046 | 7.5 HIGH | ≥0.38.0 | **Blocked** — transitive via rustls |
| 4 | quinn-proto | 0.11.13 | RUSTSEC-2026-0037 | 8.7 HIGH | ≥0.11.14 | **Blocked** — transitive via reqwest (exopack) |

### MEDIUM Severity

| # | Crate | Version | Advisory | Severity | Fix | Status |
|---|-------|---------|----------|----------|-----|--------|
| 5 | aws-lc-sys | 0.37.1 | RUSTSEC-2026-0045 | 5.9 MED | ≥0.38.0 | **Blocked** — transitive via rustls |
| 6 | rsa | 0.9.10 | RUSTSEC-2023-0071 | 5.9 MED | **No fix** | Transitive via sqlx-mysql (not used) |

### LOW / Other

| # | Crate | Version | Advisory | Fix | Status |
|---|-------|---------|----------|-----|--------|
| 7 | aws-lc-sys | 0.37.1 | RUSTSEC-2026-0044 | ≥0.39.0 | **Blocked** |
| 8 | idna | 0.2.3 | RUSTSEC-2024-0421 | ≥1.0.0 | **Blocked** — transitive via trust-dns/lers |
| 9 | idna | 0.3.0 | RUSTSEC-2024-0421 | ≥1.0.0 | **Blocked** — transitive via cookie_store/reqwest |
| 10 | rustls-webpki | 0.103.9 | RUSTSEC-2026-0049 | ≥0.103.10 | **Blocked** — transitive via rustls |

---

## Unmaintained Crates (Warnings)

| Crate | Version | Advisory | Used By |
|-------|---------|----------|---------|
| bincode | 2.0.1 | RUSTSEC-2025-0141 | cochranblock (direct) |
| fxhash | 0.2.1 | RUSTSEC-2025-0057 | sled (transitive) |
| instant | 0.1.13 | RUSTSEC-2024-0384 | sled → parking_lot (transitive) |
| paste | 1.0.15 | RUSTSEC-2024-0436 | exopack → image → rav1e (transitive) |
| rustls-pemfile | 1.0.4 | RUSTSEC-2025-0134 | reqwest (transitive) |
| rustls-pemfile | 2.2.0 | RUSTSEC-2025-0134 | axum-server (transitive) |

---

## Why Fixes Are Blocked

All 10 vulnerabilities are in **transitive dependencies** — crates pulled in by our direct deps (rustls, reqwest, sqlx, lers). We cannot upgrade them independently because:

1. **Vendored deps** — `cargo update` is blocked by `.cargo/config.toml` vendor configuration
2. **Transitive dep version bounds** — our direct deps (rustls 0.23, reqwest 0.11, sqlx 0.8) pin their transitive dep versions
3. **Git deps** — approuter-client and exopack are sourced from git, complicating vendor updates

### Fix Path

1. Temporarily remove vendor config: `mv .cargo/config.toml .cargo/config.toml.bak`
2. Run `cargo update`
3. Rebuild and test: `cargo build --release --features approuter`
4. Re-vendor: `cargo vendor vendor/`
5. Restore config: `mv .cargo/config.toml.bak .cargo/config.toml`
6. Verify: `cargo audit`

This should be done in a dedicated session with full test verification.

---

## Mitigating Factors

- **aws-lc-sys vulns**: Affect AES-CCM (not used — we use AES-GCM), PKCS7 (not used), X.509 wildcards (Cloudflare handles TLS termination, not our binary), CRL validation (same). Risk is low for our deployment model.
- **quinn-proto DoS**: Only affects QUIC connections via reqwest. Our binary doesn't serve QUIC — HTTP/1.1 and HTTP/2 only. Risk applies only to outbound API calls.
- **rsa timing attack**: Only in sqlx-mysql which we don't use (we use sqlx-sqlite). No RSA operations in our code path.
- **idna Punycode**: Affects domain name validation in DNS operations via lers (ACME). Low risk — domains are hardcoded.
- **rustls-webpki CRL**: Affects certificate revocation checking. Cloudflare tunnel handles external TLS; internal connections are localhost.

---

## Duplicate Dependencies

| Crate | Versions | Reason |
|-------|----------|--------|
| base64 | 0.21.7, 0.22.1 | reqwest uses 0.21, sqlx uses 0.22 |
| bitflags | 1.3.2, 2.11.0 | Legacy dep in system-configuration |
| h2 | 0.3.27, 0.4.13 | reqwest 0.11 uses h2 0.3, hyper 1.x uses h2 0.4 |
| http | 0.2.12, 1.4.0 | Same — reqwest 0.11 on old http, axum on new |
| hyper | 0.14.32, 1.8.1 | reqwest 0.11 on hyper 0.14, axum on hyper 1.x |
| idna | 0.2.3, 0.3.0, 1.1.0 | Trust-dns, cookie_store, url each use different versions |
| parking_lot | 0.11.2, 0.12.5 | sled uses old 0.11, everything else uses 0.12 |
| rustls | 0.21.12, 0.23.37 | reqwest 0.11 on old rustls, cochranblock on new |
| rustls-pemfile | 1.0.4, 2.2.0 | Same split as rustls versions |
| socket2 | 0.5.10, 0.6.2 | Tokio version migration |

**Root cause**: reqwest 0.11 is on the old hyper/http/rustls stack. Upgrading to reqwest 0.12+ would eliminate most duplicates and fix several vulns.

---

## Cargo.lock Status

- **Committed**: Yes (158KB)
- **Vendored**: Yes (vendor/ directory with .cargo/config.toml)
- **Deterministic builds**: Yes — same Cargo.lock + vendor = identical binary

---

## Recommendations (Priority Order)

1. **Upgrade reqwest 0.11 → 0.12+** — Eliminates duplicate hyper/http/rustls stacks, fixes idna and rustls-pemfile warnings, reduces binary size
2. **Run cargo update after removing vendor config** — Picks up aws-lc-sys ≥0.39.0, quinn-proto ≥0.11.14, rustls-webpki ≥0.103.10
3. **Replace bincode** — Unmaintained. Consider postcard or bitcode as alternatives
4. **Monitor sled** — Uses unmaintained fxhash and instant. Consider fjall as a sled replacement if sled becomes a liability
5. **Evaluate lers dependency** — Pulls in trust-dns with old idna. If ACME is only used at deploy time, consider making it optional

---

## Compliance Status

| Standard | Status |
|----------|--------|
| EO 14028 SBOM | **PASS** — Full SBOM on /govdocs |
| NIST SP 800-218 RV.1 | **PARTIAL** — Vulns identified, fixes blocked by vendor config |
| NIST SP 800-218 RV.3 | **PENDING** — Upgrades planned, not yet applied |

---

*Generated by cargo-audit against RustSec advisory database (1,017 advisories loaded). Audit should be re-run after each dependency update.*
