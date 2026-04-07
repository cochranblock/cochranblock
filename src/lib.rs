#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! CochranBlock — Rust-only SaaS. Kova: thiserror, no unwrap in lib, /// doc = Why.
//! Flattened: core/* → src/ (folder_token_map).
// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

pub mod error;
pub mod std_aliases;

#[cfg(feature = "admin")]
pub mod auth;
#[cfg(feature = "admin")]
pub mod config;
#[cfg(feature = "admin")]
pub mod crypto;
#[cfg(feature = "admin")]
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

/// t0 = AppState. Why: Shared state for router; intake pool for leads.
#[derive(Clone)]
pub struct t0 {
    pub intake_pool: Option<sqlx::SqlitePool>,
}
