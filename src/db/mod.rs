#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! Persistence: pure-Rust embedded redb. Single-file ACID, zero-copy reads,
//! no transitive RSA / no aws-lc / no fxhash / no instant — replaces the
//! previous (test-only) sled wrapper AND the runtime sqlx + sqlite intake
//! pool with a single store. Values bincoded (no zstd; redb is already
//! compact-by-design and bincode-2 is small).
//!
//! Always-available tables (intake — used by /deploy, /community-grant,
//! KNOXAI applicant intake):
//!   leads               — lead id → bincoded `LeadRow`
//!   community_grants    — grant id → bincoded `GrantRow`
//!   knoxai_applicants   — applicant id → bincoded `KnoxRow`
//!
//! `feature = "admin"` adds (currently used only by the integration test
//! suite — the production binary builds with `--features approuter` only):
//!   admins              — username → password hash bytes
//!   sessions            — session token → bincoded `t3` session
//!   settings            — key → value bytes (caller decides encryption)
//!   dns_log_next        — auto-increment counter for dns_log
//!   dns_log             — id → bincoded `DnsLogEntry`
//
// All Rights Reserved — The Cochran Block, LLC
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bincode::config::standard;
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::error::t18;

// ───────────────────── Always-on tables ─────────────────────

const T_LEADS: TableDefinition<&str, &[u8]> = TableDefinition::new("leads");
const T_GRANTS: TableDefinition<&str, &[u8]> = TableDefinition::new("community_grants");
const T_KNOX: TableDefinition<&str, &[u8]> = TableDefinition::new("knoxai_applicants");

// ───────────────────── Intake row types ─────────────────────

#[derive(Serialize, Deserialize, Clone)]
pub struct LeadRow {
    pub id: String,
    pub deploy_class: Option<String>,
    pub full_name: String,
    pub email: String,
    pub company: Option<String>,
    pub message: Option<String>,
    pub submitted_at: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub terms_version: String,
    pub consent_fee: bool,
    pub consent_hardware: bool,
    pub consent_terms: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GrantRow {
    pub id: String,
    pub org_name: String,
    pub ein: Option<String>,
    pub contact_name: String,
    pub contact_email: String,
    pub mission: String,
    pub technical_objective: String,
    pub submitted_at: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub consent_grant: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KnoxRow {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub background: String,
    pub specialty_tags: String,
    pub clearance: String,
    pub motivation: String,
    pub hazmat_answer: String,
    pub acknowledge_csam: bool,
    pub submitted_at: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// ───────────────────── Always-on helpers ─────────────────────

/// t9 = the persistence handle. Why: shared across handlers + middleware.
pub type t9 = Arc<Database>;

fn enc<T: Serialize>(v: &T) -> Result<Vec<u8>, t18> {
    bincode::serde::encode_to_vec(v, standard()).map_err(|e| t18::E6(e.to_string()))
}

fn dec<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, t18> {
    bincode::serde::decode_from_slice(b, standard())
        .map(|(v, _)| v)
        .map_err(|e| t18::E6(e.to_string()))
}

fn err<E: std::fmt::Display>(e: E) -> t18 {
    t18::E6(e.to_string())
}

/// Resolve the on-disk redb file path given a directory or file argument.
fn resolve_path(db_path: &str) -> PathBuf {
    let p = Path::new(db_path);
    if p.extension().and_then(|e| e.to_str()) == Some("redb") {
        p.to_path_buf()
    } else if p.is_dir() || db_path.ends_with('/') {
        p.join("cochranblock.redb")
    } else if p.extension().is_some() {
        p.with_file_name("cochranblock.redb")
    } else {
        p.join("cochranblock.redb")
    }
}

/// f20 = init_db. Open-or-create redb at `db_path`. Creates parent dir.
pub fn f20(db_path: &str) -> Result<t9, t18> {
    let file = resolve_path(db_path);
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent).map_err(err)?;
    }
    let db = Database::create(&file).map_err(err)?;
    Ok(Arc::new(db))
}

/// f21 = run_migrations. redb is schemaless; we touch every table so they
/// exist on first open (matches the sled-era contract).
pub fn f21(db: &t9) -> Result<(), t18> {
    let txn = db.begin_write().map_err(err)?;
    {
        let _ = txn.open_table(T_LEADS).map_err(err)?;
        let _ = txn.open_table(T_GRANTS).map_err(err)?;
        let _ = txn.open_table(T_KNOX).map_err(err)?;
        #[cfg(feature = "admin")]
        admin_only::touch_all_tables(&txn)?;
    }
    txn.commit().map_err(err)?;
    Ok(())
}

// ───────────────────── intake (leads / grants / knox) ─────────────────────

/// f50 = lead_insert.
pub fn f50(db: &t9, row: &LeadRow) -> Result<(), t18> {
    let v = enc(row)?;
    let txn = db.begin_write().map_err(err)?;
    {
        let mut table = txn.open_table(T_LEADS).map_err(err)?;
        table.insert(row.id.as_str(), v.as_slice()).map_err(err)?;
    }
    txn.commit().map_err(err)?;
    Ok(())
}

/// f51 = grant_insert.
pub fn f51(db: &t9, row: &GrantRow) -> Result<(), t18> {
    let v = enc(row)?;
    let txn = db.begin_write().map_err(err)?;
    {
        let mut table = txn.open_table(T_GRANTS).map_err(err)?;
        table.insert(row.id.as_str(), v.as_slice()).map_err(err)?;
    }
    txn.commit().map_err(err)?;
    Ok(())
}

/// f52 = knox_insert.
pub fn f52(db: &t9, row: &KnoxRow) -> Result<(), t18> {
    let v = enc(row)?;
    let txn = db.begin_write().map_err(err)?;
    {
        let mut table = txn.open_table(T_KNOX).map_err(err)?;
        table.insert(row.id.as_str(), v.as_slice()).map_err(err)?;
    }
    txn.commit().map_err(err)?;
    Ok(())
}

/// f53 = lead_count. O(1) per redb.
pub fn f53(db: &t9) -> Result<u64, t18> {
    let txn = db.begin_read().map_err(err)?;
    let table = txn.open_table(T_LEADS).map_err(err)?;
    table.len().map_err(err)
}

/// f54 = grant_count.
pub fn f54(db: &t9) -> Result<u64, t18> {
    let txn = db.begin_read().map_err(err)?;
    let table = txn.open_table(T_GRANTS).map_err(err)?;
    table.len().map_err(err)
}

/// f55 = knox_count.
pub fn f55(db: &t9) -> Result<u64, t18> {
    let txn = db.begin_read().map_err(err)?;
    let table = txn.open_table(T_KNOX).map_err(err)?;
    table.len().map_err(err)
}

// ───────────────────── admin-gated bits ─────────────────────

#[cfg(feature = "admin")]
mod admin_only {
    use super::*;
    use crate::auth::session::t3;

    pub(super) const T_ADMINS: TableDefinition<&str, &[u8]> = TableDefinition::new("admins");
    pub(super) const T_SESSIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("sessions");
    pub(super) const T_SETTINGS: TableDefinition<&str, &[u8]> = TableDefinition::new("settings");
    pub(super) const T_DNS_LOG: TableDefinition<u64, &[u8]> = TableDefinition::new("dns_log");
    pub(super) const T_DNS_NEXT: TableDefinition<&str, u64> =
        TableDefinition::new("dns_log_next");

    #[derive(Serialize, Deserialize)]
    pub struct DnsLogEntry {
        pub ts: String,
        pub ip: String,
        pub status: String,
        pub msg: String,
    }

    pub(super) fn touch_all_tables(txn: &redb::WriteTransaction) -> Result<(), t18> {
        let _ = txn.open_table(T_ADMINS).map_err(err)?;
        let _ = txn.open_table(T_SESSIONS).map_err(err)?;
        let _ = txn.open_table(T_SETTINGS).map_err(err)?;
        let _ = txn.open_table(T_DNS_LOG).map_err(err)?;
        let _ = txn.open_table(T_DNS_NEXT).map_err(err)?;
        Ok(())
    }

    /// f36 = admin_get.
    pub fn f36(db: &t9, s10: &str) -> Result<Option<(String, String)>, t18> {
        let txn = db.begin_read().map_err(err)?;
        let table = txn.open_table(T_ADMINS).map_err(err)?;
        Ok(table.get(s10).map_err(err)?.map(|v| {
            let bytes = v.value().to_vec();
            (
                s10.to_string(),
                String::from_utf8_lossy(&bytes).into_owned(),
            )
        }))
    }

    /// f37 = admin_insert. Rejects duplicates (INSERT semantics).
    pub fn f37(db: &t9, s10: &str, s11: &str) -> Result<(), t18> {
        let txn = db.begin_write().map_err(err)?;
        {
            let mut table = txn.open_table(T_ADMINS).map_err(err)?;
            if table.get(s10).map_err(err)?.is_some() {
                return Err(t18::E6("admin already exists".into()));
            }
            table.insert(s10, s11.as_bytes()).map_err(err)?;
        }
        txn.commit().map_err(err)?;
        Ok(())
    }

    /// f38 = admin_has_any.
    pub fn f38(db: &t9) -> Result<bool, t18> {
        let txn = db.begin_read().map_err(err)?;
        let table = txn.open_table(T_ADMINS).map_err(err)?;
        Ok(table.len().map_err(err)? > 0)
    }

    /// f39 = session_insert.
    pub fn f39(db: &t9, s: &t3) -> Result<(), t18> {
        let v = enc(s)?;
        let txn = db.begin_write().map_err(err)?;
        {
            let mut table = txn.open_table(T_SESSIONS).map_err(err)?;
            table.insert(s.s37.as_str(), v.as_slice()).map_err(err)?;
        }
        txn.commit().map_err(err)?;
        Ok(())
    }

    /// f40 = session_get.
    pub fn f40(db: &t9, s37: &str) -> Result<Option<t3>, t18> {
        let txn = db.begin_read().map_err(err)?;
        let table = txn.open_table(T_SESSIONS).map_err(err)?;
        match table.get(s37).map_err(err)? {
            Some(v) => Ok(Some(dec::<t3>(v.value())?)),
            None => Ok(None),
        }
    }

    /// f41 = session_delete.
    pub fn f41(db: &t9, s37: &str) -> Result<(), t18> {
        let txn = db.begin_write().map_err(err)?;
        {
            let mut table = txn.open_table(T_SESSIONS).map_err(err)?;
            table.remove(s37).map_err(err)?;
        }
        txn.commit().map_err(err)?;
        Ok(())
    }

    /// f33 = session_cleanup_expired.
    pub fn f33(db: &t9) -> Result<(), t18> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut to_del: Vec<String> = Vec::new();
        {
            let txn = db.begin_read().map_err(err)?;
            let table = txn.open_table(T_SESSIONS).map_err(err)?;
            for r in table.iter().map_err(err)? {
                let (k, v) = r.map_err(err)?;
                if let Ok(s) = dec::<t3>(v.value()) {
                    if s.s14 < now {
                        to_del.push(k.value().to_string());
                    }
                }
            }
        }
        if to_del.is_empty() {
            return Ok(());
        }
        let txn = db.begin_write().map_err(err)?;
        {
            let mut table = txn.open_table(T_SESSIONS).map_err(err)?;
            for k in to_del {
                table.remove(k.as_str()).map_err(err)?;
            }
        }
        txn.commit().map_err(err)?;
        Ok(())
    }

    /// f42 = settings_get.
    pub fn f42(db: &t9, key: &str) -> Result<Option<String>, t18> {
        let txn = db.begin_read().map_err(err)?;
        let table = txn.open_table(T_SETTINGS).map_err(err)?;
        Ok(table
            .get(key)
            .map_err(err)?
            .map(|v| String::from_utf8_lossy(v.value()).into_owned()))
    }

    /// f43 = settings_upsert.
    pub fn f43(db: &t9, key: &str, value: &str) -> Result<(), t18> {
        let txn = db.begin_write().map_err(err)?;
        {
            let mut table = txn.open_table(T_SETTINGS).map_err(err)?;
            table.insert(key, value.as_bytes()).map_err(err)?;
        }
        txn.commit().map_err(err)?;
        Ok(())
    }

    /// f44 = dns_log_append. Auto-increment id via T_DNS_NEXT counter.
    pub fn f44(db: &t9, ip: &str, status: &str, msg: &str) -> Result<(), t18> {
        let entry = DnsLogEntry {
            ts: chrono::Utc::now().to_rfc3339(),
            ip: ip.to_string(),
            status: status.to_string(),
            msg: msg.to_string(),
        };
        let v = enc(&entry)?;
        let txn = db.begin_write().map_err(err)?;
        let next = {
            let mut counter = txn.open_table(T_DNS_NEXT).map_err(err)?;
            let cur = counter
                .get("next")
                .map_err(err)?
                .map(|v| v.value())
                .unwrap_or(0);
            let nxt = cur + 1;
            counter.insert("next", nxt).map_err(err)?;
            nxt
        };
        {
            let mut table = txn.open_table(T_DNS_LOG).map_err(err)?;
            table.insert(next, v.as_slice()).map_err(err)?;
        }
        txn.commit().map_err(err)?;
        Ok(())
    }

    /// f45 = dns_log_recent. Latest N entries, id DESC.
    pub fn f45(db: &t9, limit: i32) -> Result<Vec<(String, String, String, String)>, t18> {
        let mut out = Vec::new();
        let txn = db.begin_read().map_err(err)?;
        let table = txn.open_table(T_DNS_LOG).map_err(err)?;
        for r in table.iter().map_err(err)?.rev().take(limit as usize) {
            let (_, v) = r.map_err(err)?;
            let e: DnsLogEntry = dec(v.value())?;
            out.push((e.ts, e.ip, e.status, e.msg));
        }
        Ok(out)
    }
}

// Re-export admin-only fns when feature is on so existing call sites
// (`db::f36(...)`) keep working without changes.
#[cfg(feature = "admin")]
pub use admin_only::{
    DnsLogEntry, f33, f36, f37, f38, f39, f40, f41, f42, f43, f44, f45,
};
