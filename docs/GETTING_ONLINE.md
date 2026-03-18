<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Getting Online — Caddy-like Auto HTTPS

The cochranblock uses **lers** (Rust ACME/Let's Encrypt) with **Cloudflare DNS-01** for automatic HTTPS. No HTTP challenge needed — works behind NAT, firewalls, or before the server is reachable. **ACME is always enabled.** No escape hatch.

## Prerequisites

1. **Domain** — e.g. `cochranblock.org` with DNS managed by Cloudflare
2. **Cloudflare API token** — Zone.DNS Edit, Zone.Zone Read
3. **Build** — `cargo build -p cochranblock`

### Build (WSL)

Primary workflow: build and run from WSL.

```bash
cargo build -p cochranblock --release
```

**Windows cross-compile (later):** `cargo build --release --target x86_64-pc-windows-msvc -p cochranblock`

## One-shot: Obtain certs then serve

```bash
# 1. Set env
export COCHRANBLOCK_DOMAIN=cochranblock.org,www.cochranblock.org
export CLOUDFLARE_API_TOKEN=your-token
export COCHRANBLOCK_DATA_DIR=data

# 2. Obtain certs (staging first to avoid rate limits)
COCHRANBLOCK_ACME_STAGING=1 cargo run -p cochranblock -- --acme-staging

# 3. Production certs
cargo run -p cochranblock -- --acme

# 4. Serve (certs in data/)
cargo run -p cochranblock
```

## Caddy-like: Auto obtain on startup

When certs are missing, the server will attempt ACME before serving:

```bash
export COCHRANBLOCK_DOMAIN=cochranblock.org,www.cochranblock.org
export CLOUDFLARE_API_TOKEN=your-token
export COCHRANBLOCK_PORT=443
export COCHRANBLOCK_BIND=0.0.0.0

# First run: use staging to test
export COCHRANBLOCK_ACME_STAGING=1

cargo run -p cochranblock
```

On success, certs are written to `COCHRANBLOCK_DATA_DIR` and the server serves HTTPS. On failure, it falls back to HTTP.

## Exposure options

| Method | Use case |
|--------|----------|
| **Port forward** | Router forwards 443 → your machine |
| **Cloudflare Tunnel** | `cloudflared tunnel` — no port forward, Cloudflare proxies to your origin |
| **VPS** | Run on a server with public IP |

With Cloudflare Tunnel, set origin to `https://localhost:443` or your bind address.

## Env reference

| Variable | Purpose |
|----------|---------|
| `COCHRANBLOCK_DOMAIN` | Comma-separated domains for ACME (e.g. `example.com,www.example.com`) |
| `CLOUDFLARE_API_TOKEN` | Cloudflare API token for DNS-01 |
| `COCHRANBLOCK_ACME_STAGING` | If set, use Let's Encrypt staging (no rate limits) |
| `COCHRANBLOCK_DATA_DIR` | Where certs are stored (`fullchain.pem`, `key.pem`) |