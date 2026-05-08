//! One-shot migration: cochranblock intake.sqlite -> cochranblock.redb.
//!
//! Reads `~/.local/share/cochranblock/data/intake.sqlite` (or paths from argv)
//! and writes `cochranblock.redb` next to it. Idempotent: re-running upserts
//! by primary-key id. Empty source tables are skipped silently.
//!
//! Row types here MUST match `cochranblock::db::{LeadRow, GrantRow, KnoxRow}`
//! field-for-field — bincode-2 with `standard()` config produces the same
//! bytes the live server reads.

use std::path::{Path, PathBuf};

use bincode::config::standard;
use redb::{Database, ReadableTableMetadata, TableDefinition};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

const T_LEADS: TableDefinition<&str, &[u8]> = TableDefinition::new("leads");
const T_GRANTS: TableDefinition<&str, &[u8]> = TableDefinition::new("community_grants");
const T_KNOX: TableDefinition<&str, &[u8]> = TableDefinition::new("knoxai_applicants");

#[derive(Serialize, Deserialize, Clone)]
struct LeadRow {
    id: String,
    deploy_class: Option<String>,
    full_name: String,
    email: String,
    company: Option<String>,
    message: Option<String>,
    submitted_at: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    terms_version: String,
    consent_fee: bool,
    consent_hardware: bool,
    consent_terms: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct GrantRow {
    id: String,
    org_name: String,
    ein: Option<String>,
    contact_name: String,
    contact_email: String,
    mission: String,
    technical_objective: String,
    submitted_at: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    consent_grant: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct KnoxRow {
    id: String,
    full_name: String,
    email: String,
    background: String,
    specialty_tags: String,
    clearance: String,
    motivation: String,
    hazmat_answer: String,
    acknowledge_csam: bool,
    submitted_at: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

fn default_data_dir() -> PathBuf {
    if let Ok(p) = std::env::var("CB_DATA_DIR") {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("COCHRANBLOCK_DATA_DIR") {
        return PathBuf::from(p);
    }
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".local/share/cochranblock/data");
    }
    PathBuf::from("data")
}

fn enc<T: Serialize>(v: &T) -> Vec<u8> {
    bincode::serde::encode_to_vec(v, standard())
        .expect("bincode encode")
}

fn table_exists(conn: &Connection, name: &str) -> bool {
    conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1",
        [name],
        |_| Ok(()),
    )
    .is_ok()
}

/// Build a `SELECT col1, col2, ...` that survives older schemas — any
/// requested column missing from the live table becomes `NULL AS col`,
/// so old DBs without recently-added columns still migrate cleanly.
fn select_safe(conn: &Connection, table: &str, cols: &[&str]) -> rusqlite::Result<String> {
    let mut existing = std::collections::HashSet::new();
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let mut rows = stmt.query([])?;
    while let Some(r) = rows.next()? {
        existing.insert(r.get::<_, String>(1)?);
    }
    let proj: Vec<String> = cols
        .iter()
        .map(|c| {
            if existing.contains(*c) {
                (*c).to_string()
            } else {
                format!("NULL AS {}", c)
            }
        })
        .collect();
    Ok(format!("SELECT {} FROM {}", proj.join(", "), table))
}

fn migrate_leads(conn: &Connection, db: &Database) -> Result<usize, Box<dyn std::error::Error>> {
    if !table_exists(conn, "leads") {
        return Ok(0);
    }
    let sql = select_safe(
        conn,
        "leads",
        &[
            "id",
            "deploy_class",
            "full_name",
            "email",
            "company",
            "message",
            "submitted_at",
            "ip_address",
            "user_agent",
            "terms_version",
            "consent_fee",
            "consent_hardware",
            "consent_terms",
        ],
    )?;
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| {
        Ok(LeadRow {
            id: r.get(0)?,
            deploy_class: r.get(1)?,
            full_name: r.get(2)?,
            email: r.get(3)?,
            company: r.get(4)?,
            message: r.get(5)?,
            submitted_at: r.get(6)?,
            ip_address: r.get(7)?,
            user_agent: r.get(8)?,
            terms_version: r.get(9)?,
            consent_fee: r.get::<_, i64>(10)? != 0,
            consent_hardware: r.get::<_, i64>(11)? != 0,
            consent_terms: r.get::<_, i64>(12)? != 0,
        })
    })?;

    let txn = db.begin_write()?;
    let mut n = 0usize;
    {
        let mut t = txn.open_table(T_LEADS)?;
        for row in rows {
            let row = row?;
            let v = enc(&row);
            t.insert(row.id.as_str(), v.as_slice())?;
            n += 1;
        }
    }
    txn.commit()?;
    Ok(n)
}

fn migrate_grants(conn: &Connection, db: &Database) -> Result<usize, Box<dyn std::error::Error>> {
    if !table_exists(conn, "community_grants") {
        return Ok(0);
    }
    let sql = select_safe(
        conn,
        "community_grants",
        &[
            "id",
            "org_name",
            "ein",
            "contact_name",
            "contact_email",
            "mission",
            "technical_objective",
            "submitted_at",
            "ip_address",
            "user_agent",
            "consent_grant",
        ],
    )?;
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| {
        Ok(GrantRow {
            id: r.get(0)?,
            org_name: r.get(1)?,
            ein: r.get(2)?,
            contact_name: r.get(3)?,
            contact_email: r.get(4)?,
            mission: r.get(5)?,
            technical_objective: r.get(6)?,
            submitted_at: r.get(7)?,
            ip_address: r.get(8)?,
            user_agent: r.get(9)?,
            consent_grant: r.get::<_, i64>(10)? != 0,
        })
    })?;

    let txn = db.begin_write()?;
    let mut n = 0usize;
    {
        let mut t = txn.open_table(T_GRANTS)?;
        for row in rows {
            let row = row?;
            let v = enc(&row);
            t.insert(row.id.as_str(), v.as_slice())?;
            n += 1;
        }
    }
    txn.commit()?;
    Ok(n)
}

fn migrate_knox(conn: &Connection, db: &Database) -> Result<usize, Box<dyn std::error::Error>> {
    if !table_exists(conn, "knoxai_applicants") {
        return Ok(0);
    }
    let sql = select_safe(
        conn,
        "knoxai_applicants",
        &[
            "id",
            "full_name",
            "email",
            "background",
            "specialty_tags",
            "clearance",
            "motivation",
            "hazmat_answer",
            "acknowledge_csam",
            "submitted_at",
            "ip_address",
            "user_agent",
        ],
    )?;
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| {
        Ok(KnoxRow {
            id: r.get(0)?,
            full_name: r.get(1)?,
            email: r.get(2)?,
            background: r.get(3)?,
            specialty_tags: r.get(4)?,
            clearance: r.get(5)?,
            motivation: r.get(6)?,
            hazmat_answer: r.get(7)?,
            acknowledge_csam: r.get::<_, i64>(8)? != 0,
            submitted_at: r.get(9)?,
            ip_address: r.get(10)?,
            user_agent: r.get(11)?,
        })
    })?;

    let txn = db.begin_write()?;
    let mut n = 0usize;
    {
        let mut t = txn.open_table(T_KNOX)?;
        for row in rows {
            let row = row?;
            let v = enc(&row);
            t.insert(row.id.as_str(), v.as_slice())?;
            n += 1;
        }
    }
    txn.commit()?;
    Ok(n)
}

fn touch_tables(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let txn = db.begin_write()?;
    {
        let _ = txn.open_table(T_LEADS)?;
        let _ = txn.open_table(T_GRANTS)?;
        let _ = txn.open_table(T_KNOX)?;
    }
    txn.commit()?;
    Ok(())
}

fn count(db: &Database, t: TableDefinition<&str, &[u8]>) -> Result<u64, Box<dyn std::error::Error>> {
    let txn = db.begin_read()?;
    let table = txn.open_table(t)?;
    Ok(table.len()?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let dir = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(default_data_dir);

    let sqlite_path: PathBuf = if dir.is_file() {
        dir.clone()
    } else {
        dir.join("intake.sqlite")
    };
    let redb_path: PathBuf = if dir.is_file() {
        dir.parent().unwrap_or(Path::new(".")).join("cochranblock.redb")
    } else {
        dir.join("cochranblock.redb")
    };

    println!("source : {}", sqlite_path.display());
    println!("target : {}", redb_path.display());

    if !sqlite_path.exists() {
        println!("source sqlite not found — nothing to migrate (writing empty redb)");
        if let Some(parent) = redb_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let db = Database::create(&redb_path)?;
        touch_tables(&db)?;
        println!("done.");
        return Ok(());
    }

    if let Some(parent) = redb_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open_with_flags(
        &sqlite_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;
    let db = Database::create(&redb_path)?;
    touch_tables(&db)?;

    let leads = migrate_leads(&conn, &db)?;
    let grants = migrate_grants(&conn, &db)?;
    let knox = migrate_knox(&conn, &db)?;

    println!("migrated  leads={} grants={} knox={}", leads, grants, knox);

    let lc = count(&db, T_LEADS)?;
    let gc = count(&db, T_GRANTS)?;
    let kc = count(&db, T_KNOX)?;
    println!("redb now  leads={} grants={} knox={}", lc, gc, kc);

    Ok(())
}
