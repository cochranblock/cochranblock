// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

// Client Intake: waiver-style form, honeypot, SQLite, async webhook.
// Themed to blend with cochranblock cosmic aesthetic.

use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

use crate::t0;
use super::pages::{f62, C7, C8};

const TERMS_VERSION: &str = "2026-03";
const TERMS: &str = include_str!("../../content/intake_terms.txt");

/// init_pool — create data dir, connect SQLite, migrate. Returns None on failure.
pub async fn init_pool() -> Option<sqlx::SqlitePool> {
    let data_dir = std::env::var("CB_DATA_DIR")
        .ok()
        .or_else(|| std::env::var("COCHRANBLOCK_DATA_DIR").ok())
        .or_else(|| {
            dirs::data_dir()
                .map(|p| p.join("cochranblock").join("data"))
                .and_then(|p| p.into_os_string().into_string().ok())
        })
        .unwrap_or_else(|| "data".into());
    let dir = std::path::Path::new(&data_dir);
    if let Err(e) = std::fs::create_dir_all(dir) {
        tracing::warn!("intake: could not create data dir {}: {}", data_dir, e);
        return None;
    }
    let db_path = dir.join("intake.sqlite");
    let url = format!("sqlite:{}?mode=rwc", db_path.display());
    let pool = match sqlx::sqlite::SqlitePool::connect(&url).await {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!("intake: could not connect to sqlite: {}", e);
            return None;
        }
    };
    if let Err(e) = sqlx::query("PRAGMA journal_mode=WAL").execute(&pool).await {
        tracing::warn!("intake: pragma failed: {}", e);
    }
    if let Err(e) = migrate(&pool).await {
        tracing::warn!("intake: migrate failed: {}", e);
        return None;
    }
    tracing::info!("intake: sqlite ready at {}", db_path.display());
    Some(pool)
}

async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS leads (
            id TEXT PRIMARY KEY,
            deploy_class TEXT,
            full_name TEXT NOT NULL,
            email TEXT NOT NULL,
            company TEXT,
            message TEXT,
            submitted_at TEXT NOT NULL,
            ip_address TEXT,
            user_agent TEXT,
            terms_version TEXT NOT NULL,
            consent_fee INTEGER NOT NULL,
            consent_hardware INTEGER NOT NULL,
            consent_terms INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS community_grants (
            id TEXT PRIMARY KEY,
            org_name TEXT NOT NULL,
            ein TEXT,
            contact_name TEXT NOT NULL,
            contact_email TEXT NOT NULL,
            mission TEXT NOT NULL,
            technical_objective TEXT NOT NULL,
            submitted_at TEXT NOT NULL,
            ip_address TEXT,
            user_agent TEXT,
            consent_grant INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn client_ip(addr: SocketAddr, headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

/// GET /deploy — deployment request intake form.
pub async fn get_form(State(s): State<Arc<t0>>) -> Html<String> {
    let terms_escaped = html_escape(TERMS);
    let content = format!(
        r#"<section class="intake-section"><div class="intake-steps"><span class="intake-step intake-active"><span class="intake-num">1</span> Review</span><span class="intake-step"><span class="intake-num">2</span> Submit</span><span class="intake-step"><span class="intake-num">3</span> Complete</span></div>
<div class="intake-doc"><h1 class="intake-title">Deploy With CochranBlock</h1><p class="intake-intro">Request a custom Web Appliance — a compiled Rust binary that runs on your hardware, routed via Cloudflare Zero Trust. No cloud hosting. No monthly fees. $3,500 base.</p>
<div class="intake-body"><pre>{}</pre></div>
<div class="intake-sign"><div class="intake-line"></div><p class="intake-sign-label">Submit your deployment request</p>
<form class="intake-form" method="post" action="/deploy" id="intake-form">
<label for="deploy_class">Deployment Class</label>
<select id="deploy_class" name="deploy_class" required>
<option value="" disabled selected>Select a class</option>
<option value="product">Product — $3,500 base. Your hardware. $0/mo forever.</option>
<option value="consulting">Consulting — Build it, harden it, fix it. Project-based.</option>
<option value="partnership">Partnership — Your brand + our engine. Long-term.</option>
</select>
<label for="full_name">Full Name</label><input type="text" id="full_name" name="full_name" required autocomplete="name" placeholder="Your name" maxlength="200">
<label for="email">Email</label><input type="email" id="email" name="email" required autocomplete="email" placeholder="you@company.com" maxlength="254">
<label for="company">Company or Project Name</label><input type="text" id="company" name="company" autocomplete="organization" placeholder="Optional" maxlength="200">
<label for="message">What are you building?</label><textarea id="message" name="message" placeholder="Describe your project or mission" maxlength="2000"></textarea>
<div class="intake-hp" aria-hidden="true"><label for="website_url">Leave blank</label><input type="text" id="website_url" name="website_url" tabindex="-1" autocomplete="off"></div>
<div class="intake-consent">
<label class="intake-check"><input type="checkbox" name="consent_fee" value="1" required id="consent_fee"> I acknowledge the baseline fee of $3,500 USD for a custom Web Appliance deployment.</label>
<label class="intake-check"><input type="checkbox" name="consent_hardware" value="1" required id="consent_hardware"> I understand deployments run on my own local hardware via Cloudflare Zero Trust — no cloud hosting, no monthly fees.</label>
<label class="intake-check"><input type="checkbox" name="consent_terms" value="1" required id="consent_terms"> I have read the terms above and am submitting this request voluntarily.</label>
</div>
<button type="submit" class="btn" id="deploy-submit-btn" disabled>Submit Request</button></form></div>
<p class="intake-note">Upon submission, CochranBlock will review your request and contact you within 2–3 business days to schedule a discovery call. No obligation is incurred by submitting this form.</p></div></section>
<script>(function(){{var f=document.getElementById('intake-form');var b=document.getElementById('deploy-submit-btn');var dc=document.getElementById('deploy_class');var fn=document.getElementById('full_name');var em=document.getElementById('email');var cf=document.getElementById('consent_fee');var ch=document.getElementById('consent_hardware');var ct=document.getElementById('consent_terms');function chk(){{var ok=dc.value&&fn.value.trim()&&em.value.trim()&&cf.checked&&ch.checked&&ct.checked;b.disabled=!ok;}}dc.onchange=chk;fn.oninput=fn.onchange=em.oninput=em.onchange=chk;cf.onchange=ch.onchange=ct.onchange=chk;chk();}})();</script>"#,
        terms_escaped
    );
    let _ = s;
    Html(format!(
        "{}{}{}{}",
        f62("deploy", "Deploy With Us | CochranBlock"),
        C7,
        content,
        C8
    ))
}

#[derive(Deserialize)]
pub struct IntakeForm {
    #[serde(default)]
    deploy_class: String,
    full_name: String,
    email: String,
    #[serde(default)]
    company: String,
    #[serde(default)]
    message: String,
    #[serde(rename = "website_url")]
    honeypot: Option<String>,
    consent_fee: Option<String>,
    consent_hardware: Option<String>,
    consent_terms: Option<String>,
}

fn validate_input(full_name: &str, email: &str) -> Result<(), &'static str> {
    let n = full_name.trim();
    let e = email.trim();
    if n.is_empty() {
        return Err("name empty");
    }
    if e.is_empty() {
        return Err("email empty");
    }
    if n.len() > 200 {
        return Err("name too long");
    }
    if e.len() > 254 {
        return Err("email too long");
    }
    Ok(())
}

/// POST /intake — process form.
pub async fn post_form(
    State(s): State<Arc<t0>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(f): Form<IntakeForm>,
) -> impl IntoResponse {
    if f.honeypot.as_deref().map(|v| !v.trim().is_empty()).unwrap_or(false) {
        tracing::debug!("intake honeypot triggered");
        return (StatusCode::OK, Html(confirmed_html(None))).into_response();
    }

    let deploy_class = f.deploy_class.trim();
    let full_name = f.full_name.trim();
    let email = f.email.trim();
    let company = f.company.trim();
    let message = f.message.trim();

    if validate_input(full_name, email).is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid input").into_response();
    }

    if f.consent_fee.as_deref() != Some("1")
        || f.consent_hardware.as_deref() != Some("1")
        || f.consent_terms.as_deref() != Some("1")
    {
        return (
            StatusCode::BAD_REQUEST,
            "You must acknowledge the fee, hardware model, and terms.",
        )
            .into_response();
    }

    let pool = match &s.intake_pool {
        Some(p) => p.clone(),
        None => {
            tracing::error!("intake pool not available");
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service temporarily unavailable. Please try again later.",
            )
                .into_response();
        }
    };

    let id = Uuid::new_v4().to_string();
    let submitted_at = Utc::now().to_rfc3339();
    let ip = client_ip(addr, &headers);
    let ua = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO leads (id, deploy_class, full_name, email, company, message, submitted_at, ip_address, user_agent, terms_version, consent_fee, consent_hardware, consent_terms)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 1, 1)
        "#,
    )
    .bind(&id)
    .bind(if deploy_class.is_empty() { None::<&str> } else { Some(deploy_class) })
    .bind(full_name)
    .bind(email)
    .bind(if company.is_empty() { None::<&str> } else { Some(company) })
    .bind(if message.is_empty() { None::<&str> } else { Some(message) })
    .bind(&submitted_at)
    .bind(&ip)
    .bind(&ua)
    .bind(TERMS_VERSION)
    .execute(&pool)
    .await
    {
        tracing::error!("lead insert failed: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to save. Please try again.",
        )
            .into_response();
    }

    if let Ok(url) = std::env::var("INTAKE_WEBHOOK_URL") {
        if !url.trim().is_empty() {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "id": id.clone(),
                "deploy_class": if deploy_class.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(deploy_class.to_string()) },
                "full_name": full_name,
                "email": email,
                "company": if company.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(company.to_string()) },
                "message": if message.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(message.to_string()) },
                "submitted_at": submitted_at,
            });
            let lead_id = id.clone();
            tokio::spawn(async move {
                if let Err(e) = client
                    .post(url.trim())
                    .json(&payload)
                    .timeout(std::time::Duration::from_secs(10))
                    .send()
                    .await
                {
                    tracing::warn!("intake webhook failed: {}", e);
                } else {
                    tracing::info!("intake webhook sent for lead {}", lead_id);
                }
            });
        }
    }

    let loc = format!("/deploy/confirmed?ref={}", urlencoding::encode(&id));
    Redirect::temporary(loc.as_str()).into_response()
}

fn confirmed_html(ref_id: Option<&str>) -> String {
    let ref_line = ref_id
        .filter(|s| !s.is_empty())
        .map(|r| format!(
            r#"<p class="intake-detail">Reference ID: <code>{}</code></p>"#,
            html_escape(r)
        ))
        .unwrap_or_default();

    let content = format!(
        r#"<section class="intake-section intake-done"><div class="intake-doc intake-complete"><div class="intake-check-icon">✓</div><h1>Request Received</h1><p class="intake-success">Your deployment request has been submitted.</p>{}<p class="intake-detail">Cochranblock will contact you within 2–3 business days to schedule a discovery call.</p><a href="/intake" class="btn">Done</a></div></section>"#,
        ref_line
    );

    format!(
        "{}{}{}{}",
        f62("deploy-confirmed", "Quest Accepted | CochranBlock"),
        C7,
        content,
        C8
    )
}

/// GET /intake/confirmed
pub async fn confirmed(
    Query(q): Query<std::collections::HashMap<String, String>>,
) -> Html<String> {
    let ref_id = q.get("ref").map(|s| s.as_str());
    Html(confirmed_html(ref_id))
}