<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Go Live — cochranblock.org

## 1. Prerequisites

- [x] Domain `cochranblock.org` in Cloudflare
- [x] Cloudflare API token (Zone.DNS Edit, Zone.Zone Read)
- [x] `COCHRANBLOCK_DOMAIN` and `CLOUDFLARE_API_TOKEN` in `.env`

## 2. CLI Options (No Scripts Required)

All runtime setup is via cochranblock command-line options:

| Option | Description |
|--------|-------------|
| `--ensure-cloudflared` | Download cloudflared to `bin/` if missing. Exits after. |
| `--go-live` | Ensure cloudflared, set PORT=443 BIND=0.0.0.0, serve with tunnel. |
| `--no-tunnel` | Disable Cloudflare Tunnel (tunnel runs by default). |

**Go live (single command):**
```bash
# WSL (primary workflow)
./scripts/go-live-wsl.sh
```

Or manually: `cargo build --release -p cochranblock && ./target/release/cochranblock --go-live`

**Windows:** Cross-compile later via `cargo build --release --target x86_64-pc-windows-msvc -p cochranblock`.

## 3. Quick Tunnel (No ACME, No Domain Setup)

Fastest way to get online — get a public URL in seconds:

```bash
# Terminal 1: Start cochranblock (uses existing certs or falls back to HTTP)
./target/debug/cochranblock

# Terminal 2: cloudflared tunnel
cloudflared tunnel --url https://localhost:443 --no-tls-verify
```

Cloudflared prints a URL like `https://random-words.trycloudflare.com` — that's your live site.

## 4. Obtain SSL Certs (ACME) — For cochranblock.org

```bash
# Staging first (no rate limits) — verify DNS-01 works
COCHRANBLOCK_ACME_STAGING=1 cargo run -p cochranblock -- --acme-staging

# If staging succeeds, production
cargo run -p cochranblock -- --acme
```

Certs save to `data/fullchain.pem` and `data/key.pem`.

**If DNS-01 fails:**
- Cloudflare may need 1–2 min to propagate the TXT record. Wait and retry.
- Verify token has **Zone.DNS Edit** and **Zone.Zone Read**.
- Verify `cochranblock.org` is in the same Cloudflare account as the token.
- Try apex only first: `COCHRANBLOCK_DOMAIN=cochranblock.org` (no www).

## 5. Start the Server

```bash
./target/debug/cochranblock
```

Listens on https://0.0.0.0:443.

## 6. Expose via Cloudflare Tunnel (Named — cochranblock.org)

No port forward needed. Cloudflare proxies to your local server.

### Built-in tunnel (default)

The cochranblock binary spawns cloudflared by default:

```bash
# From repo root (WSL or Windows)
cargo run -p cochranblock
```

Or with the built binary:

```bash
./target/debug/cochranblock
```

Use `--no-tunnel` to run without the tunnel.

**Requirements:**
- Run from repo root (so `data/` is found)
- `bin/cloudflared` — run `cochranblock --ensure-cloudflared` to download, or use `cloudflared` in PATH
- `data/cloudflared.yml`

### Named tunnel (one-time setup)

```bash
cloudflared tunnel create cochranblock
cloudflared tunnel route dns cochranblock cochranblock.org
cloudflared tunnel route dns cochranblock www.cochranblock.org
```

Config: `data/cloudflared.yml` (approuter-generated).

## 7. Run Both

```bash
cargo run -p cochranblock
```

Tunnel runs by default. Use `--no-tunnel` to disable.

Visit https://cochranblock.org