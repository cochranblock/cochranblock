# Quick Start

## Build and Run

```bash
# Clone
git clone https://github.com/cochranblock/cochranblock
cd cochranblock

# Build (requires Rust stable + cargo)
cargo build --release -p cochranblock --features approuter

# Run locally
./target/release/cochranblock   # serves localhost:8081
```

## Binary Size

```bash
ls -lh target/release/cochranblock   # ~13 MB x86, ~8.9 MB ARM
```

## Test Suite

```bash
# Full test run — TRIPLE SIMS gate (3x deterministic pass required)
cargo run -p cochranblock --bin cochranblock-test --features tests
```

## Supply Chain Audit

```bash
cargo audit   # 0 unignored advisories required before deploy
```

## Without approuter

Omit `--features approuter` to build an installable offline binary. It opens a browser to localhost on startup — no server registration required.
