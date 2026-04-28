// Unlicense — cochranblock.org
//! Anti-Founder Captain Logs — personal blog.
//!
//! Hosted at `captainslog.cochranblock.org`. Posts are static, embedded in
//! the binary at compile time (no DB, no CMS, no markdown renderer — just
//! authored HTML inlined below). Add a new entry by appending to `POSTS`
//! and rebuilding.

use axum::response::Html;

struct Post {
    slug: &'static str,
    title: &'static str,
    date: &'static str,
    body: &'static str,
}

const POSTS: &[Post] = &[Post {
    slug: "001-burn-the-pricing-page",
    title: "Captain's Log #001 — Burn the pricing page.",
    date: "2026-04-28",
    body: r##"<p>Today I deleted a pricing page. Four tiers. Starter, Growth, Scale, Custom. $150 to $1,500. The grid every founder builds because every founder before them built one.</p>

<p>I'm not selling whobelooking anymore. The product is free. The repo is Unlicense. The site is a demo of what runs when you clone it.</p>

<p>This isn't a marketing pivot. It's the actual move. The software market has too many founders charging $200/month for a hashmap with a chart. I'm tired of watching mediocre Node bloat get a Series A while the people who can actually ship 23 Rust binaries from a laptop have to scramble for a fractional CTO retainer.</p>

<blockquote>Eat the founder software market with free software. Double it as a resume.</blockquote>

<p>So that's what happens here. Anti-Founder Captain Logs. One operator, fifteen repos, one laptop, ten dollars a month. Free, every product, no tiers, no usage caps. n8n, Calendly, Mailchimp, Bitly, Plausible — pick one a week and ship the Rust binary that makes their pricing page look like a joke.</p>

<p>Start with whobelooking. Already up. Already free. Already running. Coming soon: it does the compute in your browser, so even my laptop bill drops.</p>

<p>This log exists because the work needs to be visible. Receipts, in public, dated.</p>

<p><em>— mc · stardate 2026.04.28</em></p>"##,
}];

const STYLE: &str = r#"<style>
:root{--bg:#0c0c10;--text:#e8e8e8;--muted:#888;--accent:#00d9ff;--rule:rgba(255,255,255,.08)}
*{box-sizing:border-box;margin:0;padding:0}
body{background:var(--bg);color:var(--text);font:16px/1.7 'Iowan Old Style','Palatino Linotype',Georgia,serif;padding:3rem 1.25rem}
.wrap{max-width:680px;margin:0 auto}
header.site{border-bottom:1px solid var(--rule);padding-bottom:1.5rem;margin-bottom:2.5rem}
header.site h1{font-family:'Iowan Old Style',Georgia,serif;font-size:1.6rem;font-style:italic;letter-spacing:-.01em;margin-bottom:.4rem}
header.site h1 a{color:var(--text);text-decoration:none}
header.site .sub{color:var(--muted);font-size:.85rem;font-style:italic}
nav.crumbs{font-size:.8rem;color:var(--muted);margin-bottom:1.5rem}
nav.crumbs a{color:var(--muted);text-decoration:underline}
article{margin-bottom:3rem}
.entry-meta{color:var(--muted);font-size:.72rem;text-transform:uppercase;letter-spacing:.14em;margin-bottom:.4rem}
.entry-title{font-family:'Iowan Old Style',Georgia,serif;font-size:1.35rem;font-weight:400;font-style:italic;margin-bottom:.6rem}
.entry-title a{color:var(--text);text-decoration:none;border-bottom:1px solid transparent}
.entry-title a:hover{border-bottom-color:var(--accent)}
.entry-body p{margin-bottom:1.1rem}
.entry-body a{color:var(--accent)}
.entry-body em{font-style:italic;color:#bbb}
.entry-body strong{color:#fff}
.entry-body blockquote{border-left:2px solid var(--accent);padding-left:1rem;margin:1.5rem 0;color:#bbb;font-style:italic}
.entry-body code{background:#222;padding:1px 5px;border-radius:2px;font:.85em ui-monospace,monospace}
.entry-body hr{border:0;border-top:1px solid var(--rule);margin:2rem 0}
footer.site{border-top:1px solid var(--rule);padding-top:1.5rem;margin-top:3rem;color:var(--muted);font-size:.78rem;font-style:italic}
footer.site a{color:var(--muted);text-decoration:underline}
</style>"#;

pub async fn index() -> Html<String> {
    let mut entries = String::new();
    for p in POSTS {
        entries.push_str(&format!(
            r#"<article>
<div class="entry-meta">{date}</div>
<h2 class="entry-title"><a href="/{slug}">{title}</a></h2>
<div class="entry-body">{body}</div>
</article>"#,
            date = p.date,
            slug = p.slug,
            title = p.title,
            body = p.body,
        ));
    }

    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Anti-Founder Captain Logs</title>
<meta name="description" content="One operator, fifteen repos, ten dollars a month. Receipts in public, dated. By Michael Cochran of cochranblock.org.">
<link rel="canonical" href="https://captainslog.cochranblock.org/">
{STYLE}
</head><body>
<div class="wrap">
<header class="site">
<h1><a href="/">Anti-Founder Captain Logs</a></h1>
<div class="sub">One operator. Fifteen repos. Ten dollars a month. Receipts in public, dated.</div>
</header>
{entries}
<footer class="site">
By <a href="https://cochranblock.org">Michael Cochran</a> &middot; <a href="https://github.com/cochranblock">github.com/cochranblock</a> &middot; Unlicense, public domain.
</footer>
</div>
</body></html>"#
    ))
}

pub async fn post(slug: &str) -> axum::response::Response {
    use axum::response::IntoResponse;
    let Some(p) = POSTS.iter().find(|p| p.slug == slug) else {
        return (axum::http::StatusCode::NOT_FOUND, "post not found").into_response();
    };
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>{title} — Anti-Founder Captain Logs</title>
<meta name="description" content="{title}">
<link rel="canonical" href="https://captainslog.cochranblock.org/{slug}">
{STYLE}
</head><body>
<div class="wrap">
<nav class="crumbs"><a href="/">&larr; All Logs</a></nav>
<header class="site">
<div class="entry-meta">{date}</div>
<h1 class="entry-title" style="font-size:1.6rem">{title}</h1>
</header>
<article class="entry-body">{body}</article>
<footer class="site">
By <a href="https://cochranblock.org">Michael Cochran</a> &middot; <a href="https://github.com/cochranblock">github.com/cochranblock</a> &middot; Unlicense, public domain.
</footer>
</div>
</body></html>"#,
        title = p.title,
        date = p.date,
        slug = p.slug,
        body = p.body,
    ))
    .into_response()
}
