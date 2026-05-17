# Code Style

cochranblock uses the **Token-Optimized Code Representation** system from [kova](https://github.com/cochranblock/kova). Compact identifiers reduce token cost in AI-assisted development sessions without sacrificing readability once you know the mapping.

## Identifier Conventions

| Pattern | Meaning |
|---------|---------|
| `f0`–`f999` | Functions (handlers, helpers) |
| `t0`–`t9` | Types / structs |
| `C0`–`C9` | Constants (HTML template chunks) |
| `v0`–`v9` | Local variables within a function |
| `p0`–`p9` | Parameters |

## Common Identifiers

| Identifier | What |
|------------|------|
| `t0` | Main app state struct (holds redb handle, config) |
| `C7` | Skip-link + `.cb-nav` nav block + `<main>` opener |
| `C8` | `</main>` + footer + closing tags |
| `C9` | Self-contained nav shim (nav + scoped CSS + cosmic starfield) |
| `f2_root` | Root subdomain dispatcher |
| `f105` | Nav injector for standalone artifact pages |

## Handler Signature

```rust
pub async fn fNN(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section>...</section>"#;
    Html([C7, v0, C8].concat())
}
```

Handlers that need the database:
```rust
pub async fn fNN(State(p0): State<Arc<t0>>) -> Html<String> {
    let db = p0.db.read().await;
    // ...
}
```

## No Comments Rule

Comments only appear when the WHY is non-obvious (a hidden constraint, subtle invariant, or workaround). Well-named identifiers carry the what. The [TIMELINE_OF_INVENTION.md](../TIMELINE_OF_INVENTION.md) carries the why at the commit level.
