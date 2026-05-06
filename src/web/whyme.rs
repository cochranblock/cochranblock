// SPDX-License-Identifier: Unlicense
// This is free and unencumbered software released into the public domain.
// For more information, please refer to <https://unlicense.org>
//
//! whyme.cochranblock.org — the case for hiring me, in one page.
//!
//! Anchored on the barber thesis: software is a craft like tattooing or
//! cutting hair — the ink is fungible, the artist is not, the patron hires
//! the artist. Below the thesis: live receipts from across the portfolio.
//!
//! Hosted at `whyme.cochranblock.org` (and reachable as `/whyme` from the
//! main site). Dispatched from `pages::f2_root` on Host-header match.

use axum::response::Html;

/// Render the whyme page. Static HTML — counters update client-side via
/// the same crates.io / GitHub fetches the home page uses.
pub async fn page() -> Html<&'static str> {
    Html(BODY)
}

const BODY: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Why hire Michael Cochran — cochranblock.org</title>
<meta name="description" content="Tattoo artists don't patent their tattoos. Software is the same kind of craft. 32 public repos, 22 shipping Rust, 1,235 verified cargo-test passes, all public domain. The portfolio is the pitch.">
<link rel="canonical" href="https://whyme.cochranblock.org/">
<link rel="icon" href="/assets/favicon.svg" type="image/svg+xml">
<style>
:root{
  --bg:#0a0a0e;--surface:#11111a;--text:#e8e8e8;--muted:#9aa0aa;
  --accent:#00d9ff;--rule:rgba(255,255,255,.10);--max:760px;
}
*{box-sizing:border-box;margin:0;padding:0}
html,body{background:var(--bg);color:var(--text)}
body{
  font:17px/1.65 -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,sans-serif;
  padding:3rem 1.25rem 5rem;
}
.wrap{max-width:var(--max);margin:0 auto}
nav.crumbs{font-size:.85rem;color:var(--muted);margin-bottom:2rem}
nav.crumbs a{color:var(--muted);text-decoration:underline}
header.hero{padding-bottom:2rem;border-bottom:1px solid var(--rule);margin-bottom:2rem}
.tag{display:inline-block;font-size:.78rem;letter-spacing:.08em;text-transform:uppercase;color:var(--accent);margin-bottom:1rem}
header.hero h1{
  font:600 2.1rem/1.15 -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
  letter-spacing:-.02em;margin-bottom:.9rem;
}
header.hero h1 em{font-style:normal;color:var(--accent)}
header.hero .lede{color:var(--muted);font-size:1.05rem;margin-bottom:1.2rem}
section.thesis p{margin-bottom:1.05rem}
section.thesis blockquote{
  border-left:3px solid var(--accent);padding:.5rem 0 .5rem 1.1rem;
  margin:1.4rem 0;color:#f0f0f0;font-size:1.08rem;
}
h2{font:600 1.25rem/1.3 -apple-system,sans-serif;letter-spacing:-.01em;margin:2.2rem 0 .9rem;color:#fff}
h2 .small{display:block;font-size:.78rem;letter-spacing:.08em;text-transform:uppercase;color:var(--muted);font-weight:500;margin-bottom:.3rem}
.receipts{
  display:grid;grid-template-columns:repeat(auto-fit,minmax(140px,1fr));gap:.9rem;
  background:var(--surface);border:1px solid var(--rule);border-radius:8px;
  padding:1.1rem 1.2rem;margin:.6rem 0 1.6rem;
}
.receipts .stat{display:flex;flex-direction:column;gap:.15rem}
.receipts .num{font:600 1.55rem/1 -apple-system,sans-serif;color:var(--accent);letter-spacing:-.02em}
.receipts .label{font-size:.78rem;color:var(--muted)}
.products{display:grid;grid-template-columns:1fr;gap:.85rem;margin-bottom:1.5rem}
.product{
  background:var(--surface);border:1px solid var(--rule);border-radius:8px;
  padding:1rem 1.15rem;
}
.product h3{font:600 1rem/1.3 -apple-system,sans-serif;margin-bottom:.35rem;color:#fff}
.product h3 a{color:#fff;text-decoration:none}
.product h3 a:hover{color:var(--accent)}
.product .meta{font-size:.8rem;color:var(--muted);margin-bottom:.45rem}
.product p{font-size:.94rem;color:#d4d4d8}
ul.bullets{margin:.4rem 0 1.2rem 1.2rem}
ul.bullets li{margin-bottom:.4rem}
.cta{
  margin-top:2.4rem;padding-top:1.6rem;border-top:1px solid var(--rule);
  display:flex;flex-wrap:wrap;gap:.6rem;align-items:center;
}
.cta a{
  display:inline-block;padding:.55rem 1rem;border-radius:6px;
  text-decoration:none;font-size:.9rem;font-weight:500;
  background:var(--accent);color:#000;
}
.cta a.secondary{background:transparent;color:var(--accent);border:1px solid var(--accent)}
.cta a:hover{filter:brightness(1.08)}
footer.foot{margin-top:3rem;padding-top:1.5rem;border-top:1px solid var(--rule);font-size:.8rem;color:var(--muted)}
footer.foot a{color:var(--muted);text-decoration:underline}
@media(max-width:480px){
  body{padding:2rem 1rem 4rem}
  header.hero h1{font-size:1.7rem}
}
</style>
</head>
<body>
<div class="wrap">

<nav class="crumbs">
  <a href="https://cochranblock.org">cochranblock.org</a> ·
  <a href="https://github.com/cochranblock">github.com/cochranblock</a>
</nav>

<header class="hero">
  <span class="tag">why hire me</span>
  <h1>Tattoo artists don't patent their tattoos.<br><em>Software is the same kind of craft.</em></h1>
  <p class="lede">Chefs don't patent recipes. Comedians don't patent jokes. Barbers don't patent fades. They compete on hand, eye, taste, relationship, speed, and trust. The work is the moat.</p>
</header>

<section class="thesis">
  <p>I think software is a negative-space industry — the kind where copying is legal, easy, and constant, and the field gets <em>more</em> creative because of it, not less. The patron isn't buying the design. They're buying the artist who can pull it off and the next one and the next one.</p>

  <p>So I ship in public domain. Unlicense on every repo. No defensive patents, no proprietary moat, no five-page commercial license. The pitch is the code, the cadence, the taste, the speed, and the fact that you can read every line before you decide.</p>

  <blockquote>I'd rather get hired for the work than file paperwork.</blockquote>

  <p>13 years in enterprise security and product. Last 60 days: 22 public-domain Rust repos. The numbers below update live from crates.io and GitHub.</p>
</section>

<h2><span class="small">Receipts · last 60 days · all verifiable</span>What I've shipped</h2>
<div class="receipts">
  <div class="stat"><span class="num" id="wm-repos">32</span><span class="label">public repos in the org</span></div>
  <div class="stat"><span class="num">22</span><span class="label">of those ship Rust code</span></div>
  <div class="stat"><span class="num" id="wm-crates">22</span><span class="label">crates on crates.io</span></div>
  <div class="stat"><span class="num">1,290</span><span class="label">commits across the portfolio</span></div>
  <div class="stat"><span class="num">1,235</span><span class="label"><code>cargo test</code> passing</span></div>
  <div class="stat"><span class="num">9</span><span class="label">currently failing — being fixed</span></div>
</div>
<p style="font-size:.85rem;color:var(--muted);margin-top:-.7rem;margin-bottom:1.6rem">Test counts measured by running <code>cargo test --workspace --no-fail-fast</code> across every cochranblock repo and counting the literal "test result: ok" lines. 1,235 passing as of last sweep. 9 failing — they fail because I'm building. Broken tests mean progress, not failure. The largest single contributor is <a href="https://github.com/cochranblock/runsible" style="color:var(--accent)">runsible</a> at 576 tests. Reproduce the count yourself: <code>for r in $(ls); do (cd $r &amp;&amp; cargo test --workspace 2&gt;&amp;1 | grep "^test result"); done</code>.</p>

<h2><span class="small">Three of them</span>The mechs I've built</h2>
<div class="products">

  <div class="product">
    <h3><a href="https://github.com/cochranblock/runsible">runsible</a> — Ansible, reimagined in Rust</h3>
    <div class="meta">14 crates · 576 tests · 33 hours · Unlicense</div>
    <p>~10ms cold-start vs Ansible's 1–3s. age + SSH-key per-recipient vault replaces the symmetric password-file model. Just-in-time SSH user certificates: each task mints a 60-second cert from a CA key. Idempotence enforced at the type system via <code>plan() → apply() → verify()</code>.</p>
  </div>

  <div class="product">
    <h3><a href="https://github.com/cochranblock/for-tanner">for-tanner</a> — n8n, reimagined in Rust</h3>
    <div class="meta">616 nodes · 12 MB binary · 40 hours · Unlicense</div>
    <p>Single binary serves an embedded WASM canvas (Leptos) over axum. Real integrations: hand-rolled SigV4 for AWS S3 (no aws-sdk dep weight), pure-Rust Kafka (rskafka), AMQP (lapin), MongoDB, Redis, MQTT. TypeScript→Rust codegen pipeline auto-generates 580 nodes from upstream n8n source.</p>
  </div>

  <div class="product">
    <h3><a href="https://github.com/cochranblock/exopack">exopack</a> — Testing capabilities for Rust binaries</h3>
    <div class="meta">Used by 16 repos · TRIPLE SIMS gating · Unlicense</div>
    <p>Augments any Rust binary with screenshot capture, video recording, interface harness, API mocking, and TRIPLE SIMS determinism gating. Each sibling repo ships a <code>&lt;crate&gt;-test</code> binary that runs an <code>f30()</code> entrypoint three times via <code>exopack::triple_sims::f60</code>; all three must pass.</p>
  </div>

</div>

<h2><span class="small">The pilot ethic</span>How I work</h2>
<ul class="bullets">
  <li><strong>The work is the pitch.</strong> No NDAs, no demo gates, no "let me get on a call to show you" — read the code right now, every line, public domain.</li>
  <li><strong>Speed and rigor aren't a tradeoff.</strong> 33 hours to build a 14-crate Ansible replacement is the same 33 hours that produces 576 passing tests and 14 deterministic provenance gates. The constraint forces clarity, not corner-cutting.</li>
  <li><strong>I refuse to compete on paperwork.</strong> No defensive patents, no proprietary licenses, no exclusivity riders. If somebody copies the work, that's the work doing what it should.</li>
  <li><strong>Only pilots fly.</strong> Competence is domain-specific. I won't take work I'm not qualified to do, and I won't keep work I've outgrown my currency on. The standard applies to me too.</li>
  <li><strong>Documents the trail.</strong> Every commit is signed and dated. <code>TIMELINE_OF_INVENTION.md</code> + <code>PROOF_OF_ARTIFACTS.md</code> in every repo log what was built, why, and what's verifiable.</li>
</ul>

<h2><span class="small">If this is your kind of engineer</span>Hire me</h2>
<p>Open to senior IC, staff, and fractional CTO conversations. Public-domain portfolio is on GitHub. Nothing held back, nothing under proprietary license, nothing requiring a meeting to evaluate.</p>

<div class="cta">
  <a href="mailto:mcochran@cochranblock.org?subject=Hiring%20Inquiry">Email</a>
  <a class="secondary" href="https://github.com/cochranblock">GitHub</a>
  <a class="secondary" href="https://www.linkedin.com/in/cochranblock">LinkedIn</a>
  <a class="secondary" href="/assets/michael-cochran-resume_may_2026.pdf" download>Resume (PDF)</a>
</div>

<footer class="foot">
  <p>The portfolio is the pitch. Everything on this page is public domain. <a href="https://github.com/cochranblock">Read the source.</a></p>
  <p style="margin-top:.6rem">— Michael Cochran · cochranblock.org · Unlicense</p>
</footer>

</div>

<script>
// Live receipts: pull current public-repo and crate counts from the GitHub
// + crates.io public APIs. Failures fall back to the embedded values above.
(function(){
  fetch('https://api.github.com/users/cochranblock/repos?per_page=100&type=public').then(function(r){return r.json();}).then(function(d){
    if(!Array.isArray(d)) return;
    var n = d.filter(function(r){ return !r.fork; }).length;
    var el = document.getElementById('wm-repos'); if(el) el.textContent = n;
  }).catch(function(){});
  fetch('https://crates.io/api/v1/users/gotemcoach').then(function(r){return r.json();}).then(function(u){
    var uid = u && u.user && u.user.id; if(!uid) return;
    return fetch('https://crates.io/api/v1/crates?user_id=' + uid + '&per_page=100');
  }).then(function(r){ return r ? r.json() : null; }).then(function(d){
    if(!d || !d.crates) return;
    var el = document.getElementById('wm-crates'); if(el) el.textContent = d.crates.length;
  }).catch(function(){});
})();
</script>
</body>
</html>"##;
