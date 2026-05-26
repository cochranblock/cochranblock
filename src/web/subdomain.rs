// Unlicense — cochranblock.org
// Subdomain routing middleware.
// products.cochranblock.org/ → rewrites URI to /products (served by existing handler)
// cochranblock.org/products  → 301 to https://products.cochranblock.org

use axum::{
    http::{Request, Uri, uri::PathAndQuery},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

const SUBDOMAINS: &[&str] = &[
    "products", "services", "about", "deploy", "book", "security",
    "arch", "stats", "openbooks", "govdocs", "codeskillz",
    "tinybinaries", "search", "handbook", "changelog", "downloads", "source",
    "analytics", "privacy",
];

pub async fn layer(mut req: Request<axum::body::Body>, next: Next) -> Response {
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();
    let host = host.split(':').next().unwrap_or("").trim();
    let path = req.uri().path().to_string();

    // Inbound via subdomain: products.cochranblock.org/anything → rewrite to /products[/anything]
    if let Some(sub) = host.strip_suffix(".cochranblock.org") {
        if SUBDOMAINS.contains(&sub) {
            let canonical = if path == "/" || path.is_empty() {
                format!("/{}", sub)
            } else if path == format!("/{}", sub) || path.starts_with(&format!("/{}/", sub)) {
                // Already prefixed — form action="/deploy" POSTed from deploy.cochranblock.org,
                // or confirmation sub-path after a form redirect (e.g. /deploy/confirmed).
                path.to_string()
            } else {
                // Sub-path without subdomain prefix — prepend it.
                // e.g. /confirmed on deploy.cochranblock.org → /deploy/confirmed
                format!("/{}{}", sub, path)
            };
            let pq_str = req.uri().query()
                .map(|q| format!("{}?{}", canonical, q))
                .unwrap_or(canonical);
            if let Ok(pq) = pq_str.parse::<PathAndQuery>() {
                let mut parts = req.uri().clone().into_parts();
                parts.path_and_query = Some(pq);
                if let Ok(new_uri) = Uri::from_parts(parts) {
                    *req.uri_mut() = new_uri;
                }
            }
        }
        return next.run(req).await;
    }

    // Direct path on main domain: cochranblock.org/products → 301 products.cochranblock.org
    if host == "cochranblock.org" || host == "www.cochranblock.org" {
        let sub = path.trim_start_matches('/').split('/').next().unwrap_or("");
        if !sub.is_empty() && SUBDOMAINS.contains(&sub) && path == format!("/{}", sub) {
            return Redirect::permanent(&format!("https://{}.cochranblock.org", sub))
                .into_response();
        }
    }

    next.run(req).await
}
