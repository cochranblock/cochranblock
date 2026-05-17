#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! Zero-cloud website in a single Rust binary. 13 MB x86 / 8.9 MB ARM. $10/month infrastructure.
//!
//! **[Documentation](https://cochranblock.github.io/cochranblock/)**
//! — [Repository](https://github.com/cochranblock/cochranblock)
//! — [cochranblock.org](https://cochranblock.org)
// All Rights Reserved — The Cochran Block, LLC
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

pub mod error;
pub mod logs;
pub mod std_aliases;

#[cfg(feature = "admin")]
pub mod auth;
#[cfg(feature = "admin")]
pub mod config;
#[cfg(feature = "admin")]
pub mod crypto;
// db is always available — runtime intake (leads/grants/knox) uses redb in
// non-admin builds; admin-only functions (admins/sessions/settings/dns_log)
// are feature-gated inside db/mod.rs.
pub mod db;
#[cfg(feature = "admin")]
pub mod dns;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(feature = "tests")]
pub mod screenshot;
#[cfg(feature = "tests")]
pub mod tests;
pub mod ux;
pub mod web;

/// t1 = SiteStats. Cached dynamic numbers. Single source of truth.
#[derive(Clone, Default, serde::Serialize)]
pub struct t1 {
    pub repo_count: usize,
    pub unlicense_count: usize,
    pub requests_7d: u64,
    pub visitors_7d: u64,
    pub ird_hours: f64,
    pub ird_value: f64,
}

/// t0 = AppState. Why: Shared state for router; redb store for intake +
/// (when admin is on) admins/sessions/settings/dns_log.
#[derive(Clone)]
pub struct t0 {
    pub intake_db: Option<crate::db::t9>,
}
