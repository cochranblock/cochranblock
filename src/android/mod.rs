// JNI entry point for Android WebView wrapper.
// Java calls startServer() → Rust spawns axum on a background thread.

use crate::t0;
use crate::web::{intake, router};

/// JNI entry: Java_org_cochranblock_cochranblock_MainActivity_startServer
#[unsafe(no_mangle)]
pub extern "C" fn Java_org_cochranblock_cochranblock_MainActivity_startServer(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(async {
            unsafe {
                std::env::set_var("PORT", "8081");
                std::env::set_var("BIND", "127.0.0.1");
            }

            let intake_pool = intake::init_pool().await;
            let app = router::f1(t0 { intake_pool });
            let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
                .await
                .expect("bind 8081");
            axum::serve(listener, app.into_make_service()).await.ok();
        });
    });
}
