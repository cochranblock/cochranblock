#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// All Rights Reserved - The Cochran Block, LLC
//
// /book - discovery call request flow.
// Calendar picks date + time, inline form collects name/email/topic, POST
// triggers SMTP send via mailer.rs (Gmail SMTP, lettre). Confirmation page
// mirrors /deploy/confirmed.
//
// Required env (process or approuter/.env):
//   SMTP_HOST          (default smtp.gmail.com)
//   SMTP_PORT          (default 465)
//   SMTP_USER          mcochran@cochranblock.org
//   SMTP_PASS          16-char Gmail App Password
//   BOOKING_NOTIFY_TO  (default = SMTP_USER)

use axum::Form;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use chrono::{Datelike, Duration, Utc, Weekday};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

use super::mailer::{self, BookingEmail};
use super::pages::{C7, C8, f62};
use crate::t0;

#[derive(Serialize)]
pub struct TimeSlot {
    pub label: String,
}

#[derive(Serialize)]
pub struct DateSlot {
    pub date: String,
    pub day_name: String,
    pub date_label: String,
    pub times: Vec<TimeSlot>,
}

/// Generate weekday slots for the next 90 days, 8:00am-5:00pm EST, capped at 60 dates.
pub fn build_slots() -> Vec<DateSlot> {
    let today = Utc::now().with_timezone(&New_York).date_naive();
    let times = [
        "8:00am", "8:30am", "9:00am", "9:30am", "10:00am", "10:30am", "11:00am", "11:30am",
        "12:00pm", "12:30pm", "1:00pm", "1:30pm", "2:00pm", "2:30pm", "3:00pm", "3:30pm", "4:00pm",
        "4:30pm", "5:00pm",
    ];
    let mut date_slots = Vec::new();
    for day_offset in 0..90i64 {
        let date = today + Duration::days(day_offset);
        let wd = date.weekday();
        if wd == Weekday::Sat || wd == Weekday::Sun {
            continue;
        }
        let day_name = match wd {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            _ => continue,
        };
        let date_str = date.format("%b %d").to_string();
        let date_iso = date.format("%Y-%m-%d").to_string();
        let time_slots: Vec<TimeSlot> = times
            .iter()
            .map(|t| TimeSlot { label: format!("{} EST", t) })
            .collect();
        date_slots.push(DateSlot {
            date: date_iso,
            day_name: day_name.to_string(),
            date_label: date_str,
            times: time_slots,
        });
        if date_slots.len() >= 60 {
            break;
        }
    }
    date_slots
}

/// GET /book - calendar + inline form.
pub async fn get_form(State(_s): State<Arc<t0>>) -> Html<String> {
    let date_slots = build_slots();
    let slots_json = serde_json::to_string(&date_slots).unwrap_or_default();
    let slots_json_escaped = slots_json.replace('<', "\\u003c").replace('>', "\\u003e");
    let v0 = format!(
        r##"<section class="booking-page"><h1>Schedule a Discovery Call</h1><p class="booking-intro">30 minutes. Discuss your goals and how I can help. All times Eastern (EST).</p><p class="booking-context">Pick a date, then a time, then tell me what you want to talk about.</p><p class="booking-legend">Available weekdays, 8am-5pm EST.</p><p class="booking-hint" id="booking-hint">Select a date, then a time, then fill the form.</p><a href="#booking-calendar" class="skip-link skip-link-calendar">Skip to calendar</a><div class="booking-calendar-wrapper" id="booking-calendar" role="region" aria-label="Calendar - pick a date then a time" aria-describedby="booking-hint"><div class="booking-calendar-header"><div class="booking-month-row"><button type="button" class="booking-nav" id="booking-prev" aria-label="Previous month">&larr;</button><h3 class="booking-month" id="booking-month"></h3><button type="button" class="booking-nav" id="booking-next" aria-label="Next month">&rarr;</button><span class="booking-available-badge" id="booking-available-badge"></span></div></div><div class="booking-calendar-grid" id="booking-grid" role="grid" aria-label="Month view"></div></div><div class="booking-time-panel" id="booking-time-panel" aria-live="polite" hidden><h3 class="booking-time-heading" id="booking-time-heading"></h3><div class="booking-time-slots" id="booking-time-slots" role="group"></div></div>
<form class="intake-form booking-form" method="post" action="/book" id="booking-form" hidden>
<h3 class="booking-form-heading" id="booking-form-heading">Your details</h3>
<input type="hidden" name="date_iso" id="book_date_iso">
<input type="hidden" name="date_label" id="book_date_label">
<input type="hidden" name="time_label" id="book_time_label">
<p class="booking-summary" id="booking-summary"></p>
<label for="book_name">Full Name</label><input type="text" id="book_name" name="name" required autocomplete="name" placeholder="Your name" maxlength="200">
<label for="book_email">Email</label><input type="email" id="book_email" name="email" required autocomplete="email" placeholder="you@company.com" maxlength="254">
<label for="book_phone">Phone (optional)</label><input type="tel" id="book_phone" name="phone" autocomplete="tel" placeholder="Optional" maxlength="40">
<label for="book_topic">What do you want to talk about?</label><textarea id="book_topic" name="topic" required placeholder="One or two sentences" maxlength="2000"></textarea>
<div class="intake-hp" aria-hidden="true"><label for="book_website_url">Leave blank</label><input type="text" id="book_website_url" name="website_url" tabindex="-1" autocomplete="off"></div>
<button type="submit" class="btn" id="book-submit-btn">Send request</button></form>
<p class="booking-note">I will confirm within 24 hours.</p><p class="booking-fallback">None of these work? <a href="mailto:mcochran@cochranblock.org?subject=Discovery%20Call%20Request">Email me</a> to propose a time.</p><script type="application/json" id="booking-slots-data">{}</script><script src="/assets/js/booking.js"></script></section>"##,
        slots_json_escaped
    );
    Html(format!(
        "{}{}{}{}",
        f62("book", "Schedule a Call | CochranBlock"),
        C7,
        v0,
        C8
    ))
}

#[derive(Deserialize)]
pub struct BookingFormData {
    name: String,
    email: String,
    #[serde(default)]
    phone: String,
    #[serde(default)]
    topic: String,
    #[serde(default)]
    date_iso: String,
    #[serde(default)]
    date_label: String,
    #[serde(default)]
    time_label: String,
    #[serde(rename = "website_url")]
    honeypot: Option<String>,
}

fn validate(name: &str, email: &str, date_iso: &str, time_label: &str) -> Result<(), &'static str> {
    let n = name.trim();
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
    if date_iso.is_empty() || time_label.is_empty() {
        return Err("date or time missing");
    }
    Ok(())
}

fn client_ip(addr: SocketAddr, headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

/// POST /book - send via SMTP, redirect to /book/confirmed.
pub async fn post_form(
    State(_s): State<Arc<t0>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(f): Form<BookingFormData>,
) -> impl IntoResponse {
    if f.honeypot.as_deref().map(|v| !v.trim().is_empty()).unwrap_or(false) {
        tracing::debug!("booking honeypot triggered");
        return Redirect::temporary("/book/confirmed").into_response();
    }

    let name = f.name.trim().to_string();
    let email = f.email.trim().to_string();
    let phone = f.phone.trim().to_string();
    let topic = f.topic.trim().to_string();
    let date_iso = f.date_iso.trim().to_string();
    let date_label = f.date_label.trim().to_string();
    let time_label = f.time_label.trim().to_string();

    if validate(&name, &email, &date_iso, &time_label).is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid input").into_response();
    }

    let id = Uuid::new_v4().to_string();
    let ip = client_ip(addr, &headers);

    let display_date = if date_label.is_empty() { date_iso.clone() } else { date_label.clone() };

    let payload = BookingEmail {
        id: id.clone(),
        name,
        email,
        phone,
        date_iso: display_date,
        time_label,
        topic,
        ip,
    };

    let send_result = tokio::task::spawn_blocking(move || mailer::send_booking(&payload)).await;

    match send_result {
        Ok(Ok(())) => {
            tracing::info!("booking sent for {}", id);
            let loc = format!("/book/confirmed?ref={}", urlencoding::encode(&id));
            Redirect::temporary(loc.as_str()).into_response()
        }
        Ok(Err(e)) => {
            tracing::error!("booking smtp send failed: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Could not send the request right now. Please email mcochran@cochranblock.org directly.",
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("booking send task panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error. Please email mcochran@cochranblock.org directly.",
            )
                .into_response()
        }
    }
}

fn confirmed_html(ref_id: Option<&str>) -> String {
    let esc = |s: &str| -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    };
    let ref_line = ref_id
        .filter(|s| !s.is_empty())
        .map(|r| format!(r#"<p class="intake-detail">Reference ID: <code>{}</code></p>"#, esc(r)))
        .unwrap_or_default();
    let content = format!(
        r#"<section class="intake-section intake-done"><div class="intake-doc intake-complete"><div class="intake-check-icon">&check;</div><h1>Request Sent</h1><p class="intake-success">Your discovery call request was sent.</p>{}<p class="intake-detail">I will confirm within 24 hours by email.</p><a href="/" class="btn btn-secondary" style="margin-top:1rem">Done</a></div></section>"#,
        ref_line
    );
    format!(
        "{}{}{}{}",
        f62("book-confirmed", "Request Sent | CochranBlock"),
        C7,
        content,
        C8
    )
}

/// GET /book/confirmed
pub async fn confirmed(Query(q): Query<std::collections::HashMap<String, String>>) -> Html<String> {
    let ref_id = q.get("ref").map(|s| s.as_str());
    Html(confirmed_html(ref_id))
}
