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

/// GET /deploy — hacker terminal intake. Interactive CLI-style form.
pub async fn get_form(State(s): State<Arc<t0>>) -> Html<String> {
    let terms_escaped = html_escape(TERMS);
    let content = format!(
        r#"<section class="intake-section">

<div class="crt-monitor">
<div class="crt-bezel">
<div class="crt-screen booting">
<div class="crt-glow"></div>
<div class="crt-glass"></div>
<div class="term-wrap">
<div class="term-header"><span class="term-dot term-red"></span><span class="term-dot term-yellow"></span><span class="term-dot term-green"></span><span class="term-title">cochranblock@deploy:~$</span></div>
<div class="term-body" id="terminal">
<div class="term-output" id="term-out"></div>
<div class="term-input-row" id="term-input-row">
<span class="term-prompt">$</span>
<input type="text" class="term-input" id="term-in" autocomplete="off" spellcheck="false" autofocus>
</div>
</div>
</div>
</div>
<span class="crt-brand">CochranBlock</span>
<span class="crt-power"></span>
</div>
</div>

<form method="post" action="/deploy" id="intake-form" style="display:none">
<input type="hidden" name="deploy_class" id="deploy_class" value="">
<input type="hidden" name="full_name" id="full_name" value="">
<input type="hidden" name="email" id="email" value="">
<input type="hidden" name="company" id="company" value="">
<input type="hidden" name="message" id="message" value="">
<input type="hidden" name="website_url" id="website_url" value="">
<input type="hidden" name="consent_fee" id="consent_fee" value="">
<input type="hidden" name="consent_hardware" id="consent_hardware" value="">
<input type="hidden" name="consent_terms" id="consent_terms" value="">
</form>

<details id="terms-drawer" class="deploy-terms-drawer" style="margin-top:1rem">
<summary style="color:#00d9ff;cursor:pointer;font-family:'JetBrains Mono',monospace;font-size:0.85rem">cat terms.txt</summary>
<div class="intake-body"><pre>{}</pre></div>
</details>

<script>
(function(){{
var out=document.getElementById('term-out');
var inp=document.getElementById('term-in');
var step=0;
var answers={{}};
var bootLines=[
'',
'  ######   #######   ######  ##   ## ########     ###    ##   ##',
'  ##   ## ##     ## ##    ## ##   ## ##     ##   ## ##  ####  ##',
'  ##      ##     ## ##       ####### ########  ##   ## ## ## ##',
'  ##   ## ##     ## ##    ## ##   ## ##   ##  ######## ##  ####',
'  ######   #######   ######  ##   ## ##    ## ##   ## ##   ###',
'           ########  ##       #######   ######  ##   ##',
'           ##     ## ##      ##     ## ##    ## ##  ##',
'           ########  ##      ##     ## ##       #####',
'           ##     ## ##      ##     ## ##    ## ##  ##',
'           ########  ######## #######   ######  ##   ##',
'',
'  DEPLOY SYSTEM v0.3.0',
'  ==========================================',
'',
'  [BIOS] Memory check............ 64GB OK',
'  [BIOS] Rust toolchain.......... 1.94.0 OK',
'  [BIOS] Identity................ VERIFIED',
'  [BIOS] Cloudflare tunnel....... CONNECTED',
'  [BIOS] Encryption.............. AES-256-GCM',
'  [BIOS] Honeypot................ ARMED',
'',
'  Booting deploy interface...',
'',
];
var steps=[
  {{q:'Connection established. System ready.\n\nThree deployment classes available:\n\n  [1] PRODUCT     $3,500 base. Your hardware. $0/mo forever.\n  [2] CONSULTING  Build it, harden it, fix it. Project-based.\n  [3] PARTNERSHIP Your brand + our engine. Long-term.\n\nSelect class (1/2/3):',field:'deploy_class',validate:function(v){{return ['1','2','3'].indexOf(v)!==-1}},transform:function(v){{return ['','product','consulting','partnership'][parseInt(v)]}}}},
  {{q:'Enter your name:',field:'full_name',validate:function(v){{return v.trim().length>0}}}},
  {{q:'Enter your email:',field:'email',validate:function(v){{return v.trim().length>0&&v.indexOf('@')>0}}}},
  {{q:'Company or project name (enter to skip):',field:'company',validate:function(){{return true}}}},
  {{q:'Describe your mission (what are you building?):',field:'message',validate:function(){{return true}}}},
  {{q:'Pricing acknowledged? You understand the model for your class. (y/n):',field:'consent_fee',validate:function(v){{return v.toLowerCase()==='y'||v.toLowerCase()==='n'}},transform:function(v){{return v.toLowerCase()==='y'?'1':''}}}},
  {{q:'Hardware model: deployments run on YOUR local hardware via Cloudflare Zero Trust.\nNo cloud hosting. No monthly fees. Acknowledged? (y/n):',field:'consent_hardware',validate:function(v){{return v.toLowerCase()==='y'||v.toLowerCase()==='n'}},transform:function(v){{return v.toLowerCase()==='y'?'1':''}}}},
  {{q:'Terms accepted? (type "cat terms" to read, or y/n):',field:'consent_terms',validate:function(v){{var l=v.toLowerCase();return l==='y'||l==='n'||l==='cat terms'}},transform:function(v){{return v.toLowerCase()==='y'?'1':''}}}}
];

function typeOut(text,cb){{
  var lines=text.split('\n');
  var i=0;
  function nextLine(){{
    if(i>=lines.length){{if(cb)cb();return;}}
    var div=document.createElement('div');
    div.className='term-line';
    div.textContent=lines[i];
    out.appendChild(div);
    out.scrollTop=out.scrollHeight;
    i++;
    setTimeout(nextLine,40);
  }}
  nextLine();
}}

function showPrompt(){{
  if(step>=steps.length){{
    // All consent checks passed?
    var f=document.getElementById('intake-form');
    var fee=document.getElementById('consent_fee').value;
    var hw=document.getElementById('consent_hardware').value;
    var terms=document.getElementById('consent_terms').value;
    if(fee==='1'&&hw==='1'&&terms==='1'){{
      typeOut('\n[DEPLOY] All systems green. Transmitting request...\n[DEPLOY] Quest submitted. Stand by for contact within 48 hours.\n',function(){{
        f.submit();
      }});
    }}else{{
      typeOut('\n[ABORT] All three acknowledgments required to deploy.\n[ABORT] Type "restart" to try again.\n',null);
      step=-1;
    }}
    return;
  }}
  typeOut('\n'+steps[step].q+'\n',function(){{
    inp.focus();
  }});
}}

inp.addEventListener('keydown',function(ev){{
  if(ev.key!=='Enter')return;
  var val=inp.value;
  inp.value='';

  var echo=document.createElement('div');
  echo.className='term-line term-echo';
  echo.textContent='$ '+val;
  out.appendChild(echo);

  if(val.toLowerCase()==='restart'){{
    step=0;
    out.innerHTML='';
    showPrompt();
    return;
  }}

  if(val.toLowerCase()==='cat terms'){{
    document.getElementById('terms-drawer').open=true;
    typeOut('[TERMS] Scroll down to read terms.txt\n',function(){{
      showPrompt();
    }});
    return;
  }}

  if(step<0)return;
  if(step>=steps.length)return;

  var s=steps[step];
  if(!s.validate(val)){{
    typeOut('[ERROR] Invalid input. Try again.\n',function(){{inp.focus();}});
    return;
  }}

  var transformed=s.transform?s.transform(val):val;
  document.getElementById(s.field).value=transformed;
  answers[s.field]=transformed;

  if(s.field==='deploy_class'){{
    var names={{product:'PRODUCT DEPLOYMENT',consulting:'CONSULTING',partnership:'PARTNERSHIP'}};
    typeOut('[OK] Class selected: '+names[transformed]+'\n',null);
  }}

  step++;
  showPrompt();
}});

// Boot sequence then first prompt
var bootIdx=0;
function bootLine(){{
  if(bootIdx>=bootLines.length){{
    setTimeout(function(){{showPrompt();}},400);
    return;
  }}
  var div=document.createElement('div');
  div.className='term-line';
  div.textContent=bootLines[bootIdx];
  out.appendChild(div);
  out.scrollTop=out.scrollHeight;
  bootIdx++;
  var delay=bootLines[bootIdx-1].indexOf('[BIOS]')>=0?120:30;
  setTimeout(bootLine,delay);
}}
bootLine();
}})();
</script>
</section>"#,
        terms_escaped
    );
    let _ = s;
    Html(format!(
        "{}{}{}{}",
        f62("deploy", "Deploy | CochranBlock"),
        C7,
        content,
        C8
    ))
}

#[derive(Deserialize)]
pub struct IntakeForm {
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
        INSERT INTO leads (id, full_name, email, company, message, submitted_at, ip_address, user_agent, terms_version, consent_fee, consent_hardware, consent_terms)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 1, 1)
        "#,
    )
    .bind(&id)
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