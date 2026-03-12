# Cochranblock Test Walkthrough

This document describes the `cochranblock-test` binary, how to run it, and what each test group covers.

## Running Tests

```bash
cargo run -p cochranblock --bin cochranblock-test --features tests
```

The binary runs all test groups in sequence (unit → integration → HTTP), then performs screenshot capture. It runs via exopack TRIPLE SIMS (3 full passes) for robustness. If any test fails, the process exits with code 1.

**Environment:** Optional `.env` via `dotenvy`. Tests use `t0 { intake_pool: None }` — no live database for HTTP tests. Integration tests use a temporary SQLite DB.

## Test Groups

### 1. Unit Tests (`unit::f49`)

Covers core libraries without external services:

| Test | Purpose |
|------|---------|
| `crypto_roundtrip` | AEAD encrypt/decrypt roundtrip |
| `crypto_tamper` | Tampered ciphertext fails decrypt |
| `crypto_wrong_key` | Wrong key fails decrypt |
| `crypto_determinism` | AEAD produces different ciphertext each time |
| `crypto_not_plaintext` | Ciphertext does not leak plaintext |
| `key_derivation_isolation` | Different contexts yield different keys |
| `password_hash_verify` | Argon2 hash/verify |
| `password_unique_salts` | Each hash uses unique salt |
| `password_wrong_returns_false` | Wrong password does not verify |
| `session_creation` | Session token encode/decode |
| `session_expired_invalid` | Expired session rejected |
| `config_defaults` | Config defaults from env |
| `config_db_path` | Config DB path resolution |
| `config_with_values` | Config with explicit values |
| `error_no_internal_leaks` | Error types do not leak internals |
| `dns_url_build` | Cloudflare DNS URL construction |
| `dns_ip_changed` | DNS change detection |
| `dns_empty_token` | Empty token handling |
| `key_f32_deterministic` | Key f32 determinism |
| `session_id_format` | Session ID format validation |

**Feature:** Requires `admin` (crypto, auth, config, dns).

### 2. Integration Tests (`integration::f50`)

Uses a temporary SQLite database:

| Test | Purpose |
|------|---------|
| `db_init` | DB init and migrations apply |
| `admin_crud` | Admin insert/retrieve/count |
| `admin_duplicate_rejection` | Duplicate admin rejected |
| `session_crud` | Session insert/retrieve/delete |
| `encrypted_settings` | Encrypted key-value store roundtrip |
| `encrypted_settings_wrong_key` | Wrong key fails decrypt |
| `plaintext_upsert` | Plaintext settings upsert |
| `dns_log` | DNS log insert |
| `session_cleanup` | Session cleanup |
| `db_nested_path` | DB nested path handling |
| `dns_log_limit` | DNS log limit |
| `settings_missing_key` | Missing settings key handling |

**Feature:** Requires `admin`.

### 3. HTTP Tests (`http::f51`)

Spawns an in-memory Axum server and hits routes via reqwest:

| Category | Tests |
|----------|-------|
| **Pages** | `index_200`, `services_200`, `about_200`, `contact_200`, `intake_200`, `community_grant_200`, `intake_confirmed_200`, `community_grant_confirmed_200`, `products_200`, `book_200`, `federal_partners_200` |
| **Form fields** | `intake_form_fields`, `community_grant_form_fields` |
| **Static assets** | `static_css`, `calendar_js`, `booking_js`, `favicon_svg`, `resume_pdf`, `logo_svg`, `product_images_200` |
| **SEO / meta** | `robots_txt`, `sitemap_xml`, `json_ld_org`, `html_doctype`, `meta_viewport`, `semantic_main_nav` |
| **Health** | `health_ok` |
| **Nav / footer** | `nav_footer_links`, `buttons_nav_all_200`, `buttons_footer_links_200`, `buttons_hero_ctas_200`, `buttons_products_ctas_200`, `buttons_contact_ctas_200`, `buttons_book_nav_200` |
| **Content** | `index_business`, `services_content`, `contact_links`, `about_tabs`, `home_ctas`, `federal_partners_200`, various product/service/contact content checks |
| **Book** | `book_calendar_structure`, `book_slots_mailto`, `book_slots_json`, `book_weekdays_only` |
| **Errors** | `404`, `assets_404`, `removed_routes_404` |
| **Encoding** | `gzip_encoding` |
| **Dev routes** | `dev_routes_200` (when `dev` feature enabled) |

**Intake / Community Grant:** Tests assert presence of form fields (`full_name`, `email`, `org_name`, `mission`, `technical_objective`), consent checkboxes ($3,500, $500), and form actions. Confirmed pages assert "Request Received" / "Application Received" or "submitted".

**Unlisted:** `/community-grant` is not in sitemap or nav; it is tested for 200 and form structure.

### 4. Screenshot Capture (`screenshot::f53`)

Uses exopack to capture pages for TRIPLE SIMS visual verification. Pages:

| Name | Path |
|------|------|
| index | / |
| services | /services |
| products | /products |
| federal-partners | /federal-partners |
| about | /about |
| contact | /contact |
| intake | /intake |
| community-grant | /community-grant |
| book | /book |

Screenshots are written to the exopack cache, then copied to `screenshots/` in the repo.

## Routes Covered

| Route | Tested |
|-------|--------|
| `/` | index_200, index_business, home_ctas, etc. |
| `/services` | services_200, services_content, etc. |
| `/products` | products_200, products_all_coming_soon, etc. |
| `/about` | about_200, about_tabs, etc. |
| `/contact` | contact_200, contact_links, etc. |
| `/intake` | intake_200, intake_form_fields |
| `/intake/confirmed` | intake_confirmed_200 |
| `/community-grant` | community_grant_200, community_grant_form_fields |
| `/community-grant/confirmed` | community_grant_confirmed_200 |
| `/book` | book_200, book_calendar_structure, etc. |
| `/federal-partners` | federal_partners_200 |
| `/health` | health_ok |
| `/robots.txt` | robots_txt |
| `/sitemap.xml` | sitemap_xml |
| `/assets/*` | static_css, calendar_js, favicon_svg, etc. |

## Build Preset

From workspace root:

```bash
cargo build -p cochranblock --release --features approuter
```

Tests use `--features tests` (includes `admin` for unit/integration).
