#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// Unlicense — public domain — cochranblock.org

pub mod assets;
pub mod booking;
pub mod captains_log;
pub mod community_grant;
pub mod intake;
pub mod mailer;
pub mod n_bench;
pub mod pages;
pub mod router;
pub mod subdomain;
pub mod visits;
pub mod whyme;

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

static FORM_RATE: OnceLock<Mutex<HashMap<String, Vec<Instant>>>> = OnceLock::new();

/// check_rate_limit — per-IP gate for form POST endpoints.
/// Returns Ok(()) if the request is allowed, Err(retry_secs) if the IP has
/// exceeded 3 submissions in the last hour.
pub(super) fn check_rate_limit(ip: &str) -> Result<(), u64> {
    let store = FORM_RATE.get_or_init(|| Mutex::new(HashMap::new()));
    let now = Instant::now();
    let window = Duration::from_secs(3600);
    let mut guard = store.lock().unwrap_or_else(|e| e.into_inner());
    let entry = guard.entry(ip.to_string()).or_default();
    entry.retain(|t| now.duration_since(*t) < window);
    if entry.len() >= 3 {
        let retry_secs = (window.saturating_sub(now.duration_since(entry[0]))).as_secs() + 1;
        return Err(retry_secs);
    }
    entry.push(now);
    Ok(())
}
