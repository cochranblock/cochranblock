#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use axum::http::header::{HeaderName, HeaderValue};
use axum::response::Redirect;
use axum::{Router, routing::get};
use tower_http::{
    compression::CompressionLayer, set_header::SetResponseHeaderLayer, trace::TraceLayer,
};

use super::{assets, community_grant, intake, pages};
use crate::t0;

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
        .route("/", get(pages::f2_root))
        .route("/products", get(pages::f67))
        .route("/deploy", get(intake::get_form).post(intake::post_form))
        .route("/deploy/confirmed", get(intake::confirmed))
        .route("/about", get(pages::f12))
        .route("/contact", get(pages::f13))
        .route("/book", get(pages::f63))
        .route("/downloads", get(pages::f68))
        .route(
            "/community-grant",
            get(community_grant::get_form).post(community_grant::post_form),
        )
        .route(
            "/community-grant/confirmed",
            get(community_grant::confirmed),
        )
        // Redirects for old routes
        .route("/intake", get(|| async { Redirect::permanent("/deploy") }))
        .route("/services", get(pages::f11))
        .route("/mathskillz", get(|| async { Redirect::permanent("/stats") }))
        .route("/provenance", get(pages::f74))
        .route("/sbir", get(pages::f74))
        .route("/codeskillz", get(pages::f76))
        .route("/govdocs", get(pages::f77))
        .route("/tinybinaries", get(pages::f81))
        .route("/vre", get(pages::f82))
        .route("/source", get(pages::f83))
        .route("/search", get(pages::f84))
        .route("/stats", get(pages::f97))
        .route("/sovereignty", get(pages::f103))
        .route("/sovereign", get(|| async { Redirect::permanent("/sovereignty") }))
        .route("/proof", get(|| async { Redirect::permanent("/sovereignty") }))
        .route("/knox", get(pages::f104))
        .route("/knoxai", get(pages::f104))
        .route("/john", get(pages::f105))
        .route("/onboarding", get(pages::f106))
        .route("/handbook", get(pages::f106))
        .route("/speed", get(|| async { Redirect::permanent("/stats") }))
        .route("/openbooks", get(pages::f86))
        .route(
            "/govdocs/faq",
            get(|| async { axum::response::Redirect::permanent("/govdocs") }),
        )
        .route("/api/openbooks", get(pages::f87))
        .route("/dcaa", get(pages::f86))
        .route("/api/dcaa", get(pages::f87))
        .route("/arch", get(pages::f96))
        .route("/inventions", get(|| async { Redirect::permanent("/arch") }))
        .route("/protocols", get(|| async { Redirect::permanent("/arch#p26") }))
        .route("/security", get(pages::f98))
        .route("/pulse", get(pages::f100))
        .route("/operations", get(pages::f101))
        .route(
            "/operating-agreement",
            get(|| async { Redirect::permanent("/operations") }),
        )
        .route(
            "/manifesto",
            get(|| async { Redirect::permanent("/operations") }),
        )
        .route(
            "/operating-agreement.pdf",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "application/pdf",
                    )],
                    include_packed::include_packed!("assets/operating-agreement.pdf"),
                )
            }),
        )
        .route(
            "/operating-agreement.md",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/markdown; charset=utf-8",
                    )],
                    include_packed::include_packed!("assets/operating-agreement.md"),
                )
            }),
        )
        .route("/recommendation-melissa", get(pages::f102))
        .route(
            "/recommendation-melissa.pdf",
            get(|| async {
                (
                    [(axum::http::header::CONTENT_TYPE, "application/pdf")],
                    include_packed::include_packed!("assets/recommendation-melissa.pdf"),
                )
            }),
        )
        .route(
            "/recommendation-melissa.md",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/markdown; charset=utf-8",
                    )],
                    include_packed::include_packed!("assets/recommendation-melissa.md"),
                )
            }),
        )
        .route(
            "/live",
            get(|| async { Redirect::permanent("/pulse") }),
        )
        // ── Buzzword redirects → /security. The industry term and the name
        // we should have picked. The meta-joke requires both URLs to land.
        .route(
            "/secure-by-design",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/secure-by-default",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/zero-trust",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/fort-knox-lockdown-mega-infrastructure",
            get(|| async { Redirect::permanent("/security") }),
        )
        // ── Federal + compliance buzzword nets — SEO and discoverability
        .route("/cmmc", get(|| async { Redirect::permanent("/security") }))
        .route("/fedramp", get(|| async { Redirect::permanent("/security") }))
        .route(
            "/nist-800-171",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/nist-800-218",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/nist-800-53",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route("/fisma", get(|| async { Redirect::permanent("/security") }))
        .route("/soc2", get(|| async { Redirect::permanent("/security") }))
        // ── Iron Man monologue easter egg — read top to bottom, forms a
        //    hidden paragraph about kova, JARVIS, Mouse copyright threats,
        //    hero complex, and rust binaries gatling firewall gun protocol.
        //    Each URL is its own row in CF Analytics but the paragraph
        //    only reveals itself to someone who reads the whole list.
        .route(
            "/socyou3000",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/kova-was-supposed-to-jarvis-but-mouse-will-sue-me",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/ironman-is-my-favorite-superhero",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/the-way-iron-man-1-ended-was-fucking-epic",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/but-think-about-the-way-iron-man-1-started",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/the-whole-structure-of-the-movie-plot",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/was-to-hook-us-and-then-leave-us-happy",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/i-know-i-have-massive-hero-complex-and-wield-it-",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/like-surprise-mofo-when-someone-try-me",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/rust-binaries-gatling-firewall-gun-protocol-initiate",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route("/ato", get(|| async { Redirect::permanent("/security") }))
        .route(
            "/eo-14028",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route("/rmf", get(|| async { Redirect::permanent("/security") }))
        .route("/sbom", get(|| async { Redirect::permanent("/security") }))
        .route("/ssdf", get(|| async { Redirect::permanent("/security") }))
        .route("/cui", get(|| async { Redirect::permanent("/security") }))
        // ── Industry buzzwords
        .route(
            "/devsecops",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/shift-left",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/memory-safe",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/memory-safety",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/threat-intel",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route("/easm", get(|| async { Redirect::permanent("/security") }))
        .route(
            "/attack-surface",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/supply-chain-security",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route("/ztna", get(|| async { Redirect::permanent("/security") }))
        // ── The meta-joke alternatives. All land on /security. The humor is the
        //    collision: the absurd name and the compliance-board name do the same job.
        .route(
            "/the-vault",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/bunker-mode",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/hackerproof",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/you-shall-not-pass",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/lock-stock-and-two-smoking-binaries",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/airgap-lyf",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/not-today-satan",
            get(|| async { Redirect::permanent("/security") }),
        )
        .route(
            "/dead-mans-switch",
            get(|| async { Redirect::permanent("/security") }),
        )
        // ── Playful rate-card redirects. Each one is register-specific so the
        // recipient of a single link gets exactly the message you meant.
        // Every URL is its own row in CF Analytics → one click = one bit.
        .route(
            "/myratesareherebud",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route(
            "/nohackathons",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route("/402", get(|| async { Redirect::permanent("/services") }))
        .route(
            "/coffeechat",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route(
            "/paythetoll",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route(
            "/exposureisntcurrency",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route(
            "/backpocket",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route("/panels", get(|| async { Redirect::permanent("/services") }))
        .route(
            "/equityisnotasalary",
            get(|| async { Redirect::permanent("/services") }),
        )
        .route(
            "/MOONSHOT_FRAME.md",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/markdown; charset=utf-8",
                    )],
                    include_packed::include_packed!("assets/MOONSHOT_FRAME.md"),
                )
            }),
        )
        .route(
            "/supplement-msu-2026-04.html",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/html; charset=utf-8",
                    )],
                    include_packed::include_packed!("assets/supplement-msu-2026-04.html"),
                )
            }),
        )
        .route(
            "/diamond",
            get(|| async { Redirect::permanent("/arch#p27") }),
        )
        .route(
            "/diamond-architecture",
            get(|| async { Redirect::permanent("/arch#p27") }),
        )
        .route(
            "/diamond-rust-binary-architecture",
            get(|| async { Redirect::permanent("/arch#p27") }),
        )
        .route(
            "/p27",
            get(|| async { Redirect::permanent("/arch#p27") }),
        )
        .route(
            "/diamond-profile.toml",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/plain; charset=utf-8",
                    )],
                    include_packed::include_packed!("assets/diamond-profile.toml"),
                )
            }),
        )
        .route(
            "/moonshot-frame",
            get(|| async { Redirect::permanent("/MOONSHOT_FRAME.md") }),
        )
        .route("/privacy", get(pages::f93))
        .route("/changelog", get(pages::f94))
        .route("/barz", get(|| async { Redirect::permanent("/stats") }))
        .route("/analytics", get(pages::f90))
        .route("/api/analytics", get(pages::f91))
        .route("/api/site-stats", get(pages::f92))
        .route(
            "/federal-partners",
            get(|| async { Redirect::permanent("/products") }),
        )
        .route("/health", get(pages::f10))
        .route("/api/stats", get(pages::f73))
        .route("/api/velocity", get(pages::f75))
        .route(
            "/sw.js",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "application/javascript; charset=utf-8",
                    )],
                    include_packed::include_packed!("assets/js/sw.js"),
                )
            }),
        )
        .route("/robots.txt", get(pages::f69))
        .route("/llms.txt", get(pages::f78))
        .route("/llms-full.txt", get(pages::f88))
        .route("/api/summary", get(pages::f89))
        .route("/humans.txt", get(pages::f80))
        .route("/.well-known/security.txt", get(pages::f79))
        .route(
            "/cochranblock-indexnow-key.txt",
            get(|| async { "cochranblock-indexnow-key" }),
        )
        .route("/sitemap.xml", get(pages::f70))
        .route("/assets/*path", get(assets::f23));
    #[cfg(feature = "dev")]
    let r0 = r0
        .route("/dev/source", get(pages::f57))
        .route("/dev/exec-summary", get(pages::f58))
        .route("/dev/rules", get(pages::f59))
        .route("/dev/ai-orchestration", get(pages::f65))
        .route("/dev/prompts", get(pages::f66));
    r0.fallback(pages::f71)
        .layer(h1)
        .layer(h2)
        .layer(h3)
        .layer(CompressionLayer::new().zstd(true))
        .layer(TraceLayer::new_for_http())
        .with_state(p0)
}
