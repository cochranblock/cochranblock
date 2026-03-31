#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! CochranBlock server with hot reload via PID lockfile.
//! Deploy: copy new binary, run it. Old one dies gracefully.
// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use cochranblock::t0;
use cochranblock::web::{intake, router};
use std::path::PathBuf;

fn pid_path() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let dir = base.join("cochranblock");
    let _ = std::fs::create_dir_all(&dir);
    dir.join("pid")
}

fn read_old_pid() -> Option<u32> {
    std::fs::read_to_string(pid_path())
        .ok()
        .and_then(|s| s.trim().parse().ok())
}

fn write_pid() {
    let _ = std::fs::write(pid_path(), std::process::id().to_string());
}

fn kill_old(pid: u32) {
    #[cfg(unix)]
    {
        use std::process::Command;
        use std::time::{Duration, Instant};

        // SIGTERM — graceful
        let _ = Command::new("kill").arg("-TERM").arg(pid.to_string()).output();
        tracing::info!("sent SIGTERM to old PID {}", pid);

        // Wait up to 5 seconds
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            let alive = Command::new("kill").arg("-0").arg(pid.to_string())
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if !alive {
                tracing::info!("old PID {} exited cleanly", pid);
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        // SIGKILL — force
        let _ = Command::new("kill").arg("-KILL").arg(pid.to_string()).output();
        tracing::warn!("sent SIGKILL to old PID {} (didn't exit in 5s)", pid);
        std::thread::sleep(Duration::from_millis(200));
    }
}

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

    let is_local = !cfg!(feature = "approuter");
    let bind = std::env::var("BIND").unwrap_or_else(|_| {
        if is_local { "127.0.0.1".into() } else { "0.0.0.0".into() }
    });
    let addr = format!("{}:{}", bind, port);

    // Kill old instance before binding port
    if let Some(pid) = read_old_pid().filter(|&p| p != std::process::id()) {
        kill_old(pid);
    }

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("cochranblock listening on http://127.0.0.1:{}", port);

    // Write our PID
    write_pid();

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

    if is_local {
        let url = format!("http://127.0.0.1:{}", port);
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
