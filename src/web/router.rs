#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use axum::{routing::get, Router};
use axum::http::header::{HeaderName, HeaderValue};
use axum::response::Redirect;
use tower_http::{
    compression::CompressionLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use crate::t0;
use super::{assets, community_grant, intake, pages};

/// f1 = app_router. Why: Single router with compression, trace, security headers; state shared via Arc.
pub fn f1(p0: t0) -> Router {
    let p0: std::sync::Arc<t0> = std::sync::Arc::new(p0);
    let h1 = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    let h2 = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("SAMEORIGIN"),
    );
    let h3 = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    let r0 = Router::new()
        .route("/", get(pages::f2))
        .route("/products", get(pages::f67))
        .route("/deploy", get(intake::get_form).post(intake::post_form))
        .route("/deploy/confirmed", get(intake::confirmed))
        .route("/about", get(pages::f12))
        .route("/contact", get(pages::f13))
        .route("/book", get(pages::f63))
        .route("/downloads", get(pages::f68))
        .route("/community-grant", get(community_grant::get_form).post(community_grant::post_form))
        .route("/community-grant/confirmed", get(community_grant::confirmed))
        // Redirects for old routes
        .route("/intake", get(|| async { Redirect::permanent("/deploy") }))
        .route("/services", get(pages::f11))
        .route("/mathskillz", get(pages::f72))
        .route("/provenance", get(pages::f74))
        .route("/sbir", get(pages::f74))
        .route("/codeskillz", get(pages::f76))
        .route("/govdocs", get(pages::f77))
        .route("/tinybinaries", get(pages::f81))
        .route("/vre", get(pages::f82))
        .route("/source", get(pages::f83))
        .route("/search", get(pages::f84))
        .route("/speed", get(pages::f85))
        .route("/federal-partners", get(|| async { Redirect::permanent("/products") }))
        .route("/health", get(pages::f10))
        .route("/api/stats", get(pages::f73))
        .route("/api/velocity", get(pages::f75))
        .route("/robots.txt", get(pages::f69))
        .route("/llms.txt", get(pages::f78))
        .route("/humans.txt", get(pages::f80))
        .route("/.well-known/security.txt", get(pages::f79))
        .route("/cochranblock-indexnow-key.txt", get(|| async { "cochranblock-indexnow-key" }))
        .route("/sitemap.xml", get(pages::f70))
        .route("/assets/*path", get(assets::f23));
    #[cfg(feature = "dev")]
    let r0 = r0
        .route("/dev/source", get(pages::f57))
        .route("/dev/exec-summary", get(pages::f58))
        .route("/dev/rules", get(pages::f59))
        .route("/dev/ai-orchestration", get(pages::f65))
        .route("/dev/prompts", get(pages::f66));
    r0
        .fallback(pages::f71)
        .layer(h1)
        .layer(h2)
        .layer(h3)
        .layer(CompressionLayer::new().zstd(true))
        .layer(TraceLayer::new_for_http())
        .with_state(p0)
}
