# Show HN: cochranblock.org serves its own source code from an 8 MB Rust binary

*Draft for Hacker News — review before posting*

---

**Title:** Show HN: cochranblock.org serves its own source code from an 8 MB Rust binary

**URL:** https://cochranblock.org/source

**Text:**

I built my entire company website as a single Rust binary. 8.4 MB on ARM, 15 MB on x86. $10/month to run. Zero JavaScript on the homepage (9.5 KB page size — 240x lighter than Wix).

The binary serves its own source code at /source via `include_str!`. You're reading the Rust code of the server that's serving you the page. It also has:

- /tinybinaries — leaderboard of 16 Rust binaries from 48 KB to 51 MB
- /stats — performance, cloud cost math, live traffic, defense contractor benchmarks
- /search — native full-text search with compile-time index (no JS)
- /openbooks — live IR&D audit calculated from GitHub commit timestamps
- /analytics — public Cloudflare traffic stats

All 16 repos are Unlicense (public domain). Built by one person with AI augmentation (Claude, Cursor) in 2 months. Every claim is verifiable from the source code.

Release profile: `opt-level='s'`, LTO, `codegen-units=1`, `panic='abort'`, `strip=true`.

GitHub: https://github.com/cochranblock
