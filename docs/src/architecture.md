# Architecture

## System Diagram

```
User → Cloudflare Tunnel → approuter :8080 → cochranblock :8081
                                           → oakilydokily :3000
cochranblock :8081 → redb (embedded ACID database)
                   → Embedded Assets (HTML/CSS/PDF baked into binary)
```

## Binary Composition

The release binary embeds everything at compile time via `include_packed`:

- All HTML assets (`assets/`, `content/`)
- CSS and static files
- PDFs (capability statement, resume)
- redb database file path (database itself is runtime-created)

No files are read from disk at runtime except the redb database. The binary is fully self-contained.

## Subdomain Dispatch

Incoming requests are dispatched by `Host` header in `f2_root`:

| Subdomain | Handler |
|-----------|---------|
| `cochranblock.org` (apex) | LET'S TEAM page (`assets/lets-team.html`) |
| `manual.cochranblock.org` | Folded manifesto + ops manual |
| `knox.cochranblock.org` | KNOXAI operator portal |
| `simplify.cochranblock.org` | Simplify page |
| `whyme.cochranblock.org` | Hire-me page |
| everything else | Standard C7+C8 template routing |

## Template System

Pages use two compile-time constants:

- `C7` — skip-link + `.cb-nav` navigation block + `<main>` opener
- `C8` — `</main>` + footer + closing tags

Handler pattern:
```rust
pub async fn f67(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section>...</section>"#;
    Html([C7, v0, C8].concat())
}
```

Standalone artifact pages (manifesto, constitution, etc.) use `f105()` to inject the nav shim after `<body>`.

## Storage

- **redb**: single-file ACID embedded database. Used for intake form submissions, analytics, and session data.
- No external database. No cloud storage. The database file lives on the VPS at a configured path.

## Infrastructure Cost

| Line item | Cost |
|-----------|------|
| VPS (gd) | ~$10/month |
| Cloudflare Tunnel | Free |
| Domain | ~$12/year |
| **Total** | **~$10/month** |
