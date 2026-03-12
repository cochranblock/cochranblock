// Unlicense — cochranblock.org
// Community Grant: eligibility-audit questionnaire, reuses intake pattern.
// Unlisted page — no nav link.

use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use chrono::Utc;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

use crate::t0;
use super::pages::{f62, C7, C8};

const INTRO: &str = include_str!("../../content/community_grant_intro.txt");

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

/// GET /community-grant — eligibility audit form.
pub async fn get_form(State(_s): State<Arc<t0>>) -> Html<String> {
    let intro_escaped = html_escape(INTRO);
    let content = format!(
        r#"<section class="intake-section"><div class="intake-steps"><span class="intake-step intake-active"><span class="intake-num">1</span> Review</span><span class="intake-step"><span class="intake-num">2</span> Submit</span><span class="intake-step"><span class="intake-num">3</span> Complete</span></div>
<div class="intake-doc"><h1 class="intake-title">The Cochranblock Community Grant Application</h1><p class="intake-intro">{}</p>
<div class="intake-sign"><div class="intake-line"></div><p class="intake-sign-label">Submit your application</p>
<form class="intake-form" method="post" action="/community-grant" id="grant-form">
<label for="org_name">Organization Name</label><input type="text" id="org_name" name="org_name" required autocomplete="organization" placeholder="Your organization name" maxlength="200">
<label for="ein">501(c)(3) EIN or Tax ID (If applicable)</label><input type="text" id="ein" name="ein" autocomplete="off" placeholder="XX-XXXXXXX" maxlength="20">
<label for="contact_name">Point of Contact — Name</label><input type="text" id="contact_name" name="contact_name" required autocomplete="name" placeholder="Full name" maxlength="200">
<label for="contact_email">Point of Contact — Email</label><input type="email" id="contact_email" name="contact_email" required autocomplete="email" placeholder="contact@org.org" maxlength="254">
<label for="mission">The Mission</label><textarea id="mission" name="mission" required placeholder="Briefly describe your organization's primary mission. How specifically does your work serve and elevate the local community?" maxlength="2000"></textarea>
<label for="technical_objective">Technical Objective</label><textarea id="technical_objective" name="technical_objective" required placeholder="What will this web appliance be used for? (e.g., Volunteer intake routing, public information broadcasting, donation processing parameters)" maxlength="2000"></textarea>
<div class="intake-hp" aria-hidden="true"><label for="website_url">Leave blank</label><input type="text" id="website_url" name="website_url" tabindex="-1" autocomplete="off"></div>
<div class="intake-consent"><label class="intake-check"><input type="checkbox" name="consent_grant" value="1" required id="consent_grant"> I acknowledge the hardware requirements and the $500 deployment fee should our application be selected for this quarter's grant.</label></div>
<button type="submit" class="btn" id="grant-submit-btn" disabled>Submit Application</button></form></div>
<p class="intake-note">If awarded, this grant subsidizes 85% of our standard deployment costs. One deployment per fiscal quarter.</p></div></section>
<script>(function(){{var f=document.getElementById('grant-form');var b=document.getElementById('grant-submit-btn');var c=document.getElementById('consent_grant');var o=document.getElementById('org_name');var n=document.getElementById('contact_name');var e=document.getElementById('contact_email');var m=document.getElementById('mission');var t=document.getElementById('technical_objective');function chk(){{var ok=c&&o&&n&&e&&m&&t&&c.checked&&o.value.trim()&&n.value.trim()&&e.value.trim()&&m.value.trim()&&t.value.trim();b.disabled=!ok;}}if(f){{c.onchange=chk;o.oninput=o.onchange=n.oninput=n.onchange=e.oninput=e.onchange=m.oninput=m.onchange=t.oninput=t.onchange=chk;chk();}}}})();</script>"#,
        intro_escaped
    );
    Html(format!(
        "{}{}{}{}",
        f62("community-grant", "Community Grant | CochranBlock"),
        C7,
        content,
        C8
    ))
}

#[derive(Deserialize)]
pub struct CommunityGrantForm {
    org_name: String,
    #[serde(default)]
    ein: String,
    contact_name: String,
    contact_email: String,
    mission: String,
    technical_objective: String,
    #[serde(rename = "website_url")]
    honeypot: Option<String>,
    consent_grant: Option<String>,
}

fn validate_grant_input(
    org_name: &str,
    contact_name: &str,
    contact_email: &str,
    mission: &str,
    technical_objective: &str,
) -> Result<(), &'static str> {
    if org_name.trim().is_empty() {
        return Err("org name empty");
    }
    if contact_name.trim().is_empty() {
        return Err("contact name empty");
    }
    if contact_email.trim().is_empty() {
        return Err("contact email empty");
    }
    if mission.trim().is_empty() {
        return Err("mission empty");
    }
    if technical_objective.trim().is_empty() {
        return Err("technical objective empty");
    }
    if org_name.len() > 200 || contact_name.len() > 200 || contact_email.len() > 254 {
        return Err("field too long");
    }
    if mission.len() > 2000 || technical_objective.len() > 2000 {
        return Err("field too long");
    }
    Ok(())
}

/// POST /community-grant — process form.
pub async fn post_form(
    State(s): State<Arc<t0>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(f): Form<CommunityGrantForm>,
) -> impl IntoResponse {
    if f.honeypot.as_deref().map(|v| !v.trim().is_empty()).unwrap_or(false) {
        tracing::debug!("community grant honeypot triggered");
        return (StatusCode::OK, Html(confirmed_html(None))).into_response();
    }

    let org_name = f.org_name.trim();
    let ein = f.ein.trim();
    let contact_name = f.contact_name.trim();
    let contact_email = f.contact_email.trim();
    let mission = f.mission.trim();
    let technical_objective = f.technical_objective.trim();

    if validate_grant_input(org_name, contact_name, contact_email, mission, technical_objective)
        .is_err()
    {
        return (StatusCode::BAD_REQUEST, "Invalid input").into_response();
    }

    if f.consent_grant.as_deref() != Some("1") {
        return (
            StatusCode::BAD_REQUEST,
            "You must acknowledge the hardware requirements and $500 deployment fee.",
        )
            .into_response();
    }

    let pool = match &s.intake_pool {
        Some(p) => p.clone(),
        None => {
            tracing::error!("community grant: pool not available");
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
        INSERT INTO community_grants (id, org_name, ein, contact_name, contact_email, mission, technical_objective, submitted_at, ip_address, user_agent, consent_grant)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1)
        "#,
    )
    .bind(&id)
    .bind(org_name)
    .bind(if ein.is_empty() { None::<&str> } else { Some(ein) })
    .bind(contact_name)
    .bind(contact_email)
    .bind(mission)
    .bind(technical_objective)
    .bind(&submitted_at)
    .bind(&ip)
    .bind(&ua)
    .execute(&pool)
    .await
    {
        tracing::error!("community grant insert failed: {}", e);
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
                "type": "community_grant",
                "id": id.clone(),
                "org_name": org_name,
                "ein": if ein.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(ein.to_string()) },
                "contact_name": contact_name,
                "contact_email": contact_email,
                "mission": mission,
                "technical_objective": technical_objective,
                "submitted_at": submitted_at,
            });
            let grant_id = id.clone();
            tokio::spawn(async move {
                if let Err(e) = client
                    .post(url.trim())
                    .json(&payload)
                    .timeout(std::time::Duration::from_secs(10))
                    .send()
                    .await
                {
                    tracing::warn!("community grant webhook failed: {}", e);
                } else {
                    tracing::info!("community grant webhook sent for {}", grant_id);
                }
            });
        }
    }

    let loc = format!("/community-grant/confirmed?ref={}", urlencoding::encode(&id));
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
        r#"<section class="intake-section intake-done"><div class="intake-doc intake-complete"><div class="intake-check-icon">✓</div><h1>Application Received</h1><p class="intake-success">Your Community Grant application has been submitted.</p>{}<p class="intake-detail">Cochranblock will review applications and contact selected applicants within 2–3 business days of the quarter close.</p><a href="/community-grant" class="btn">Done</a></div></section>"#,
        ref_line
    );

    format!(
        "{}{}{}{}",
        f62("community-grant-confirmed", "Application Received | CochranBlock"),
        C7,
        content,
        C8
    )
}

/// GET /community-grant/confirmed
pub async fn confirmed(
    Query(q): Query<std::collections::HashMap<String, String>>,
) -> Html<String> {
    let ref_id = q.get("ref").map(|s| s.as_str());
    Html(confirmed_html(ref_id))
}
