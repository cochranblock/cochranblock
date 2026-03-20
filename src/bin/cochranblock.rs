#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! CochranBlock server.
//! With `approuter` feature: registers with approuter, binds 0.0.0.0 (production).
//! Without: binds 127.0.0.1, opens browser (installable offline app).
// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use cochranblock::t0;
use cochranblock::web::{intake, router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = dotenvy::dotenv();
    if let Ok(root) = std::env::var("COCHRANBLOCK_ROOT") {
        let p = std::path::Path::new(&root).join(".env");
        let _ = dotenvy::from_path_override(&p);
    }
    let _ = dotenvy::from_path_override("cochranblock/.env");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")))
        .init();

    let port: u16 = std::env::var("PORT").ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);

    // Production (approuter): bind 0.0.0.0. Offline/local: bind 127.0.0.1.
    let is_local = !cfg!(feature = "approuter");
    let bind = std::env::var("BIND").unwrap_or_else(|_| {
        if is_local { "127.0.0.1".into() } else { "0.0.0.0".into() }
    });
    let addr = format!("{}:{}", bind, port);

    let intake_pool = intake::init_pool().await;
    let app = router::f1(t0 { intake_pool });

    #[cfg(feature = "approuter")]
    approuter_client::f116(approuter_client::RegisterConfig {
        app_id: "cochranblock",
        hostnames: std::env::var("CB_HOSTNAMES")
            .unwrap_or_else(|_| "cochranblock.org,www.cochranblock.org".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        backend_url: std::env::var("CB_BACKEND_URL")
            .unwrap_or_else(|_| format!("http://127.0.0.1:{}", port)),
    })
    .await;

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let url = format!("http://127.0.0.1:{}", port);
    tracing::info!("cochranblock listening on {}", url);

    // Offline mode: open browser automatically
    if is_local {
        println!("\n  cochranblock v{}", env!("CARGO_PKG_VERSION"));
        println!("  Running at {}", url);
        println!("  Press Ctrl+C to stop\n");
        let _ = open::that(&url);
    }

    let shutdown = async {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down");
    };
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
}
