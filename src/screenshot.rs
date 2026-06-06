#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! f53 = screenshot_capture. exopack for TRIPLE SIMS visual verification. Why: Kova forbids unwrap in lib.
// All Rights Reserved — The Cochran Block, LLC
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::fs;
use std::path::Path;

use crate::t0;
use crate::web::router;

const COCHRANBLOCK_PAGES: &[(&str, &str)] = &[
    ("index", "/"),
    ("products", "/products"),
    ("about", "/about"),
    ("contact", "/contact"),
    ("community-grant", "/community-grant"),
    ("book", "/book"),
    ("deploy", "/deploy"),
    ("deploy-confirmed", "/deploy/confirmed"),
    ("investigations", "/investigations"),
    ("investigations-viz", "/investigations/viz"),
    ("legacy", "/legacy"),
    ("resume", "/resume"),
    ("security", "/security"),
    ("arch", "/arch"),
    ("govdocs", "/govdocs"),
    ("stats", "/stats"),
    ("openbooks", "/openbooks"),
    ("no-quarter", "/no-quarter"),
];

/// Copy screenshots from cache to repo screenshots/ for TRIPLE SIMS docs.
fn copy_to_repo() {
    let cache = exopack::screenshot::f70("cochranblock");
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("screenshots");
    if let Err(e) = fs::create_dir_all(&repo) {
        eprintln!("screenshot: mkdir {}: {}", repo.display(), e);
        return;
    }
    for (name, _) in COCHRANBLOCK_PAGES {
        let src = cache.join(format!("{}.png", name));
        let dst = repo.join(format!("{}.png", name));
        if src.exists() {
            if let Err(e) = fs::copy(&src, &dst) {
                eprintln!(
                    "screenshot: copy {} -> {}: {}",
                    src.display(),
                    dst.display(),
                    e
                );
            }
        }
    }
    println!(
        "Screenshots copied to {}/ (for TRIPLE SIMS)",
        repo.display()
    );
}

/// f53 = screenshot_capture. Why: TRIPLE SIMS visual verification via exopack.
pub async fn f53() -> bool {
    let p0 = t0 { intake_db: None };
    let v1 = router::f1(p0);
    let v2 = match tokio::net::TcpListener::bind("127.0.0.1:0").await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("screenshot: bind failed: {}", e);
            return false;
        }
    };
    let v3 = match v2.local_addr() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("screenshot: local_addr failed: {}", e);
            return false;
        }
    };
    let v4 = format!("http://127.0.0.1:{}", v3.port());
    tokio::spawn(async move {
        let _ = axum::serve(v2, v1).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let theme = exopack::screenshot::f78();
    let ok = exopack::screenshot::f79(&v4, "cochranblock", COCHRANBLOCK_PAGES, &theme).await;
    if ok {
        copy_to_repo();
    }
    ok
}
