// All Rights Reserved — The Cochran Block, LLC
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

// Client Intake: waiver-style form, honeypot, SQLite, async webhook.
// Themed to blend with cochranblock cosmic aesthetic.

use axum::Form;
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

use super::pages::{C7, C8, f62};
use crate::t0;

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
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS knoxai_applicants (
            id TEXT PRIMARY KEY,
            full_name TEXT NOT NULL,
            email TEXT NOT NULL,
            background TEXT NOT NULL,
            specialty_tags TEXT NOT NULL,
            clearance TEXT NOT NULL,
            motivation TEXT NOT NULL,
            acknowledge_csam INTEGER NOT NULL,
            submitted_at TEXT NOT NULL,
            ip_address TEXT,
            user_agent TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub(crate) fn client_ip(addr: SocketAddr, headers: &HeaderMap) -> String {
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

pub(crate) fn validate_input(full_name: &str, email: &str) -> Result<(), &'static str> {
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
    if f.honeypot
        .as_deref()
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
    {
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

    if let Some(url) = std::env::var("INTAKE_WEBHOOK_URL")
        .ok()
        .map(|u| u.trim().to_string())
        .filter(|u| !u.is_empty())
    {
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

    let loc = format!("/deploy/confirmed?ref={}", urlencoding::encode(&id));
    Redirect::temporary(loc.as_str()).into_response()
}

fn confirmed_html(ref_id: Option<&str>) -> String {
    let ref_line = ref_id
        .filter(|s| !s.is_empty())
        .map(|r| {
            format!(
                r#"<p class="intake-detail">Reference ID: <code>{}</code></p>"#,
                html_escape(r)
            )
        })
        .unwrap_or_default();

    let content = format!(
        r#"<section class="intake-section intake-done"><div class="intake-doc intake-complete"><div class="intake-check-icon">✓</div><h1>Request Received</h1><p class="intake-success">Your deployment request has been submitted.</p>{}<p class="intake-detail">Cochranblock will contact you within 2–3 business days to schedule a discovery call.</p><div class="rocket-pad"><button class="btn rocket-launch-btn" onclick="launchRocket(this)">Launch 🚀</button><div class="rocket-ship" id="rocket-ship">🚀</div></div><a href="/intake" class="btn btn-secondary" style="margin-top:1rem">Done</a></div></section>
<style>
.rocket-pad{{position:relative;margin:2rem 0 0;text-align:center;min-height:60px}}
.rocket-ship{{position:fixed;bottom:-60px;left:50%;font-size:2.5rem;opacity:0;pointer-events:none;z-index:50;transform:translateX(-50%);filter:drop-shadow(0 0 12px rgba(255,107,53,0.6))}}
.rocket-ship.countdown{{opacity:1;bottom:20%;animation:rocketShake 0.15s ease-in-out infinite alternate}}
.rocket-ship.launched{{opacity:1;animation:rocketLaunch 2s ease-in forwards}}
.rocket-exhaust{{position:fixed;bottom:0;left:50%;width:4px;height:0;background:linear-gradient(to top,rgba(255,107,53,0.8),rgba(255,200,50,0.4),transparent);transform:translateX(-50%);z-index:49;pointer-events:none;border-radius:2px}}
.rocket-exhaust.active{{animation:exhaustTrail 2s ease-in forwards}}
.rocket-launch-btn.counting{{pointer-events:none;background:var(--orange);border-color:var(--orange)}}
@keyframes rocketShake{{0%{{transform:translateX(-50%) rotate(-2deg)}}100%{{transform:translateX(-50%) rotate(2deg)}}}}
@keyframes rocketLaunch{{0%{{bottom:20%;opacity:1;transform:translateX(-50%) rotate(0)}}30%{{bottom:40%}}100%{{bottom:120vh;opacity:0.3;transform:translateX(-50%) rotate(0)}}}}
@keyframes exhaustTrail{{0%{{height:0;opacity:0.8}}30%{{height:40vh;opacity:0.6}}100%{{height:0;opacity:0}}}}
</style>
<script>
function launchRocket(btn){{var r=document.getElementById('rocket-ship');if(r.classList.contains('launched'))return;btn.classList.add('counting');var c=3;btn.textContent=c;r.style.opacity='1';r.style.bottom='20%';r.classList.add('countdown');var ex=document.createElement('div');ex.className='rocket-exhaust';document.body.appendChild(ex);var t=setInterval(function(){{c--;if(c>0){{btn.textContent=c}}else{{clearInterval(t);btn.textContent='Launched!';r.classList.remove('countdown');r.classList.add('launched');ex.classList.add('active');setTimeout(function(){{btn.classList.remove('counting');btn.textContent='Launch \u{{1F680}}';r.classList.remove('launched');r.style.opacity='0';r.style.bottom='-60px';ex.remove()}},3000)}}}},800)}}
</script>"#,
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
pub async fn confirmed(Query(q): Query<std::collections::HashMap<String, String>>) -> Html<String> {
    let ref_id = q.get("ref").map(|s| s.as_str());
    Html(confirmed_html(ref_id))
}

// ═══════════════════════════════════════════════════════════════
// KNOXAI Operator Registration
// ═══════════════════════════════════════════════════════════════

/// GET /knox/apply — operator application form
pub async fn knox_apply_form(State(_s): State<Arc<t0>>) -> Html<String> {
    let head = super::pages::f62d(
        "knox-apply",
        "KNOXAI — Operator Application",
        "Apply to join the KNOXAI operator guild. AI model certification for CSAM and harmful content detection.",
    );
    let content = r#"<section style="max-width:820px;margin:0 auto;padding:3rem 1.5rem 6rem;font-family:'JetBrains Mono','SF Mono',Consolas,monospace;font-size:14px;line-height:1.65;color:#fcfcfa">

<div style="font-size:0.6rem;letter-spacing:0.5em;color:#ff6188;text-transform:uppercase;text-align:center;padding:0.5rem 0;margin-bottom:2rem;border-top:1px solid #ff6188;border-bottom:1px solid #ff6188;opacity:0.7">Operator Application · KNOXAI</div>

<h1 style="font-size:2rem;font-weight:900;letter-spacing:0.12em;margin-bottom:0.8rem">KNOXAI</h1>
<div style="font-size:0.85rem;letter-spacing:0.25em;color:#ffd866;margin-bottom:2rem;text-transform:uppercase">Operator Application</div>

<div style="background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);border-left:3px solid #ff6188;padding:1.2rem;border-radius:4px;margin-bottom:2rem">
<div style="font-size:0.7rem;letter-spacing:0.25em;text-transform:uppercase;font-weight:700;color:#ff6188;margin-bottom:0.8rem">Before You Apply — Read This</div>
<p style="color:#c1c0c0;margin-bottom:0.8rem">KNOXAI operators audit AI models for <strong style="color:#fcfcfa">child sexual abuse material (CSAM)</strong> and other harmful content. This work exists because AI-generated CSAM increased <strong style="color:#ff6188">260x in one year</strong> (Internet Watch Foundation, 2025).</p>
<p style="color:#c1c0c0;margin-bottom:0.8rem"><strong style="color:#fcfcfa">What you will encounter:</strong> You will run automated detection gates against AI models. Gate 1 (hash scan) compares model outputs against known-bad hash databases. You will <strong style="color:#fcfcfa">never view actual CSAM</strong> — the detection pipeline uses perceptual hashes, membership inference scores, and classifier outputs. You see numbers and pass/fail verdicts, not images.</p>
<p style="color:#c1c0c0;margin-bottom:0.8rem"><strong style="color:#fcfcfa">What triggers mandatory reporting:</strong> If Gate 1 produces a hash match against the NCMEC database, the platform automatically files a CyberTipline report under <strong>18 USC §2258A</strong>. You sign the emergency blacklist cert (L thumb). The publisher is frozen. This is federal law — there is no discretion.</p>
<p style="color:#c1c0c0;margin-bottom:0"><strong style="color:#fcfcfa">What you earn:</strong> 80% of Standard ($20), 70% of Operator ($500), 60% of Portfolio ($5K+), 50% of Gov tier. Paid within 24 hours via Stripe Connect. 1099-NEC at year end. You are an independent contractor, not an employee.</p>
</div>

<div style="background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);border-left:3px solid #a9dc76;padding:1.2rem;border-radius:4px;margin-bottom:2rem">
<div style="font-size:0.7rem;letter-spacing:0.25em;text-transform:uppercase;font-weight:700;color:#a9dc76;margin-bottom:0.8rem">What Makes a Good Operator</div>
<p style="color:#c1c0c0;margin-bottom:0">Red teamers. ML researchers. ML engineers. Data scientists. AI safety researchers. Veterans with cyber backgrounds. People with security clearances. You don't need all of these — you need at least one. And you need to care about protecting kids more than you care about a side gig.</p>
</div>

<form method="post" action="/knox/apply" style="margin-top:2rem">

<label style="display:block;font-size:0.72rem;letter-spacing:0.15em;text-transform:uppercase;color:#727072;margin-bottom:0.3rem;margin-top:1.5rem">Full Name *</label>
<input type="text" name="full_name" required maxlength="200" placeholder="Your real name" style="width:100%;background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);color:#fcfcfa;padding:0.6rem 0.8rem;font-family:inherit;font-size:0.85rem;border-radius:4px">

<label style="display:block;font-size:0.72rem;letter-spacing:0.15em;text-transform:uppercase;color:#727072;margin-bottom:0.3rem;margin-top:1.5rem">Email *</label>
<input type="email" name="email" required maxlength="254" placeholder="you@example.com" style="width:100%;background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);color:#fcfcfa;padding:0.6rem 0.8rem;font-family:inherit;font-size:0.85rem;border-radius:4px">

<label style="display:block;font-size:0.72rem;letter-spacing:0.15em;text-transform:uppercase;color:#727072;margin-bottom:0.3rem;margin-top:1.5rem">Background *</label>
<textarea name="background" required maxlength="2000" placeholder="Military service, current role, relevant experience, certifications (OSCP, GIAC, CISSP, etc.)" rows="4" style="width:100%;background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);color:#fcfcfa;padding:0.6rem 0.8rem;font-family:inherit;font-size:0.85rem;border-radius:4px;resize:vertical"></textarea>

<label style="display:block;font-size:0.72rem;letter-spacing:0.15em;text-transform:uppercase;color:#727072;margin-bottom:0.3rem;margin-top:1.5rem">Specialty Tags (check all that apply) *</label>
<div style="display:grid;grid-template-columns:repeat(2,1fr);gap:0.5rem;margin-bottom:0.5rem">
<label style="color:#c1c0c0;font-size:0.82rem"><input type="checkbox" name="tag_redteam" value="1"> <span style="color:#ff6188">redteam</span> — offensive cyber, adversarial prompting</label>
<label style="color:#c1c0c0;font-size:0.82rem"><input type="checkbox" name="tag_ml_research" value="1"> <span style="color:#78dce8">ml-research</span> — adversarial ML, membership inference</label>
<label style="color:#c1c0c0;font-size:0.82rem"><input type="checkbox" name="tag_ml_eng" value="1"> <span style="color:#a9dc76">ml-eng</span> — training pipelines, framework internals</label>
<label style="color:#c1c0c0;font-size:0.82rem"><input type="checkbox" name="tag_data" value="1"> <span style="color:#ffd866">data</span> — dataset provenance, corpus analysis</label>
<label style="color:#c1c0c0;font-size:0.82rem"><input type="checkbox" name="tag_safety" value="1"> <span style="color:#ab9df2">safety</span> — AI safety, eval design, classifiers</label>
<label style="color:#c1c0c0;font-size:0.82rem"><input type="checkbox" name="tag_cleared" value="1"> <span style="color:#727072">cleared</span> — active U.S. security clearance</label>
</div>

<label style="display:block;font-size:0.72rem;letter-spacing:0.15em;text-transform:uppercase;color:#727072;margin-bottom:0.3rem;margin-top:1.5rem">Clearance Status</label>
<select name="clearance" style="width:100%;background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);color:#fcfcfa;padding:0.6rem 0.8rem;font-family:inherit;font-size:0.85rem;border-radius:4px">
<option value="none">No active clearance</option>
<option value="secret">Secret</option>
<option value="ts">Top Secret</option>
<option value="ts-sci">TS/SCI</option>
<option value="expired">Expired (reactivatable)</option>
</select>

<label style="display:block;font-size:0.72rem;letter-spacing:0.15em;text-transform:uppercase;color:#727072;margin-bottom:0.3rem;margin-top:1.5rem">Why do you want to do this? *</label>
<textarea name="motivation" required maxlength="2000" placeholder="Be honest. We're not looking for a resume answer." rows="3" style="width:100%;background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);color:#fcfcfa;padding:0.6rem 0.8rem;font-family:inherit;font-size:0.85rem;border-radius:4px;resize:vertical"></textarea>

<div style="background:#2d2a2e;border:1px solid rgba(255,255,255,0.12);border-left:3px solid #ffd866;padding:1rem;border-radius:4px;margin-top:1.5rem">
<label style="color:#c1c0c0;font-size:0.85rem;display:flex;align-items:flex-start;gap:0.6rem">
<input type="checkbox" name="acknowledge_csam" value="1" required style="margin-top:0.3rem">
<span>I understand that KNOXAI operators certify AI models for CSAM and harmful content. I understand that hash-match findings trigger mandatory federal reporting under 18 USC §2258A. I understand that I will never view actual CSAM — the pipeline uses perceptual hashes and classifier scores. I want to help stop this.</span>
</label>
</div>

<div style="display:none"><input type="text" name="website" tabindex="-1" autocomplete="off"></div>

<button type="submit" style="margin-top:1.5rem;width:100%;background:rgba(255,216,102,0.15);color:#ffd866;border:1px solid #ffd866;padding:0.8rem;font-family:inherit;font-size:0.85rem;font-weight:700;letter-spacing:0.15em;text-transform:uppercase;border-radius:4px;cursor:pointer">Submit Application</button>

</form>

<p style="font-size:0.7rem;color:#727072;margin-top:1.5rem;text-align:center">Applications reviewed by Operator #0. Response within 48 hours.<br>The Cochran Block, LLC · CAGE 1CQ66 · All Rights Reserved</p>

</section>"#;
    Html([head.as_str(), C7, content, C8].concat())
}

#[derive(Deserialize)]
pub struct KnoxApplyForm {
    full_name: String,
    email: String,
    background: String,
    #[serde(default)]
    tag_redteam: Option<String>,
    #[serde(default)]
    tag_ml_research: Option<String>,
    #[serde(default)]
    tag_ml_eng: Option<String>,
    #[serde(default)]
    tag_data: Option<String>,
    #[serde(default)]
    tag_safety: Option<String>,
    #[serde(default)]
    tag_cleared: Option<String>,
    clearance: String,
    motivation: String,
    #[serde(default)]
    acknowledge_csam: Option<String>,
    #[serde(default)]
    website: Option<String>,
}

/// POST /knox/apply — process operator application
pub async fn knox_apply_submit(
    State(s): State<Arc<t0>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(f): Form<KnoxApplyForm>,
) -> impl IntoResponse {
    // Honeypot
    if f.website.as_deref().unwrap_or("").len() > 0 {
        return Redirect::temporary("/knox/apply/confirmed?ref=bot").into_response();
    }

    let id = Uuid::new_v4().to_string();
    let submitted_at = Utc::now().to_rfc3339();
    let ip = client_ip(addr, &headers);
    let ua = headers.get("user-agent").and_then(|v| v.to_str().ok()).unwrap_or("").to_string();

    let mut tags = Vec::new();
    if f.tag_redteam.is_some() { tags.push("redteam"); }
    if f.tag_ml_research.is_some() { tags.push("ml-research"); }
    if f.tag_ml_eng.is_some() { tags.push("ml-eng"); }
    if f.tag_data.is_some() { tags.push("data"); }
    if f.tag_safety.is_some() { tags.push("safety"); }
    if f.tag_cleared.is_some() { tags.push("cleared"); }
    let tags_str = tags.join(",");

    let ack = if f.acknowledge_csam.is_some() { 1 } else { 0 };

    if let Some(pool) = &s.intake_pool {
        let _ = sqlx::query(
            "INSERT INTO knoxai_applicants (id, full_name, email, background, specialty_tags, clearance, motivation, acknowledge_csam, submitted_at, ip_address, user_agent) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&f.full_name)
        .bind(&f.email)
        .bind(&f.background)
        .bind(&tags_str)
        .bind(&f.clearance)
        .bind(&f.motivation)
        .bind(ack)
        .bind(&submitted_at)
        .bind(&ip)
        .bind(&ua)
        .execute(pool)
        .await;
    }

    // Webhook notification
    if let Some(url) = std::env::var("INTAKE_WEBHOOK_URL")
        .ok()
        .filter(|u| !u.is_empty())
    {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "type": "knoxai_operator_application",
            "id": id,
            "full_name": f.full_name,
            "email": f.email,
            "background": f.background,
            "specialty_tags": tags_str,
            "clearance": f.clearance,
            "motivation": f.motivation,
            "submitted_at": submitted_at,
        });
        let aid = id.clone();
        tokio::spawn(async move {
            if let Err(e) = client.post(url.trim()).json(&payload).timeout(std::time::Duration::from_secs(10)).send().await {
                tracing::warn!("knoxai apply webhook failed: {}", e);
            } else {
                tracing::info!("knoxai apply webhook sent for {}", aid);
            }
        });
    }

    let loc = format!("/knox/apply/confirmed?ref={}", urlencoding::encode(&id));
    Redirect::temporary(loc.as_str()).into_response()
}

/// GET /knox/apply/confirmed
pub async fn knox_apply_confirmed(Query(q): Query<std::collections::HashMap<String, String>>) -> Html<String> {
    let ref_id = q.get("ref").map(|s| html_escape(s)).unwrap_or_default();
    let head = super::pages::f62d(
        "knox-apply-confirmed",
        "Application Received — KNOXAI",
        "Your KNOXAI operator application has been received.",
    );
    let content = format!(
        r#"<section style="max-width:820px;margin:0 auto;padding:3rem 1.5rem 6rem;font-family:'JetBrains Mono','SF Mono',Consolas,monospace;font-size:14px;line-height:1.65;color:#fcfcfa;text-align:center">
<div style="font-size:3rem;margin-bottom:1rem;color:#a9dc76">✓</div>
<h1 style="font-size:1.5rem;font-weight:900;letter-spacing:0.12em;margin-bottom:1rem">Application Received</h1>
<p style="color:#c1c0c0;margin-bottom:0.5rem">Reference: <code style="color:#ffd866">{}</code></p>
<p style="color:#c1c0c0;margin-bottom:2rem">Operator #0 will review your application within 48 hours.</p>
<p style="color:#727072;font-size:0.8rem">If accepted, you'll receive the full onboarding handbook and hardware signing device instructions.</p>
<a href="/knox" style="display:inline-block;margin-top:2rem;background:rgba(255,216,102,0.15);color:#ffd866;border:1px solid #ffd866;padding:0.6rem 1.5rem;font-family:inherit;font-size:0.8rem;font-weight:700;letter-spacing:0.15em;text-transform:uppercase;border-radius:4px;text-decoration:none">Back to KNOXAI</a>
</section>"#,
        ref_id
    );
    Html([head.as_str(), C7, &content, C8].concat())
}
