#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use axum::extract::State;
use axum::response::Html;
use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use chrono_tz::America::New_York;
use serde::Serialize;
use std::sync::Arc;

use crate::t0;

#[derive(Serialize)]
struct t19 {
    label: String,
    mailto: String,
}

#[derive(Serialize)]
struct t20 {
    date: String,
    day_name: String,
    date_label: String,
    times: Vec<t19>,
}

const BASE_URL: &str = "https://cochranblock.org";

/// f69 = robots_txt. Why: Allow crawlers; point to sitemap.
pub async fn f69(State(_p0): State<Arc<t0>>) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "User-agent: *\nAllow: /\nSitemap: https://cochranblock.org/sitemap.xml\n",
    )
}

/// f70 = sitemap_xml. Why: Main pages for search engines.
pub async fn f70(State(_p0): State<Arc<t0>>) -> impl axum::response::IntoResponse {
    let urls = [
        ("/", "1.0", "weekly"),
        ("/products", "0.9", "weekly"),
        ("/services", "0.9", "weekly"),
        ("/deploy", "0.9", "weekly"),
        ("/downloads", "0.8", "weekly"),
        ("/mathskillz", "0.8", "weekly"),
        ("/provenance", "0.8", "weekly"),
        ("/sbir", "0.8", "weekly"),
        ("/codeskillz", "0.8", "weekly"),
        ("/about", "0.8", "monthly"),
        ("/contact", "0.8", "monthly"),
        ("/book", "0.8", "weekly"),
    ];
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    for (path, priority, changefreq) in urls {
        xml.push_str(&format!(
            r#"<url><loc>{}{}</loc><changefreq>{}</changefreq><priority>{}</priority></url>"#,
            BASE_URL, path, changefreq, priority
        ));
    }
    xml.push_str("</urlset>");
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
}

const JSON_LD_ORG: &str = r#"<script type="application/ld+json">{"@context":"https://schema.org","@type":"Organization","name":"CochranBlock","url":"https://cochranblock.org","description":"Fractional CTO and Zero-Cloud Architect. This entire company runs as a single Rust binary on a laptop for $10/month. 12 Unlicense repos prove every claim."}</script>"#;

/// f62 = html_head. Why: Consistent head + body open; data-page for CSS/JS targeting; JSON-LD Organization.
pub fn f62(p0: &str, p1: &str) -> String {
    let v_path = if p0 == "home" {
        "/".to_string()
    } else {
        format!("/{}", p0)
    };
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="CochranBlock — Fractional CTO &amp; Zero-Cloud Architect. Entire company runs as a single Rust binary on a laptop. $10/month infrastructure. 12 Unlicense repos prove every claim."><title>{}</title><meta property="og:title" content="{}"><meta property="og:description" content="Fractional CTO &amp; Zero-Cloud Architect. Single Rust binary, $10/month laptop, 12 Unlicense repos. It's not the Mech — it's the pilot."><meta name="twitter:card" content="summary_large_image"><meta name="twitter:image" content="https://cochranblock.org/assets/og-image.png"><meta property="og:image" content="https://cochranblock.org/assets/og-image.png"><meta property="og:image:width" content="1200"><meta property="og:image:height" content="630"><meta property="og:type" content="website"><meta property="og:url" content="{}{}"><link rel="icon" type="image/svg+xml" href="/assets/favicon.svg?v=9" sizes="32x32"><link rel="apple-touch-icon" href="/assets/apple-touch-icon.png"><link rel="manifest" href="/assets/manifest.json"><link rel="stylesheet" href="/assets/css/main.css?v=2">{}</head><body data-page="{}">"#,
        p1,
        p1,
        BASE_URL,
        v_path,
        JSON_LD_ORG,
        p0
    )
}
pub const C7: &str = r##"<a href="#main" class="skip-link">Skip to main content</a><nav class="nav"><a href="/" class="nav-brand"><img src="/assets/favicon.svg?v=9" alt="" class="nav-favicon" width="32" height="32">CochranBlock</a><button class="nav-toggle" aria-label="Toggle menu" aria-expanded="false" aria-controls="nav-links"><span class="nav-toggle-bar"></span><span class="nav-toggle-bar"></span><span class="nav-toggle-bar"></span></button><div id="nav-links" class="nav-links" role="navigation"><a href="/services">Services</a><a href="/deploy">Deploy</a><a href="/book">Book</a><a href="/mathskillz">Math</a><a href="/codeskillz">Code</a><a href="/products">Products</a><a href="/about">About</a><a href="/sbir" class="nav-more">SBIR</a><a href="/downloads" class="nav-more">Downloads</a><a href="/contact" class="nav-more">Contact</a><a href="/community-grant" class="nav-more">Grant</a></div></nav><main id="main" class="content">"##;
pub const C8: &str = r#"</main><footer class="footer"><nav class="footer-nav"><a href="/">Home</a><a href="/products">Products</a><a href="/services">Services</a><a href="/deploy">Deploy</a><a href="/book">Book</a><a href="/mathskillz">Math</a><a href="/codeskillz">Code</a><a href="/products">Products</a><a href="/about">About</a><a href="/contact">Contact</a><a href="/sbir">SBIR</a></nav><p class="footer-brand"><a href="https://cochranblock.org"><img src="/assets/cochranblock-logo.svg?v=9" alt="CochranBlock" class="footer-logo" width="180" height="32"></a></p><p class="footer-certs">Service-Disabled Veteran-Owned Small Business (SDVOSB) · Pending</p><p>&copy; 2026 CochranBlock</p><p class="footer-cta"><a href="mailto:mcochran@cochranblock.org?subject=CochranBlock%20Inquiry" class="btn btn-secondary">Get in Touch</a></p><p class="footer-links"><a href="https://www.linkedin.com/in/cochranblock" target="_blank" rel="noopener noreferrer">LinkedIn</a></p></footer><script>(function(){var t=document.querySelector('.nav-toggle');var n=document.getElementById('nav-links');if(t&&n){t.onclick=function(){var o=n.classList.toggle('nav-open');t.setAttribute('aria-expanded',o);}}}());</script></body></html>"#;

/// f2 = serve_index. Why: Hero page; first impression for cochranblock.org.
pub async fn f2(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="hero"><p class="hero-status">Fractional CTO · Zero-Cloud Architect · Veteran-Owned · Consulting: open</p><div class="hero-logo"><a href="/products"><img src="/assets/cochranblock-hero-logo.svg?v=9" alt="" class="hero-logo-img" width="128" height="128"></a></div><h1>Your server bill is too high.</h1><p class="tagline">This page — the site you're reading right now — is a single Rust binary running on a laptop — 18MB on x86, 9.9MB on ARM. Total cost: <strong>$10/month</strong>. No AWS. No Kubernetes. No DevOps team.</p><p class="hero-stats">You're looking at the proof.</p><p class="hero-note">I'm a Fractional CTO who builds zero-cloud architectures. Edge compute beats cloud. One binary replaces five services. I've done it for 13 years across defense and enterprise — and I open-sourced <a href="https://github.com/cochranblock">12 Rust repos</a> so you can verify every claim before we talk.</p><p class="hero-skills">Sovereign Intelligence for the Public Domain · Zero-Cloud Architecture · Rust SaaS · 13 Years Defense &amp; Enterprise · AI-Piloted Development · 12 Unlicense Repos</p><p class="hero-cta"><a href="/deploy" class="btn">Find Out How Much You Can Save</a><a href="/products" class="btn btn-secondary">See the Architecture</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="https://github.com/cochranblock" class="btn btn-secondary">GitHub (Proof)</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("home", "CochranBlock | Fractional CTO · Zero-Cloud Architect"), C7, v0, C8))
}

/// f11 = serve_services. Why: Pricing + Fractional CTO services. Funnels to /deploy.
pub async fn f11(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="services">
<h1>Services &amp; Pricing</h1>
<p class="services-intro">Fractional CTO and zero-cloud architecture. I replace your $5K/month AWS bill with a $10/month laptop. Transparent pricing because technical founders hate hidden costs.</p>

<h2 class="services-section-head">What I Replace</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>Your current stack → my stack</summary>
<p>$5,000/month AWS bill → <strong>$10/month</strong> laptop + Cloudflare tunnel<br>
Kubernetes + 5 managed services → <strong>single Rust binary</strong><br>
DevOps team → <strong>cargo build</strong> + one deploy command<br>
Microservices sprawl → <strong>all routes embedded</strong>, one process<br>
Cloud vendor lock-in → <strong>Unlicensed code</strong>, runs anywhere<br>
<span class="service-outcome">cochranblock.org is the live proof. You're looking at it right now.</span></p>
</details>
</div>

<h2 class="services-section-head">Pricing</h2>
<div class="pricing-cards">
<article class="pricing-card"><span class="product-badge">Ship It</span><h2>One-Time Deployment</h2><div class="pricing-amount">$3,500</div><div class="pricing-unit">flat fee</div><p>Compile a production Rust binary and deploy it to your local or edge hardware. Includes 14-day warranty. You own the binary, the server, and the code.</p></article>
<article class="pricing-card"><span class="product-badge">Build It</span><h2>Long-Term Project</h2><div class="pricing-amount">$3,500</div><div class="pricing-unit">per milestone</div><p>Pre-agreed architectural phases or microservices. Scope defined before work starts. Each milestone is a deployable deliverable — no half-built features.</p></article>
<article class="pricing-card"><span class="product-badge">Own It</span><h2>Monthly Retainer</h2><div class="pricing-amount">$3,500</div><div class="pricing-unit">per month · 15 hours</div><p>Fractional CTO strategy, architecture design, code review, and infrastructure consulting. Your on-call senior engineer without the full-time salary.</p></article>
</div>

<p class="pricing-divider">need something outside the scope?</p>

<div class="service-cards">
<details class="service-card">
<summary>A La Carte — $225/hr</summary>
<p>Hands-on-keyboard coding, additional hours beyond retainer cap, feature additions, or anything outside the defined scope. Minimum 30-minute increments. Written estimate + your approval before work begins.<span class="service-outcome">No surprises. You approve every hour before I bill it.</span></p>
</details>
<details class="service-card">
<summary>Emergency — $337.50/hr (1.5x)</summary>
<p>After-hours or drop-everything priority. Weekends, nights, production-down situations. Same approval workflow — you say go, I go.<span class="service-outcome">For when the server is on fire and you need someone who's done this before.</span></p>
</details>
</div>

<h2 class="services-section-head">What You Get</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>Zero lock-in</summary>
<p>All custom code delivered under the Unlicense — public domain. You own everything. No recurring license fees, no dependency on me, no vendor trap. I earn repeat business by being good, not by holding your code hostage.<span class="service-outcome">Every other consultant locks you in. I give you the keys.</span></p>
</details>
<details class="service-card" open>
<summary>13 years defense &amp; enterprise</summary>
<p>USCYBERCOM J38 dev lead for a Congressional NDAA-directed offensive cyber operations study. Two Six Technologies. Enterprise security. Active clearance. I've shipped production systems for organizations that can't afford downtime.<span class="service-outcome">Not my first build. Not my first deployment. Not my first fire.</span></p>
</details>
<details class="service-card">
<summary>12 open source repos as proof</summary>
<p>Every repo at <a href="https://github.com/cochranblock">github.com/cochranblock</a> ships with Proof of Artifacts and a Timeline of Invention. Wire diagrams, screenshots, build output, commit-level history. You can verify my work before we ever talk.<span class="service-outcome">Talk is cheap. Code ships.</span></p>
</details>
</div>

<p class="services-cta"><a href="/deploy" class="btn">Start a Project</a><a href="/book" class="btn btn-secondary">Book a Call First</a></p>
</section>"#;
    Html(format!("{}{}{}{}", f62("services", "Services & Pricing | CochranBlock"), C7, v0, C8))
}

/// f72 = serve_mathskillz. Why: Public cost-savings math — cloud vs zero-cloud, justifies pricing.
pub async fn f72(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="services">
<h1>Your $3,000/month cloud bill should be $10.</h1>
<p class="services-intro">Every number on this page is real. The site you're reading right now is the proof.</p>

<div class="cost-summary">
<table class="cost-table">
<tr><td>Typical cloud cost</td><td class="cost-amount cost-old">$36,000/year</td></tr>
<tr><td>After CochranBlock deployment</td><td class="cost-amount cost-new">$120/year</td></tr>
<tr><td>One-time deployment fee</td><td class="cost-amount">$3,500</td></tr>
<tr class="cost-row-highlight"><td><strong>Year 1 net savings</strong></td><td class="cost-amount cost-new"><strong>$32,380</strong></td></tr>
<tr class="cost-row-highlight"><td><strong>Year 2+ savings</strong></td><td class="cost-amount cost-new"><strong>$35,880/year</strong></td></tr>
</table>
</div>

<p class="services-cta"><a href="/deploy" class="btn">Start a Project</a><a href="/book" class="btn btn-secondary">Book a Call</a></p>

<h2 class="services-section-head">The Scenario</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>A 3-person startup on AWS running a standard web app</summary>
<p>
EC2 instances (2x t3.medium): <strong>$120/mo</strong><br>
RDS PostgreSQL (db.t3.medium): <strong>$140/mo</strong><br>
Application Load Balancer: <strong>$50/mo</strong><br>
CloudFront CDN: <strong>$30/mo</strong><br>
S3 storage + transfer: <strong>$25/mo</strong><br>
CloudWatch monitoring: <strong>$30/mo</strong><br>
CI/CD (CodePipeline + CodeBuild): <strong>$45/mo</strong><br>
Part-time DevOps contractor: <strong>$2,500/mo</strong><br>
<span class="service-outcome">Total: $2,940/month → $35,280/year. This is conservative. Most spend more.</span>
</p>
</details>
<details class="service-card" open>
<summary>The same app on CochranBlock infrastructure</summary>
<p>
Compiled Rust binary (web server + database + API + assets): <strong>$0</strong><br>
Bare metal hardware (one-time, you already own a computer): <strong>$0/mo</strong><br>
Electricity + internet: <strong>~$10/mo</strong><br>
Cloudflare tunnel (free tier): <strong>$0</strong><br>
CI/CD: <strong>$0</strong> (test binary is the pipeline)<br>
Monitoring: <strong>$0</strong> (built into the binary)<br>
DevOps: <strong>$0</strong> (there's nothing to operate — one process, one file)<br>
<span class="service-outcome">Total: $10/month → $120/year. No vendor invoices. No surprise bills. No scaling fees.</span>
</p>
</details>
</div>

<h2 class="services-section-head">ROI on Every Dollar</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>$3,500 deployment — pays for itself in 37 days</summary>
<p>
You're spending $2,940/month on cloud. I deploy a binary that replaces it for $3,500.<br>
Day 1: you stop paying AWS.<br>
Day 37: deployment fee is recovered from savings.<br>
Day 365: you've saved <strong>$32,380</strong>.<br>
<span class="service-outcome">Every year after that: $35,880 stays in your pocket instead of going to Amazon.</span>
</p>
</details>
<details class="service-card" open>
<summary>$225/hr consulting — 14-day payback</summary>
<p>
One hour of work ($225) that eliminates a $500/mo cloud service:<br>
Payback period: <strong>14 days</strong><br>
Year 1 return: <strong>$5,775</strong> (2,567% ROI)<br><br>
An 8-hour engagement ($1,800) replacing a $2,000/mo stack:<br>
Payback period: <strong>27 days</strong><br>
Year 1 return: <strong>$22,200</strong> (1,233% ROI)<br>
<span class="service-outcome">Every dollar you spend here saves $10–$50 in recurring costs.</span>
</p>
</details>
</div>

<h2 class="services-section-head">This Isn't New</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>37signals saved $10M leaving AWS</summary>
<p>
In 2022, 37signals (makers of Basecamp and HEY) spent $3.2 million per year on AWS. They moved to owned hardware and saved over $10 million in five years. DHH wrote: <em>"Renting computers is mostly a bad deal for medium-sized companies like ours."</em><br><br>
You don't need to be their size to benefit. The math scales down. A startup spending $3K/month on cloud saves $32K+ in year one with the same approach.<br>
<span class="service-outcome">I build the same architecture they moved to — but for small businesses, at a fraction of the cost.</span>
</p>
</details>
</div>

<h2 class="services-section-head">Why This Model Lasts</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>Low overhead = I don't disappear mid-contract</summary>
<p>
My infrastructure costs are near zero. No office lease. No cloud bills eating into revenue. No VC runway clock ticking down.<br><br>
That means I don't need to upsell you, I don't need to raise prices, and I don't need your next contract to keep the lights on. Pricing stays flat because the model is durable.<br>
<span class="service-outcome">You're hiring a business that can't be squeezed by a vendor — because there are no vendors.</span>
</p>
</details>
</div>

<h2 class="services-section-head">How It Works</h2>
<div class="service-cards">
<details class="service-card">
<summary>Single binary architecture</summary>
<p>
Your entire application — web server, database, API, static assets, TLS — compiles into one executable. 18MB on x86, 9.9MB on ARM. Deploy: copy it to a server and run it.<br>
<span class="service-outcome">cochranblock.org is that binary. Intake forms, SQLite, booking calendar, community grants — one process, one file.</span>
</p>
</details>
<details class="service-card">
<summary>Technical deep dive</summary>
<p>
Rust compiles to native machine code with no runtime. No garbage collector, no VM, no interpreter. A Rust web server uses 2–10MB of RAM. The equivalent Node.js/Python/Java app uses 100–500MB.<br>
<span class="service-outcome">Smaller binary. Less RAM. Less power. Less cost. Faster response. <a href="/downloads">Download the binary</a> and verify.</span>
</p>
</details>
</div>

<h2 class="services-section-head">Edge Intelligence — Ghost Fabric</h2>
<p class="services-intro" style="margin-bottom:1rem;">Cloud savings is half the story. The other half is what happens when there is no cloud.</p>
<div class="service-cards">
<details class="service-card">
<summary>The problem: AI doesn't work at the edge</summary>
<p>
Standard AI deployment: Python + PyTorch + transformers = <strong>2–4GB</strong> of dependencies before the first tensor is allocated. Cold-start: <strong>3–15 seconds</strong>. Over 400 transitive packages — each one an unaudited attack vector on a node with no firewall.<br><br>
Now try pushing that over a LoRa radio link at <strong>5.5 kbps</strong>. You can't. The cloud model breaks when bandwidth is measured in bits, not gigabits.<br>
<span class="service-outcome">Python is the right tool for research. It is the wrong tool for deployment on hardware that must survive without human intervention.</span>
</p>
</details>
<details class="service-card">
<summary>The solution: 19MB Rust binary with embedded intelligence</summary>
<p>
Ghost Fabric compiles the inference engine, quantized model weights, LoRa mesh protocol, sensor I/O, and decision agent into a single <strong>19MB</strong> statically linked binary.<br><br>
The binary's working set fits entirely in L3 cache (modern CPUs carry 16–64MB), keeping hot execution paths off the memory bus. Result: <strong>millisecond cold-boots</strong> and a deterministic memory footprint.<br><br>
No interpreter. No garbage collector. No dynamic linking. No package manager on the node. One file. One process. One owner.<br>
<span class="service-outcome">The same architecture that saves you $32K/year on cloud also runs AI on a $5 microcontroller in a field with no internet.</span>
</p>
</details>
<details class="service-card">
<summary>The math: Python vs Rust at the edge</summary>
<p>
<strong>Python inference node:</strong><br>
Dependencies: 2–4GB · Cold-start: 3–15s · RAM: 500MB–2GB · Power: requires full Linux + GPU<br>
Security surface: 400+ packages · Deployment: Docker + pip + CUDA · Update: redeploy entire container<br><br>
<strong>Ghost Fabric node:</strong><br>
Dependencies: 0 (statically linked) · Cold-start: &lt;50ms · RAM: 8–32MB · Power: runs on ARM SBC<br>
Security surface: 1 binary · Deployment: scp + run · Update: replace 1 file<br>
<span class="service-outcome">100x smaller. 100x faster to start. 100x less attack surface. Same intelligence.</span>
</p>
</details>
<details class="service-card">
<summary>Applications</summary>
<p>
<strong>Agriculture:</strong> Soil and irrigation decisions made at the sensor, transmitted as actions over LoRa. No cellular. No Wi-Fi. No cloud.<br>
<strong>Disaster response:</strong> Drop mesh nodes into an area with no infrastructure. Self-organizing situational awareness without backhaul.<br>
<strong>Perimeter security:</strong> Persistent, low-power surveillance with on-node classification. Only alerts traverse the network.<br>
<strong>Industrial IoT:</strong> Factory floor sensors making real-time quality decisions locally, reporting anomalies over the mesh.<br>
<strong>Sovereign infrastructure:</strong> Government and defense deployments where data must never leave the physical perimeter.<br>
<span class="service-outcome"><a href="https://github.com/cochranblock/ghost-fabric">Read the whitepaper →</a></span>
</p>
</details>
</div>

<h2 class="services-section-head">Trust &amp; Compliance</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>Credentials</summary>
<p>
Service-Disabled Veteran-Owned Small Business (SDVOSB) — pending certification<br>
SAM.gov — registered (pending activation)<br>
Maryland eMMA — Certified Small Business application in progress<br>
13 years defense and enterprise — USCYBERCOM J38 dev lead<br>
12 open source repos — all code verifiable at <a href="https://github.com/cochranblock">github.com/cochranblock</a><br>
<span class="service-outcome">All source code delivered under the Unlicense. You own everything. Zero vendor lock-in.</span>
</p>
</details>
</div>

<p class="services-cta"><a href="/deploy" class="btn">Start a Project</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="/services" class="btn btn-secondary">Full Pricing</a></p>
</section>"#;
    Html(format!("{}{}{}{}", f62("mathskillz", "Cost Analysis — Zero-Cloud Savings | CochranBlock"), C7, v0, C8))
}

/// f76 = serve_codeskillz. Why: Gym badge wall — every repo, what it proves, live velocity.
pub async fn f76(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="services">
<h1>Code Skillz</h1>
<p class="services-intro">14 repos. All Rust. All Unlicense. All live. Every badge is a shipped product with a timestamp proving I'm still building it.</p>

<div id="badges-grid" class="badges-grid"><p class="velocity-loading">Loading badges...</p></div>

<script>
(function(){
  var meta={
    'cochranblock':{cat:'Web',desc:'This site. 18MB binary. $10/month. The live demo.'},
    'kova':{cat:'AI',desc:'Augment engine. Agent loop, 7 tools, 4-node cluster.'},
    'ghost-fabric':{cat:'Edge',desc:'Sovereign intelligence over LoRa mesh. 19MB binary.'},
    'pixel-forge':{cat:'AI',desc:'On-device diffusion models. 3 tiers. Pure Rust.'},
    'pocket-server':{cat:'Hardware',desc:'Phone-as-web-server. $2.60/year electricity.'},
    'call-shield':{cat:'Privacy',desc:'On-device call screening. Zero audio leaves the device.'},
    'provenance-docs':{cat:'Docs',desc:'AI development documentation framework for federal acquisition.'},
    'approuter':{cat:'Infra',desc:'Reverse proxy + Cloudflare tunnel. One entry point.'},
    'oakilydokily':{cat:'Client',desc:'First paying partnership. WASM mural, ESIGN waivers.'},
    'illbethejudgeofthat':{cat:'Legal',desc:'Google Takeout to court-ready exhibit book.'},
    'whyyoulying':{cat:'Defense',desc:'DoD IG fraud detection. Labor cat + ghost billing.'},
    'rogue-repo':{cat:'Platform',desc:'Sovereign app store + ISO 8583 payment engine.'},
    'exopack':{cat:'Test',desc:'TRIPLE SIMS. Screenshots. Headless Chrome. Zero-framework CI.'},
    'wowasticker':{cat:'Mobile',desc:'Voice dictation + on-device Whisper + behavioral scoring.'}
  };
  var cats={'Web':'#00d4aa','AI':'#d4a017','Edge':'#dc143c','Hardware':'#6b2fa0','Privacy':'#00a8cc','Docs':'#888','Infra':'#4a9eff','Client':'#00d4aa','Legal':'#ff6b6b','Defense':'#dc143c','Platform':'#d4a017','Test':'#888','Mobile':'#6b2fa0'};
  fetch('/api/velocity').then(function(r){return r.json()}).then(function(d){
    var g=document.getElementById('badges-grid');
    if(!d.repos||!d.repos.length){g.innerHTML='<p>Unavailable</p>';return;}
    var h='';
    d.repos.sort(function(a,b){return new Date(b.pushed_at)-new Date(a.pushed_at)});
    d.repos.forEach(function(r){
      var m=meta[r.repo]||{cat:'Rust',desc:''};
      var ago=timeAgo(new Date(r.pushed_at));
      var color=cats[m.cat]||'#888';
      h+='<a href="https://github.com/cochranblock/'+r.repo+'" class="badge-card" style="border-color:'+color+'">';
      h+='<span class="badge-cat" style="background:'+color+'">'+m.cat+'</span>';
      h+='<span class="badge-name">'+r.repo+'</span>';
      h+='<span class="badge-desc">'+m.desc+'</span>';
      h+='<span class="badge-ago">'+ago+'</span>';
      h+='</a>';
    });
    g.innerHTML=h;
  }).catch(function(){document.getElementById('badges-grid').innerHTML='<p>Unavailable</p>'});
  function timeAgo(d){
    var s=Math.floor((Date.now()-d.getTime())/1000);
    if(s<60)return s+'s ago';
    if(s<3600)return Math.floor(s/60)+'m ago';
    if(s<86400)return Math.floor(s/3600)+'h ago';
    return Math.floor(s/86400)+'d ago';
  }
})();
</script>

<h2 class="services-section-head">The Stack</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>Language: Rust — 100% of production code</summary>
<p>
No Python in production. No JavaScript frameworks. No Go. No Java. One language, compiled to native machine code, across every product.<br>
<span class="service-outcome">13 years of defense and enterprise. One language. Zero runtime dependencies.</span>
</p>
</details>
<details class="service-card" open>
<summary>Infrastructure: 4 bare metal Debian nodes + Mac Mini</summary>
<p>
lf (20 cores, 750GB) · gd (20 cores, 760GB) · bt (12 cores, 95GB) · st (14 cores, 767GB)<br>
Mac Mini ARM for development. All connected via SSH. All running zsh.<br>
<span class="service-outcome">$10/month total. Zero cloud. Zero containers. Zero Kubernetes.</span>
</p>
</details>
<details class="service-card">
<summary>AI: Candle + Kalosm — on-device inference</summary>
<p>
No OpenAI API calls in production. Models compiled into the binary via Candle. Whisper for speech-to-text. Custom diffusion models for image generation. LoRA fine-tuning. All on-device.<br>
<span class="service-outcome">AI without the cloud bill. AI without the data leak.</span>
</p>
</details>
<details class="service-card">
<summary>Certifications &amp; Registrations</summary>
<p>
SDVOSB (pending) · SAM.gov (registered) · Maryland eMMA (vendor) · CSB (pending)<br>
Army 17C Cyber Operations · JCAC 2013 · USCYBERCOM J38<br>
<span class="service-outcome">Every badge earned. Every registration filed. Every repo proves the claim.</span>
</p>
</details>
</div>

<p class="services-cta"><a href="/deploy" class="btn">Start a Project</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="https://github.com/cochranblock" class="btn btn-secondary">GitHub</a></p>
</section>"#;
    Html(format!("{}{}{}{}", f62("codeskillz", "Code Skillz — 14 Repos, All Rust, All Live | CochranBlock"), C7, v0, C8))
}

/// f74 = serve_provenance. Why: AI-piloted development documentation framework — SBIR pitch page.
pub async fn f74(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="services">
<h1>Provenance Docs</h1>
<p class="services-intro">A commit-integrated documentation framework for AI-piloted software development. The federal government has no standard for documenting who did what when AI assists the build. We built one.</p>

<div class="cost-summary">
<table class="cost-table">
<tr><td>Repositories using this framework</td><td class="cost-amount cost-new">13</td></tr>
<tr><td>Commits documented with human/AI attribution</td><td class="cost-amount cost-new">500+</td></tr>
<tr><td>External tooling required</td><td class="cost-amount cost-new">git (already everywhere)</td></tr>
<tr><td>Framework overhead per repo</td><td class="cost-amount cost-new">2 markdown files</td></tr>
</table>
</div>

<h2 class="services-section-head">The Problem</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>AI broke the attribution chain</summary>
<p>
The USPTO requires human inventors on patents. DFARS 252.227-7014 requires identification of privately-developed vs government-funded software. NIST SP 800-218 requires provenance tracking for software components. EO 14028 mandates supply chain transparency.<br><br>
None of these frameworks address AI-generated code. When a developer accepts AI output into a codebase, the IP boundary, the security audit trail, and the inventorship record all break.<br>
<span class="service-outcome">Every federal contractor using AI is creating undocumented supply chain inputs right now.</span>
</p>
</details>
</div>

<h2 class="services-section-head">The Solution: Two Documents</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>Timeline of Invention (TOI)</summary>
<p>
A dated, commit-linked record with a mandatory <strong>"AI Role"</strong> field on every entry. Documents what the human directed versus what the AI generated — at commit time, not months later.<br><br>
<strong>Date</strong> — when the work shipped<br>
<strong>What</strong> — concrete deliverable<br>
<strong>Why</strong> — business or technical driver<br>
<strong>Commit</strong> — git hash for traceability<br>
<strong>AI Role</strong> — what AI generated vs what human directed and verified<br>
<strong>Proof</strong> — link to artifact or test output<br>
<span class="service-outcome">The "AI Role" field is the critical innovation. It forces the developer to articulate the human/AI boundary on every entry.</span>
</p>
</details>
<details class="service-card" open>
<summary>Proof of Artifacts (POA)</summary>
<p>
Architecture diagrams, build metrics, screenshots, and exact verification commands proving the software is real, runs, and does what the TOI claims.<br><br>
Any reviewer can clone the repo, run the commands, and confirm. This is not documentation — it is a <strong>reproducibility contract</strong>.<br>
<span class="service-outcome">No Word documents. No slide decks. Verifiable evidence in the repository alongside the code.</span>
</p>
</details>
</div>

<h2 class="services-section-head">Live Proof</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>13 repositories, all public, all using this framework</summary>
<p>
<a href="https://github.com/cochranblock/cochranblock">cochranblock</a> — production site, 18MB binary<br>
<a href="https://github.com/cochranblock/provenance-docs">provenance-docs</a> — this framework's whitepaper and spec<br>
<a href="https://github.com/cochranblock/ghost-fabric">ghost-fabric</a> — edge intelligence over LoRa mesh<br>
<a href="https://github.com/cochranblock/kova">kova</a> — AI augment engine<br>
<a href="https://github.com/cochranblock/pixel-forge">pixel-forge</a> — on-device diffusion models<br>
<a href="https://github.com/cochranblock/pocket-server">pocket-server</a> — phone-as-web-server<br>
<a href="https://github.com/cochranblock/approuter">approuter</a> — reverse proxy<br>
<a href="https://github.com/cochranblock/oakilydokily">oakilydokily</a> — client site<br>
<a href="https://github.com/cochranblock/illbethejudgeofthat">illbethejudgeofthat</a> — pro se case builder<br>
<a href="https://github.com/cochranblock/whyyoulying">whyyoulying</a> — DoD fraud detection<br>
<a href="https://github.com/cochranblock/exopack">exopack</a> — test framework<br>
<a href="https://github.com/cochranblock/rogue-repo">rogue-repo</a> — app store + payment engine<br>
<a href="https://github.com/cochranblock/wowasticker">wowasticker</a> — behavioral scoring app<br>
<span class="service-outcome">Every repo contains TIMELINE_OF_INVENTION.md and PROOF_OF_ARTIFACTS.md. Click any link and verify.</span>
</p>
</details>
</div>

<h2 class="services-section-head">Federal Acquisition Mapping</h2>
<div class="service-cards">
<details class="service-card">
<summary>CDRL integration</summary>
<p>
<strong>DI-IPSC-81435</strong> (Software Design Description) → POA Architecture section<br>
<strong>DI-IPSC-81438</strong> (Software Product Specification) → POA Build Output section<br>
<strong>DI-MGMT-81466</strong> (Engineering Change Proposal) → TOI entries<br>
<strong>DI-IPSC-81441</strong> (Software Test Report) → POA How to Verify section<br>
<span class="service-outcome">TOI and POA replace or augment existing CDRLs — no new paperwork categories needed.</span>
</p>
</details>
<details class="service-card">
<summary>Compliance coverage</summary>
<p>
<strong>DFARS 252.227-7014</strong> — "AI Role" field documents the private/government development boundary<br>
<strong>EO 14028</strong> — treats AI output as a supply chain input requiring provenance<br>
<strong>NIST SP 800-218 (SSDF)</strong> — extends provenance tracking to AI-generated components<br>
<strong>SBOM (NTIA)</strong> — POA serves as an AI-aware extension to the Software Bill of Materials<br>
<span class="service-outcome">One framework, four compliance requirements addressed.</span>
</p>
</details>
</div>

<h2 class="services-section-head">Who Built This</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>The Cochran Block, LLC</summary>
<p>
Michael Cochran — Army veteran (17C Cyber Operations, 35Q start at JCAC 2013). 13 years defense and enterprise. USCYBERCOM J38 dev lead for a Congressional NDAA-directed offensive cyber operations study.<br><br>
SDVOSB pending. SAM.gov registered. Maryland eMMA vendor.<br><br>
This framework was not designed in a lab. It was built by a developer who needed to prove that his AI-assisted code was human-directed — and discovered that no standard existed to do so.<br>
<span class="service-outcome"><a href="https://github.com/cochranblock/provenance-docs/blob/main/WHITEPAPER.md">Read the full whitepaper →</a></span>
</p>
</details>
</div>

<p class="services-cta"><a href="/deploy" class="btn">Start a Project</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="https://github.com/cochranblock/provenance-docs" class="btn btn-secondary">GitHub</a></p>
</section>"#;
    Html(format!("{}{}{}{}", f62("provenance", "Provenance Docs — AI Development Documentation Framework | CochranBlock"), C7, v0, C8))
}

/// f12 = serve_about. Why: Tabbed Mission + Credentials (resume).
pub async fn f12(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v5 = r#"<div class="legacy-tab-content">
<div class="resume-actions">
  <button type="button" class="btn" id="copy-resume" onclick="navigator.clipboard.writeText(document.getElementById('resume-text').innerText).then(function(){document.getElementById('copy-resume').textContent='Copied.'})">Copy to Clipboard</button>
  <a href="/assets/resume.pdf" class="btn btn-secondary" download>Download PDF</a>
</div>
<p class="resume-hint">Step 1: Click "Copy to Clipboard"<br>Step 2: Paste it wherever you want<br>Step 3: There is no step 3</p>
<pre id="resume-text" class="resume-raw">
MICHAEL COCHRAN
Fractional CTO | Zero-Cloud Architect | Federal Whistleblower
mcochran@cochranblock.org | cochranblock.org | linkedin.com/in/cochranblock

────────────────────────────────────────────────────

SUMMARY

Fractional CTO who replaces bloated cloud infrastructure with
lean Rust binaries. cochranblock.org — my entire company — runs
as a single 18MB binary on a laptop for $10/month. 11+ years
defense and enterprise. 12 Unlicense repos proving
every claim. Edge compute beats cloud.

────────────────────────────────────────────────────

THE PROOF

cochranblock.org — live production site. Single Rust binary.
  $10/month total infrastructure. Zero AWS. Zero Kubernetes.
  12 open source repos: github.com/cochranblock
  Proof of Artifacts + Timeline of Invention in every repo.

────────────────────────────────────────────────────

EXPERIENCE

FOUNDER & FRACTIONAL CTO — CochranBlock                   2024–Present
  Zero-cloud architectures for startups and SMBs.
  Built 12 Rust products: augment engine (Kova), reverse proxy
  (approuter), on-device AI (Pixel Forge: 3 diffusion models
  under 30MB), ISO 8583 payment engine (Rogue Repo), testing
  framework (exopack), and cochranblock.org itself.
  Full CI via triple sims gate. No external test frameworks.
  Deploy: single binary + Cloudflare tunnel. $10/month.

SENIOR SYSTEMS ENGINEER — Enterprise Client               2024–Present
  Mission-critical systems. Custom Python for systems survey.
  SSH/Kerberos automation. Server service repair pipelines.

SENIOR SOFTWARE ENGINEER — Two Six Technologies           2022–2024
  Enterprise security integrations. YAML→SQL pipelines.
  Regex parsing, data sanitization, cross-system interop.

SYSTEMS DEVELOPER — Enterprise Security                   2020–2022
  APIs, CI/CD, documentation, data modeling, network protocols.
  GitLab, Docker, cross-team integration.

VULNERABILITY RESEARCH & RED TEAMING — USCYBERCOM J38     2017–2020
  J38 dev lead for Congressional NDAA-directed offensive cyber
  operations study (co-lead with J9 dev lead Jacob Crome).
  Product security. 100+ enterprise-scale deployments.
  Python automation. Red team initiative founder.
  "Thanks for building the groundwork for development here." — SFC Rios

SECURITY ANALYST — Product Security                       2014–2017
  Network mapping, analysis, security reporting.
  Laid foundation for JMOC-E development operations.

────────────────────────────────────────────────────

WHAT I REPLACE

  $5K/month AWS bill        →  $10/month laptop + tunnel
  Kubernetes + 5 services   →  Single Rust binary
  DevOps team               →  Cargo build + Cloudflare
  Microservices sprawl      →  One binary, all routes embedded
  Cloud vendor lock-in      →  Unlicensed, runs anywhere

────────────────────────────────────────────────────

SKILLS

Languages:  Rust, Python, C, C++, Assembly, Go, JavaScript
Security:   Penetration Testing, Vulnerability Research, Red Teaming
Infra:      Linux, Docker, Cloudflare, SSH/Kerberos, sled, SQLite
AI/ML:      On-device training (candle), LoRA, MoE diffusion models
Zero-Cloud: Single-binary architecture, edge compute, Axum, Tokio

────────────────────────────────────────────────────

CLEARANCE

Active — details available upon request.

────────────────────────────────────────────────────

REFERENCES

"You are one of the brightest people I ever had the pleasure
of working with. You're forged to thrive." — Carpenter, USCYBERCOM

"You taught others and left 'gifts' of code. We are better
for having you here." — Jay, USCYBERCOM

"Continue your legacy. The red team initiative is still alive."
— LTC Beal, USCYBERCOM
</pre>
</div>"#.to_string();

    let v6 = format!(
        r#"<section class="about"><h1>About CochranBlock</h1>
<div class="tabs" role="tablist">
  <button class="tab-btn active" data-tab="profile" role="tab" aria-selected="true">Mission</button>
  <button class="tab-btn" data-tab="legacy" role="tab" aria-selected="false">Credentials</button>
</div>
<div id="profile" class="tab-pane active" role="tabpanel" aria-hidden="false">
  <h3 class="profile-subhead">The Mission</h3>
  <p>Your server bill is too high. CochranBlock exists to prove it — and fix it. This entire company runs as a single Rust binary on a laptop for $10/month. We build zero-cloud architectures that replace bloated infrastructure with lean, fast binaries. All source code released under the Unlicense.</p>
  <h3 class="profile-subhead">What We Do</h3>
  <p>Fractional CTO services for startups and SMBs drowning in cloud costs. We audit your infrastructure, identify what you're overpaying for, and replace it with edge compute that you own. 12 open source Rust repos back every claim we make.</p>
  <h3 class="profile-subhead">The Architecture</h3>
  <p>Single-binary Rust. Embedded assets. No external databases for static sites. Cloudflare tunnel for internet exposure. Total cost: a laptop and $10/month. We've built an augment engine, a reverse proxy, on-device AI models, a payment engine, and a testing framework — all running this way.</p>
  <h3 class="profile-subhead">Founded By</h3>
  <p>Michael Cochran — Fractional CTO, Zero-Cloud Architect, Army veteran (17C Cyber Operations). 13 years defense and enterprise. SDVOSB pending. It's not the Mech — it's the pilot.</p>
  <h3 class="profile-subhead">What the Team Said</h3>
  <div class="testimonials-grid">
    <blockquote class="testimonial">"You are one of the brightest people I ever had the pleasure of working with. Your passion to elevate whatever you work on, coupled with your crazy research skills are something to aspire to. You're forged to thrive."<cite>— Carpenter, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"You taught others and left 'gifts' of code. We are better for having you here."<cite>— Jay, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"Awesome team player, dedicated fact finder, and loyal and honest teammate."<cite>— CPT Nate Durbala, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"I certainly hope you find the place that allows you to display your incredible talent. Thank you for your service."<cite>— Jeremy Ritz, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"Working with you made the JMOC-E a better place. I know you will continue to do great things."<cite>— TSgt Holland, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"Thanks for building the groundwork for development here."<cite>— SFC Rios, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"Thank you for pushing me to pursue becoming an expert in a programming language."<cite>— SSgt Muirhead, USCYBERCOM J38 JMOC-E</cite></blockquote>
    <blockquote class="testimonial">"Continue your legacy. The red team initiative is still alive."<cite>— LTC Beal, USCYBERCOM J38 JMOC-E</cite></blockquote>
  </div>
</div>
<div id="legacy" class="tab-pane legacy-pane" role="tabpanel" aria-hidden="true">
  {}
</div>
</section>
<script>
document.addEventListener('DOMContentLoaded',function(){{var btns=document.querySelectorAll('.tab-btn');var panes=document.querySelectorAll('.tab-pane');btns.forEach(function(btn){{btn.addEventListener('click',function(){{var tab=btn.getAttribute('data-tab');btns.forEach(function(b){{b.classList.remove('active');b.setAttribute('aria-selected','false')}});panes.forEach(function(p){{p.classList.remove('active');p.setAttribute('aria-hidden',p.id===tab?'false':'true');if(p.id===tab)p.classList.add('active')}});btn.classList.add('active');btn.setAttribute('aria-selected','true')}})}})}});
</script>"#,
        v5
    );
    Html(format!("{}{}{}{}", f62("about", "About | CochranBlock"), C7, v6, C8))
}

/// f13 = serve_contact. Why: Email CTA; no form friction.
pub async fn f13(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="contact"><h1>Contact</h1><p class="trust-badge">Product in development · Consulting: open</p><blockquote class="testimonial">"You are one of the brightest people I ever had the pleasure of working with. Your passion to elevate whatever you work on, coupled with your crazy research skills are something to aspire to."<cite>— Carpenter, USCYBERCOM J38 JMOC-E</cite></blockquote><p>Interested in our product roadmap, consulting, or a discovery call? Reach out by email.</p><p class="contact-micro">Product interest? Email with subject "Product Launch" to get notified when we ship.</p><p class="contact-micro">No form, no friction — just email.</p><p class="contact-cta"><a href="mailto:mcochran@cochranblock.org?subject=CochranBlock%20Inquiry" class="btn">Email Us</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="/assets/resume.pdf" class="btn btn-secondary" download>Michael's Resume</a></p><p class="contact-secondary">Or connect on <a href="https://www.linkedin.com/in/cochranblock" target="_blank" rel="noopener noreferrer">LinkedIn</a></p><p class="contact-note">We typically respond within 24–48 hours.</p></section>"#;
    Html(format!("{}{}{}{}", f62("contact", "Contact | CochranBlock"), C7, v0, C8))
}

/// f64 = get_date_slots — weekdays, 8am–5pm EST
fn f64() -> Vec<t20> {
    let today = Utc::now().with_timezone(&New_York).date_naive();
    let times = [
        "8:00am", "8:30am", "9:00am", "9:30am", "10:00am", "10:30am", "11:00am", "11:30am",
        "12:00pm", "12:30pm", "1:00pm", "1:30pm", "2:00pm", "2:30pm", "3:00pm", "3:30pm",
        "4:00pm", "4:30pm", "5:00pm",
    ];
    let mut date_slots = Vec::new();
    for day_offset in 0..90i64 {
        let date = today + Duration::days(day_offset);
        let wd = date.weekday();
        if wd != Weekday::Sat && wd != Weekday::Sun {
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
            let time_slots: Vec<t19> = times
                .iter()
                .map(|time| {
                    let subject = format!(
                        "Discovery%20Call%20Request%20-%20{}%20{}%20EST",
                        date_str.replace(' ', "%20"),
                        time.replace(':', "%3A")
                    );
                    t19 {
                        label: format!("{} EST", time),
                        mailto: format!("mailto:mcochran@cochranblock.org?subject={}", subject),
                    }
                })
                .collect();
            date_slots.push(t20 {
                date: date_iso,
                day_name: day_name.to_string(),
                date_label: date_str,
                times: time_slots,
            });
        }
        if date_slots.len() >= 60 {
            break;
        }
    }
    date_slots
}

/// f63 = serve_book. Why: Calendar + time slots for discovery calls; Braintrust flow.
pub async fn f63(State(_p0): State<Arc<t0>>) -> Html<String> {
    let date_slots = f64();
    let slots_json = serde_json::to_string(&date_slots).unwrap_or_default();
    let slots_json_escaped = slots_json.replace('<', "\\u003c").replace('>', "\\u003e");
    let v0 = format!(
        r##"<section class="booking-page"><h1>Schedule a Discovery Call</h1><p class="booking-intro">30 minutes · Discuss your goals and how I can help · All times Eastern (EST)</p><p class="booking-context">Shared via Braintrust — pick a date, then a time. Your email will open with a pre-filled request.</p><p class="booking-legend">Available weekdays · 8am–5pm EST</p><p class="booking-hint" id="booking-hint">Select a date, then a time.</p><a href="#booking-calendar" class="skip-link skip-link-calendar">Skip to calendar</a><div class="booking-calendar-wrapper" id="booking-calendar" role="region" aria-label="Calendar - pick a date then a time" aria-describedby="booking-hint"><div class="booking-calendar-header"><div class="booking-month-row"><button type="button" class="booking-nav" id="booking-prev" aria-label="Previous month">&larr;</button><h3 class="booking-month" id="booking-month"></h3><button type="button" class="booking-nav" id="booking-next" aria-label="Next month">&rarr;</button><span class="booking-available-badge" id="booking-available-badge"></span></div></div><div class="booking-calendar-grid" id="booking-grid" role="grid" aria-label="Month view"></div></div><div class="booking-time-panel" id="booking-time-panel" aria-live="polite" hidden><h3 class="booking-time-heading" id="booking-time-heading"></h3><div class="booking-time-slots" id="booking-time-slots" role="group"></div></div><p class="booking-note">I'll confirm within 24 hours.</p><p class="booking-fallback">None of these work? <a href="mailto:mcochran@cochranblock.org?subject=Discovery%20Call%20Request">Email me</a> to propose a time.</p><script type="application/json" id="booking-slots-data">{}</script><script src="/assets/js/booking.js"></script></section>"##,
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

/// f67 = serve_products. Why: Three-tier product catalog — Platforms, Partnerships, Open Source.
pub async fn f67(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="products"><h1>Products</h1><p class="products-intro">Everything we build. Platforms we sell, partners we power, and open source we give away.</p>

<h2 class="products-category">Platforms</h2>
<p class="products-category-desc">Commercial products. Rust-only. Offline-first. Priced to kill the big guys.</p>
<div class="product-cards">
<article class="product-card"><span class="product-badge">Coming Soon</span><a href="https://roguerepo.io" rel="noopener noreferrer"><img src="/assets/img/rogue-repo.png" alt="Rogue Repo" class="product-img" width="400" height="300"></a><h2><a href="https://roguerepo.io" rel="noopener noreferrer">Rogue Repo</a></h2><p>Rust-only app store. No JavaScript tax. No cloud lock-in. Offline-first, creative mode, superior pricing. The anti-enterprise app store.</p></article>
<article class="product-card"><span class="product-badge">Coming Soon</span><a href="https://ronin-sites.pro" rel="noopener noreferrer"><img src="/assets/img/ronin-sites.png" alt="Ronin Sites" class="product-img" width="400" height="300"></a><h2><a href="https://ronin-sites.pro" rel="noopener noreferrer">Ronin Sites</a></h2><p>Shop and artist platform. Subdomain routing, MinIO storage, mobile-first site tuner. For creators who refuse to pay Shopify prices.</p></article>
</div>

<h2 class="products-category">Hardware</h2>
<p class="products-category-desc">Your website lives on your phone. No hosting bill. Ever.</p>
<div class="product-cards">
<article class="product-card"><span class="product-badge">Coming Soon</span><h2>Pocket Server</h2><p>A compiled Rust web server running on a phone you own. Bold kiosk dashboard shows live stats — visitors, uptime, power draw. Plug it in, it's live. No cloud. No monthly fee. $2.60/year in electricity.</p>
<div class="pricing-cards" style="margin-top:1rem">
<article class="pricing-card" style="margin:0"><span class="product-badge">BYOD</span><div class="pricing-amount">$500</div><div class="pricing-unit">bring your own phone</div></article>
<article class="pricing-card" style="margin:0"><span class="product-badge">Starter</span><div class="pricing-amount">$750</div><div class="pricing-unit">refurb phone + configured</div></article>
<article class="pricing-card" style="margin:0"><span class="product-badge">Turnkey</span><div class="pricing-amount">$1,000</div><div class="pricing-unit">new phone + domain + ready to plug in</div></article>
</div>
</article>
<article class="product-card"><span class="product-badge">Coming Soon</span><h2><a href="https://github.com/cochranblock/ghost-fabric">Ghost Fabric</a></h2><p>Sovereign edge intelligence over LoRa mesh networks. 19MB Rust binary with embedded AI, running on bare metal nodes at 915MHz. Sensors that think. Networks that survive. No cloud dependency.</p></article>
</div>

<h2 class="products-category">Business Partnerships</h2>
<p class="products-category-desc">We build for partners who share the mission. Their brand, our engine.</p>
<div class="product-cards">
<article class="product-card"><span class="product-badge">Live</span><a href="https://oakilydokily.com" rel="noopener noreferrer"><h2><a href="https://oakilydokily.com" rel="noopener noreferrer">oakilydokily.com</a></h2></a><p>Waiver management, digital intake, and resume platform. Rust backend, zero bloat, deployed on local hardware via Cloudflare Zero Trust. First paying partnership.</p></article>
</div>

<h2 class="products-category">Open Source</h2>
<p class="products-category-desc">Free. Copy-left. Use it, fork it, ship it. We build in the open.</p>
<div class="product-cards">
<article class="product-card"><span class="product-badge">Active</span><a href="https://github.com/cochranblock/pixel-forge" rel="noopener noreferrer"><img src="/assets/img/pixel-forge.png" alt="Pixel Forge" class="product-img" width="400" height="300"></a><h2><a href="https://github.com/cochranblock/pixel-forge" rel="noopener noreferrer">Pixel Forge</a></h2><p>Free pixel art generator. Three on-device diffusion models (Cinder 4.2 MB / Quench 22 MB / Anvil 64 MB) generate, judge, and arrange 16&times;16 sprites into game scenes. MoE cascade, LoRA fine-tuning, scene builder. No cloud. No subscription. Pure Rust.</p></article>
<article class="product-card"><span class="product-badge">Active</span><a href="https://github.com/cochranblock/kova" rel="noopener noreferrer"><img src="/assets/img/kova.png" alt="Kova" class="product-img" width="400" height="300"></a><h2><a href="https://github.com/cochranblock/kova" rel="noopener noreferrer">Kova</a></h2><p>Augment engine. AI Bending — human directs, AI executes. Local LLM, egui GUI, agent loop with tool use. The brain behind everything we ship.</p></article>
<article class="product-card"><span class="product-badge">Active</span><h2><a href="https://github.com/cochranblock/illbethejudgeofthat" rel="noopener noreferrer">illbethejudgeofthat</a></h2><p>Pro se custody case builder. Google Takeout to court-ready exhibit book + filled forms in one evening. Built by a father who needed it.</p></article>
<article class="product-card"><span class="product-badge">Active</span><h2><a href="https://github.com/cochranblock/approuter" rel="noopener noreferrer">approuter</a></h2><p>Reverse proxy. All products behind one entry point. Cloudflare tunnel integration. Zero-config service registration.</p></article>
<article class="product-card"><span class="product-badge">Active</span><h2><a href="https://github.com/cochranblock/cochranblock" rel="noopener noreferrer">cochranblock</a></h2><p>This site. Rust + Axum. No templates, no JavaScript frameworks. Embedded HTML, zstd-packed assets, single binary. The website is the product demo.</p></article>
</div>

<p class="products-cta"><a href="/deploy" class="btn">Deploy With Us</a><a href="/codeskillz" class="btn btn-secondary">See All 14 Repos Live</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("products", "Products | CochranBlock"), C7, v0, C8))
}

/// f68 = serve_downloads. Why: Binary downloads page — the site IS the app.
pub async fn f68(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="services"><h1>Download cochranblock</h1>
<p class="services-intro">The same binary running this site right now. Download it, run it, and the entire production site opens in your browser. No Docker. No npm. No configuration. One binary, one command.</p>

<div class="service-cards">
<details class="service-card" open>
<summary>macOS (Apple Silicon)</summary>
<p>M1, M2, M3, M4. 9.8 MB binary.<span class="service-outcome"><a href="https://github.com/cochranblock/cochranblock/releases/latest/download/cochranblock-macos-arm64" class="btn">Download</a></span></p>
<pre>chmod +x cochranblock-macos-arm64
./cochranblock-macos-arm64</pre>
</details>

<details class="service-card" open>
<summary>Linux x86_64 (Debian / Ubuntu)</summary>
<p>Any modern 64-bit Linux. 17 MB binary.<span class="service-outcome"><a href="https://github.com/cochranblock/cochranblock/releases/latest/download/cochranblock-linux-amd64" class="btn">Download</a></span></p>
<pre>chmod +x cochranblock-linux-amd64
./cochranblock-linux-amd64</pre>
</details>

<details class="service-card">
<summary>Cargo Install (Any Platform)</summary>
<p>If you have Rust installed, this is the easiest way.<span class="service-outcome">Works on macOS, Linux, Windows, ARM, x86.</span></p>
<pre>cargo install --git https://github.com/cochranblock/cochranblock</pre>
</details>

<details class="service-card">
<summary>Coming Soon</summary>
<p>Windows (.exe), Debian (.deb), macOS Intel, Linux ARM, iOS (PWA), Android (PWA).<span class="service-outcome">All from the same codebase. Zero framework tax.</span></p>
</details>
</div>

<h2 class="services-section-head">What You're Getting</h2>
<div class="service-cards">
<details class="service-card" open>
<summary>The full cochranblock.org site — locally</summary>
<p>Every page, every asset, every route. Embedded CSS, JS, images (zstd-packed). SQLite for persistence (auto-creates on first run). Opens your browser to localhost:8081.<span class="service-outcome">Outcome: You're running an entire company's infrastructure on your machine.</span></p>
</details>
<details class="service-card">
<summary>Zero dependencies</summary>
<p>No runtime. No interpreter. No package manager. No database server. The binary IS the server, the database, and the asset pipeline. That's the point.<span class="service-outcome">Outcome: The same architecture I build for clients. This is the live demo.</span></p>
</details>
</div>

<p class="services-cta"><a href="https://github.com/cochranblock/cochranblock" class="btn">View Source</a><a href="/deploy" class="btn btn-secondary">Zero-Cloud Tech Intake</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("downloads", "Download | CochranBlock"), C7, v0, C8))
}

// f68_old removed — federal-partners page retired.

/// f10 = health. Why: Load balancer / approuter liveness probe.
pub async fn f10(State(_p0): State<Arc<t0>>) -> &'static str {
    "OK"
}

/// f73 = api_stats. Why: Live metrics for social proof, outreach, and monitoring.
pub async fn f73(State(p0): State<Arc<t0>>) -> impl axum::response::IntoResponse {
    let lead_count: i64 = if let Some(ref pool) = p0.intake_pool {
        sqlx::query_scalar("SELECT COUNT(*) FROM leads")
            .fetch_one(pool)
            .await
            .unwrap_or(0)
    } else {
        0
    };
    let grant_count: i64 = if let Some(ref pool) = p0.intake_pool {
        sqlx::query_scalar("SELECT COUNT(*) FROM community_grants")
            .fetch_one(pool)
            .await
            .unwrap_or(0)
    } else {
        0
    };
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let json = format!(
        r#"{{"binary_size_arm":"9.9MB","binary_size_x86":"18MB","monthly_cost":"$10","repos":12,"leads":{},"grants":{},"timestamp":{}}}"#,
        lead_count, grant_count, uptime
    );
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        json,
    )
}

/// f75 = api_velocity. Why: Live last-push timestamps from GitHub for all repos — proves velocity.
/// Caches in memory for 30 minutes to avoid GitHub rate limits.
pub async fn f75(State(_p0): State<Arc<t0>>) -> impl axum::response::IntoResponse {
    use std::sync::OnceLock;
    use std::sync::Mutex;

    static CACHE: OnceLock<Mutex<(String, std::time::Instant)>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new((String::new(), std::time::Instant::now() - std::time::Duration::from_secs(9999))));

    // Return cached if fresh (30 min)
    {
        let guard = cache.lock().unwrap();
        if guard.1.elapsed().as_secs() < 1800 && !guard.0.is_empty() {
            return (
                axum::http::StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, "application/json"),
                 (axum::http::header::CACHE_CONTROL, "public, max-age=1800")],
                guard.0.clone(),
            );
        }
    }

    let repos = [
        "cochranblock", "ghost-fabric", "kova", "pixel-forge", "approuter",
        "oakilydokily", "illbethejudgeofthat", "exopack", "rogue-repo",
        "wowasticker", "whyyoulying", "pocket-server", "provenance-docs", "call-shield",
    ];

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, "cochranblock/1.0".parse().unwrap());
    if !token.is_empty() {
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    }

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let mut entries = Vec::new();
    for repo in repos {
        let url = format!("https://api.github.com/repos/cochranblock/{}", repo);
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(body) = resp.text().await {
                if let Some(start) = body.find("\"pushed_at\":\"") {
                    let rest = &body[start + 13..];
                    if let Some(end) = rest.find('"') {
                        let pushed = &rest[..end];
                        entries.push(format!(r#"{{"repo":"{}","pushed_at":"{}"}}"#, repo, pushed));
                    }
                }
            }
        }
    }
    let json = format!(r#"{{"repos":[{}],"count":{}}}"#, entries.join(","), entries.len());

    // Update cache
    {
        let mut guard = cache.lock().unwrap();
        *guard = (json.clone(), std::time::Instant::now());
    }

    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json"),
         (axum::http::header::CACHE_CONTROL, "public, max-age=1800")],
        json,
    )
}

/// f71 = handler_404. Why: Site-styled 404 instead of axum default.
pub async fn f71(State(_p0): State<Arc<t0>>) -> (axum::http::StatusCode, Html<String>) {
    let body = r#"<section class="contact"><h1>Page Not Found</h1><p>The page you're looking for doesn't exist or has moved.</p><p class="contact-cta"><a href="/" class="btn">Back to Home</a><a href="/contact" class="btn btn-secondary">Get in Touch</a></p></section>"#;
    let html = format!("{}{}{}{}", f62("404", "Page Not Found | CochranBlock"), C7, body, C8);
    (axum::http::StatusCode::NOT_FOUND, Html(html))
}

/// f60 = html_escape. Why: XSS prevention for user content.
fn f60(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// f61 = strip_frontmatter. Why: Markdown-style frontmatter removal for embedded content.
fn f61(s: &str) -> &str {
    if let Some(v0) = s.strip_prefix("---\n") {
        if let Some(v1) = v0.find("\n---\n") {
            return s.get(v1 + 5..).unwrap_or(s).trim_start();
        }
    }
    s
}

/// f57 = serve_source — developer source view
pub async fn f57(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v_test = include_str!("../bin/cochranblock-test.rs");
    let v_tunnel = include_str!("../tunnel.rs");
    let v_router = include_str!("router.rs");
    let v0 = format!(
        r#"<section class="developer-page"><h1>Source Code</h1><p class="page-intro">Key files. Tokenized identifiers (f0, t0, etc.) per compression map.</p>
<details class="source-file" open><summary><span>cochranblock-test.rs</span><button type="button" class="copy-btn" data-copy="test" aria-label="Copy">Copy</button></summary><pre class="source-code" id="test"><code>{}</code></pre></details>
<details class="source-file"><summary><span>tunnel.rs</span><button type="button" class="copy-btn" data-copy="tunnel" aria-label="Copy">Copy</button></summary><pre class="source-code" id="tunnel"><code>{}</code></pre></details>
<details class="source-file"><summary><span>router.rs</span><button type="button" class="copy-btn" data-copy="router" aria-label="Copy">Copy</button></summary><pre class="source-code" id="router"><code>{}</code></pre></details>
</section><script>document.querySelectorAll('.copy-btn').forEach(function(b){{b.onclick=function(e){{e.stopPropagation();var p=document.getElementById(b.getAttribute('data-copy'));if(p&&navigator.clipboard){{navigator.clipboard.writeText(p.textContent);b.textContent='Copied!';setTimeout(function(){{b.textContent='Copy';}},1500);}}}}}});</script>"#,
        f60(v_test),
        f60(v_tunnel),
        f60(v_router)
    );
    Html(format!(
        "{}{}{}{}",
        f62("source", "Source Code | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f58 = serve_executive_summary
pub async fn f58(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/executive_summary.html");
    Html(format!(
        "{}{}{}{}",
        f62("executive-summary", "Executive Summary | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f65 = serve_ai_orchestration
pub async fn f65(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/ai_orchestration.html");
    Html(format!(
        "{}{}{}{}",
        f62("ai-orchestration", "AI Orchestration | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f66 = serve_prompts
pub async fn f66(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/prompt_reference.html");
    Html(format!(
        "{}{}{}{}",
        f62("prompts", "Prompt Reference | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f59 = serve_rules — cursor rules view
pub async fn f59(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v_rebuild = include_str!("../../.cursor/rules/rebuild-redeploy.mdc");
    let v_token = include_str!("../../.cursor/rules/tokenization.mdc");
    let v_anti = include_str!("../../.cursor/rules/anti-patterns.mdc");
    let v_ci = include_str!("../../.cursor/rules/ci-pipeline.mdc");
    let v_test = include_str!("../../.cursor/rules/test-binary.mdc");
    let v_prod = include_str!("../../.cursor/rules/production-binary.mdc");
    let v0 = format!(
        r#"<section class="developer-page"><h1>Cursor Rules</h1><p class="page-intro">Project rules that govern AI assistance on this codebase.</p>
<details class="rule-file" open><summary>rebuild-redeploy.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>tokenization.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>anti-patterns.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>ci-pipeline.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>test-binary.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>production-binary.mdc</summary><pre class="rule-content">{}</pre></details>
</section>"#,
        f60(f61(v_rebuild)),
        f60(f61(v_token)),
        f60(f61(v_anti)),
        f60(f61(v_ci)),
        f60(f61(v_test)),
        f60(f61(v_prod))
    );
    Html(format!(
        "{}{}{}{}",
        f62("cursor-rules", "Cursor Rules | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}