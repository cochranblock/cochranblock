#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// All Rights Reserved. The Cochran Block, LLC.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.7, Composer 1.5

// /n-bench. Szeder-styled N-x-engineer calculator. Cloudflare Turnstile gate
// on calculator submit; Stripe Checkout for the personalized PDF report.
//
// Formula constants are tunable here. Page math runs client-side from the
// same constants embedded into the page so the calculator is fast and the
// server-side stays stateless.

use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::t0;

// ----- formula constants (tune here) -------------------------------------

const SOLO_VELOCITY_BASELINE: f64 = 3.0;       // commits/day baseline for a solo dev
const VELOCITY_CAP: f64 = 5.0;                 // sustainability ceiling on velocity multiplier
const SOLO_BREADTH_NORM: f64 = 1.5;            // active-repos baseline
const BREADTH_CAP: f64 = 2.5;                  // breadth multiplier ceiling
const DISCIPLINE_LIFT: f64 = 0.7;              // max boost from discipline (0..1) -> 1..1.7
const VOICE_LIFT: f64 = 0.5;                   // max boost from authentic-voice score (0..1) -> 1..1.5
const M_FLOOR: f64 = 0.10;                     // residual shipped-asset value at zero revenue
const M_CEIL: f64 = 2.0;                       // monetization realism ceiling

const REPORT_PRICE_CENTS: u32 = 1000;          // $10 personalized PDF report

const TURNSTILE_VERIFY_URL: &str =
    "https://challenges.cloudflare.com/turnstile/v0/siteverify";

// ----- handlers ----------------------------------------------------------

pub async fn get_page(State(_p0): State<Arc<t0>>) -> Html<String> {
    let site_key = std::env::var("TURNSTILE_SITE_KEY").unwrap_or_default();
    let stripe_pk = std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_default();
    Html(render_page(&site_key, &stripe_pk))
}

#[derive(Deserialize)]
pub struct CheckoutForm {
    pub email: String,
    #[serde(rename = "cf-turnstile-response")]
    pub turnstile_token: String,
    pub n_engineering: String,
    pub n_founder: String,
}

pub async fn create_checkout(
    State(_p0): State<Arc<t0>>,
    Form(form): Form<CheckoutForm>,
) -> impl IntoResponse {
    let secret = std::env::var("TURNSTILE_SECRET_KEY").unwrap_or_default();
    if secret.is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "turnstile not configured"})),
        )
            .into_response();
    }
    if !verify_turnstile(&secret, &form.turnstile_token).await {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "turnstile failed"})),
        )
            .into_response();
    }

    let stripe_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_default();
    if stripe_key.is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "payments not configured"})),
        )
            .into_response();
    }

    let client = reqwest::Client::new();
    let price_str = REPORT_PRICE_CENTS.to_string();
    let n_eng = form.n_engineering.clone();
    let n_fnd = form.n_founder.clone();
    let product_name = format!(
        "N-Bench Report (Eng {}x, Founder {}x)",
        clamp_display(&n_eng),
        clamp_display(&n_fnd)
    );
    let params = [
        ("payment_method_types[]", "card"),
        ("mode", "payment"),
        ("customer_email", form.email.as_str()),
        ("line_items[0][price_data][currency]", "usd"),
        ("line_items[0][price_data][unit_amount]", price_str.as_str()),
        (
            "line_items[0][price_data][product_data][name]",
            product_name.as_str(),
        ),
        ("line_items[0][quantity]", "1"),
        (
            "success_url",
            "https://cochranblock.org/n-bench/thanks?session_id={CHECKOUT_SESSION_ID}",
        ),
        ("cancel_url", "https://cochranblock.org/n-bench"),
        ("metadata[n_engineering]", n_eng.as_str()),
        ("metadata[n_founder]", n_fnd.as_str()),
    ];

    match client
        .post("https://api.stripe.com/v1/checkout/sessions")
        .header("Authorization", format!("Bearer {}", stripe_key))
        .form(&params)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(body) => {
                if let Some(url) = body.get("url").and_then(|u| u.as_str()) {
                    return axum::response::Redirect::to(url).into_response();
                }
                Json(serde_json::json!({"id": body["id"], "raw": body})).into_response()
            }
            Err(e) => Json(serde_json::json!({"error": format!("{}", e)})).into_response(),
        },
        Err(e) => Json(serde_json::json!({"error": format!("{}", e)})).into_response(),
    }
}

pub async fn thanks(State(_p0): State<Arc<t0>>) -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>N-Bench, paid</title>
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>body{background:#0a0a14;color:#e0e0e0;font-family:system-ui,sans-serif;max-width:640px;margin:80px auto;padding:24px;line-height:1.6}a{color:#ff6600}h1{font-size:1.6rem}</style>
</head><body>
<h1>Paid. Report inbound.</h1>
<p>Your personalized N-Bench breakdown will land in the email you provided within 24 hours. Includes the full Szeder-styled formula walkthrough, the vaporware decay curve charted against your runway, and the one-customer-flips-the-board sensitivity pass.</p>
<p><a href="/n-bench">Run another scenario</a> | <a href="/">Cochran Block</a></p>
</body></html>"#,
    )
}

// ----- helpers -----------------------------------------------------------

async fn verify_turnstile(secret: &str, token: &str) -> bool {
    let client = reqwest::Client::new();
    let params = [("secret", secret), ("response", token)];
    match client
        .post(TURNSTILE_VERIFY_URL)
        .form(&params)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(body) => body.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

fn clamp_display(s: &str) -> String {
    let v: f64 = s.parse().unwrap_or(0.0);
    let v = v.clamp(0.0, 999.0);
    format!("{:.1}", v)
}

// ----- page render -------------------------------------------------------

fn render_page(turnstile_site_key: &str, stripe_pk: &str) -> String {
    let _ = stripe_pk; // reserved for future inline Stripe.js paths
    let widget = if turnstile_site_key.is_empty() {
        // Dev-mode fallback when no key is configured. Calculator still works,
        // checkout endpoint will reject the empty token.
        String::from(r#"<div class="t-warn">Turnstile site key not configured. Calculator runs locally; report checkout disabled.</div>"#)
    } else {
        format!(
            r#"<div class="cf-turnstile" data-sitekey="{}" data-theme="dark"></div>"#,
            turnstile_site_key
        )
    };

    format!(
        r##"<!DOCTYPE html><html lang="en"><head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>N-Bench. What kind of N-x engineer are you?</title>
<meta name="description" content="Run the Szeder-styled N-x-engineer calculator on yourself. Two scoreboards: code output and founder output. Vaporware decay included.">
<script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer></script>
<style>
:root{{--bg:#0a0a14;--fg:#e0e0e0;--fg2:#aaa;--fg3:#888;--accent:#ff6600;--card:#15152a;--line:#2a2a44}}
*{{box-sizing:border-box}}
body{{background:var(--bg);color:var(--fg);font-family:system-ui,-apple-system,sans-serif;line-height:1.55;margin:0;padding:0}}
.wrap{{max-width:880px;margin:0 auto;padding:32px 24px 96px}}
h1{{font-size:1.8rem;margin:0 0 8px;color:#fff}}
h2{{font-size:1.15rem;margin:32px 0 12px;color:#fff;border-bottom:1px solid var(--line);padding-bottom:6px}}
p.lede{{color:var(--fg2);margin:0 0 24px}}
.card{{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:18px 20px;margin:16px 0}}
.row{{display:grid;grid-template-columns:1fr 120px;gap:12px;align-items:center;margin:8px 0}}
.row label{{color:var(--fg2);font-size:0.92rem}}
.row input,.row select{{background:#0d0d1c;color:var(--fg);border:1px solid var(--line);border-radius:4px;padding:6px 8px;font-family:inherit;font-size:0.95rem;text-align:right}}
.checks{{display:grid;grid-template-columns:1fr 1fr;gap:6px 16px;font-size:0.9rem;color:var(--fg2)}}
.checks label{{display:flex;gap:8px;align-items:center;cursor:pointer}}
.scoreboard{{display:grid;grid-template-columns:1fr 1fr;gap:18px;margin:24px 0}}
.score{{background:#0d0d1c;border:1px solid var(--accent);border-radius:8px;padding:18px;text-align:center}}
.score .v{{font-size:2.8rem;font-weight:700;color:var(--accent);line-height:1}}
.score .l{{color:var(--fg2);font-size:0.85rem;letter-spacing:0.06em;text-transform:uppercase;margin-top:6px}}
.score .x{{color:var(--fg3);font-size:0.78rem;margin-top:6px;font-style:italic}}
.gap{{background:#1a1208;border:1px solid #663300;border-radius:6px;padding:10px 14px;margin:12px 0;font-size:0.9rem;color:#ffaa66}}
.cta{{display:flex;gap:12px;flex-wrap:wrap;margin-top:24px}}
.btn{{background:var(--accent);color:#000;border:none;border-radius:4px;padding:10px 18px;font-weight:600;cursor:pointer;font-size:0.95rem;text-decoration:none}}
.btn.ghost{{background:transparent;color:var(--fg);border:1px solid var(--line)}}
.t-warn{{color:#aa6600;font-size:0.82rem;font-style:italic;margin:8px 0}}
.email{{display:grid;grid-template-columns:1fr 200px;gap:8px;margin:8px 0}}
.email input{{background:#0d0d1c;color:var(--fg);border:1px solid var(--line);border-radius:4px;padding:8px 10px;font-size:0.95rem}}
.foot{{color:var(--fg3);font-size:0.78rem;margin-top:32px;border-top:1px solid var(--line);padding-top:12px}}
.curve{{font-family:ui-monospace,Menlo,monospace;font-size:0.78rem;color:var(--fg2);background:#0d0d1c;padding:10px;border:1px solid var(--line);border-radius:4px;white-space:pre;overflow-x:auto}}
</style>
</head><body><div class="wrap">

<h1>N-Bench</h1>
<p class="lede">Two scoreboards, one engineer. Code output and founder output do not measure the same thing. Run yours below. Math is client-side, lifted from the Szeder lens (build vs buy, ship discipline, sustainable team velocity, vaporware decay).</p>

<div class="card">
<h2>Engineering inputs</h2>
<div class="row"><label>Commits per day, sustained</label><input id="cpd" type="number" min="0" step="0.1" value="3"></div>
<div class="row"><label>Active shipping repos</label><input id="repos" type="number" min="0" step="1" value="2"></div>
<div class="row"><label>Ship ratio (production / projects-touched)</label><input id="ship" type="number" min="0" max="1" step="0.05" value="0.7"></div>
<div class="row"><label>Team multiplier (1.0 solo, 1.2 +cofounder, 1.4 +operator)</label><input id="team" type="number" min="0.5" max="3" step="0.1" value="1.0"></div>
<div class="row"><label>NIH friction (0 = pure buy, 1 = pure NIH)</label><input id="nih" type="number" min="0" max="1" step="0.05" value="0.10"></div>

<h2 style="margin-top:18px">Discipline checklist</h2>
<div class="checks">
<label><input type="checkbox" class="d" checked> Per-repo BACKLOG.md</label>
<label><input type="checkbox" class="d"> P-prefix initiative log</label>
<label><input type="checkbox" class="d"> CI gate on every push</label>
<label><input type="checkbox" class="d"> Triple-sims / triple-lens review</label>
<label><input type="checkbox" class="d"> Header-writer / contributor attribution</label>
<label><input type="checkbox" class="d"> Federal compliance harness</label>
<label><input type="checkbox" class="d"> Supply-chain audit ritual</label>
<label><input type="checkbox" class="d"> Hot-reload zero-downtime deploy</label>
</div>

<h2 style="margin-top:18px">Authentic voice (commit messages)</h2>
<div class="checks">
<label><input type="checkbox" class="v" checked> No "fix bug" boilerplate</label>
<label><input type="checkbox" class="v"> Distinctive language / project codenames</label>
<label><input type="checkbox" class="v"> Cross-references to plans or docs</label>
<label><input type="checkbox" class="v"> Numbers / measurements in messages</label>
</div>
</div>

<div class="card">
<h2>Founder inputs</h2>
<div class="row"><label>Monthly revenue (USD)</label><input id="rev" type="number" min="0" step="50" value="0"></div>
<div class="row"><label>Days at zero revenue while shipping</label><input id="dz" type="number" min="0" step="1" value="47"></div>
<div class="row"><label>Runway days remaining</label><input id="rd" type="number" min="1" step="1" value="90"></div>
</div>

<div class="scoreboard">
<div class="score"><div class="v" id="ne">0.0x</div><div class="l">Eng-N</div><div class="x">code-output bench</div></div>
<div class="score"><div class="v" id="nf">0.0x</div><div class="l">Founder-N</div><div class="x">market-output bench</div></div>
</div>

<div id="gap" class="gap" style="display:none"></div>

<div class="card">
<h2>Vaporware decay (next 90 days at zero revenue)</h2>
<pre class="curve" id="curve"></pre>
<div style="color:var(--fg3);font-size:0.82rem;margin-top:6px">Founder-N is gated by F_vap = exp(-days_zero_rev / runway). One paying customer pulls M off the floor and the curve flattens.</div>
</div>

<div class="card">
<h2>Personalized PDF report ($10)</h2>
<p style="color:var(--fg2);font-size:0.92rem;margin-top:0">Full Szeder-styled walkthrough on your inputs, vaporware decay charted against your runway, sensitivity pass on the one-customer flip, and a single concrete next-action call. Email-delivered, signed, no marketing.</p>
<form id="checkout" method="post" action="/api/n-bench/checkout">
<div class="email">
<input id="email" name="email" type="email" placeholder="email for delivery" required>
<button class="btn" type="submit" id="btn-pay">Pay $10. Get report.</button>
</div>
<input type="hidden" name="n_engineering" id="ne_h" value="0">
<input type="hidden" name="n_founder" id="nf_h" value="0">
{widget}
</form>
</div>

<div class="foot">
Cochran Block. Unlicense reasoning, paid delivery. CAGE 1CQ66. UEI W7X3HAQL9CF9. Math constants in the page source if you want to fork the formula.
</div>

</div>

<script>
(function(){{
  var SOLO_V = {solo_v}, V_CAP = {v_cap}, SOLO_B = {solo_b}, B_CAP = {b_cap};
  var D_LIFT = {d_lift}, V_LIFT_VOICE = {voice_lift}, M_FLOOR = {m_floor}, M_CEIL = {m_ceil};

  function num(id){{ return parseFloat(document.getElementById(id).value) || 0; }}
  function clamp(x,a,b){{ return Math.max(a,Math.min(b,x)); }}
  function disciplineScore(){{
    var els = document.querySelectorAll('.d');
    var t = 0, n = els.length;
    els.forEach(function(e){{ if(e.checked) t++; }});
    return n ? t/n : 0;
  }}
  function voiceScore(){{
    var els = document.querySelectorAll('.v');
    var t = 0, n = els.length;
    els.forEach(function(e){{ if(e.checked) t++; }});
    return n ? t/n : 0;
  }}

  function compute(){{
    var v = clamp(num('cpd')/SOLO_V, 0, V_CAP);
    var b = clamp(num('repos')/SOLO_B, 0, B_CAP);
    var d = 1 + D_LIFT * disciplineScore();
    var s = clamp(num('ship'), 0, 1);
    var t = clamp(num('team'), 0.5, 3);
    var a = 1 + V_LIFT_VOICE * voiceScore();
    var f = clamp(num('nih'), 0, 1);
    var nEng = v * b * d * s * t * a * (1 - f);

    var rev = Math.max(0, num('rev'));
    var dz = Math.max(0, num('dz'));
    var rd = Math.max(1, num('rd'));
    var m = Math.log(1 + rev/100) / Math.log(11);
    m = clamp(m, M_FLOOR, M_CEIL);
    var fvap = Math.exp(-dz / rd);
    var nFnd = nEng * m * fvap;

    document.getElementById('ne').textContent = nEng.toFixed(1) + 'x';
    document.getElementById('nf').textContent = nFnd.toFixed(1) + 'x';
    document.getElementById('ne_h').value = nEng.toFixed(2);
    document.getElementById('nf_h').value = nFnd.toFixed(2);

    var gap = document.getElementById('gap');
    var ratio = nEng > 0 ? nFnd / nEng : 0;
    if (ratio < 0.2 && nEng > 5) {{
      gap.style.display = 'block';
      gap.textContent = 'Gap diagnosis: code output ' + nEng.toFixed(1) + 'x, market output ' + nFnd.toFixed(1) + 'x. The leverage is not more code. It is the first paying customer.';
    }} else if (rev === 0 && nEng > 1) {{
      gap.style.display = 'block';
      gap.textContent = 'Zero revenue active. F_vap = ' + fvap.toFixed(2) + '. The clock is your real budget, not your commit budget.';
    }} else {{
      gap.style.display = 'none';
    }}

    var curve = '';
    var step = Math.max(1, Math.floor(rd / 12));
    for (var i = 0; i <= rd; i += step) {{
      var fv = Math.exp(-i/rd);
      var bar = '';
      var w = Math.round(fv * 40);
      for (var j = 0; j < w; j++) bar += '#';
      curve += ('day ' + i).padEnd(8) + ' ' + bar + ' ' + fv.toFixed(2) + '\n';
    }}
    document.getElementById('curve').textContent = curve;
  }}

  document.querySelectorAll('input,select').forEach(function(el){{
    el.addEventListener('input', compute);
    el.addEventListener('change', compute);
  }});
  compute();
}})();
</script>
</body></html>"##,
        widget = widget,
        solo_v = SOLO_VELOCITY_BASELINE,
        v_cap = VELOCITY_CAP,
        solo_b = SOLO_BREADTH_NORM,
        b_cap = BREADTH_CAP,
        d_lift = DISCIPLINE_LIFT,
        voice_lift = VOICE_LIFT,
        m_floor = M_FLOOR,
        m_ceil = M_CEIL,
    )
}
