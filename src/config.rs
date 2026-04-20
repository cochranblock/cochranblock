#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// All Rights Reserved — The Cochran Block, LLC
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use super::error::t18;

#[derive(Clone, Debug)]
pub struct t1 {
    pub s16: u16,
    pub s17: String,
    pub s18: String,
    pub s20: String,
    pub s21: String,
    pub s22: String,
    pub s9: u64,
    pub s23: Option<String>,
    pub s24: Option<String>,
    pub s26: Option<u16>,
}

impl t1 {
    /// from_env. Why: COCHRANBLOCK_MASTER_KEY required for crypto; others have safe defaults.
    pub fn from_env() -> Result<Self, t18> {
        let s20 = std::env::var("COCHRANBLOCK_MASTER_KEY")
            .map_err(|_| t18::E7("COCHRANBLOCK_MASTER_KEY required".into()))?;
        if s20.len() < 16 {
            return Err(t18::E7(
                "COCHRANBLOCK_MASTER_KEY must be at least 16 bytes".into(),
            ));
        }
        let env = |a: &str, default: &str| std::env::var(a).unwrap_or_else(|_| default.into());
        Ok(Self {
            s16: env("COCHRANBLOCK_PORT", "3000").parse().unwrap_or(3000),
            s17: env("COCHRANBLOCK_BIND", "127.0.0.1"),
            s18: std::env::var("COCHRANBLOCK_DATA_DIR").unwrap_or_else(|_| {
                std::env::var("COCHRANBLOCK_DATA_ROOT")
                    .ok()
                    .map(|r| format!("{}/cochranblock", r.trim_end_matches('/')))
                    .or_else(|| {
                        dirs::data_dir()
                            .map(|p| p.join("cochranblock").join("data"))
                            .and_then(|p| p.to_str().map(String::from))
                    })
                    .unwrap_or_else(|| "data".into())
            }),
            s20,
            s21: env(
                "COCHRANBLOCK_CF_BASE",
                "https://api.cloudflare.com/client/v4",
            ),
            s22: env("COCHRANBLOCK_IPIFY_URL", "https://api.ipify.org"),
            s9: env("COCHRANBLOCK_DNS_INTERVAL", "300")
                .parse()
                .unwrap_or(300),
            s23: std::env::var("COCHRANBLOCK_DOMAIN")
                .ok()
                .filter(|s| !s.trim().is_empty()),
            s24: std::env::var("COCHRANBLOCK_BIND_2")
                .ok()
                .filter(|s| !s.trim().is_empty()),
            s26: std::env::var("COCHRANBLOCK_PORT_2")
                .ok()
                .and_then(|s| s.trim().parse().ok()),
        })
    }

    pub fn db_path(&self) -> String {
        format!("{}/cochranblock.db", self.s18)
    }

    /// with_values. Why: Tests need explicit config without env vars.
    pub fn with_values(
        s16: u16,
        s17: &str,
        s18: &str,
        s20: &str,
        s21: &str,
        s22: &str,
        s9: u64,
    ) -> Self {
        Self {
            s16,
            s17: s17.to_string(),
            s18: s18.to_string(),
            s20: s20.to_string(),
            s21: s21.to_string(),
            s22: s22.to_string(),
            s9,
            s23: None,
            s24: None,
            s26: None,
        }
    }
}

impl Default for t1 {
    fn default() -> Self {
        Self {
            s16: 3000,
            s17: "127.0.0.1".into(),
            s18: "data".into(),
            s20: "test-master-key-32-bytes-long!!!!".into(),
            s21: "https://api.cloudflare.com/client/v4".into(),
            s22: "https://api.ipify.org".into(),
            s9: 300,
            s23: None,
            s24: None,
            s26: None,
        }
    }
}
