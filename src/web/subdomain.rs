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
                // Root of the subdomain → the section page.
                format!("/{}", sub)
            } else if path == format!("/{}", sub) || path.starts_with(&format!("/{}/", sub)) {
                // Already prefixed (e.g. form action="/deploy" on deploy.cochranblock.org,
                // or /deploy/confirmed after a server redirect).
                path.to_string()
            } else {
                // Any other path on this subdomain (nav links like /services, /about,
                // /stats, etc.) — route it as-is so the router can serve it directly.
                // We no longer prepend the current sub to avoid /products/services 404s.
                path.to_string()
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
    // Handles exact match (/products) and trailing slash (/products/).
    if host == "cochranblock.org" || host == "www.cochranblock.org" {
        let sub = path.trim_start_matches('/').split('/').next().unwrap_or("");
        if !sub.is_empty() && SUBDOMAINS.contains(&sub) {
            let trimmed = path.trim_end_matches('/');
            if trimmed == format!("/{}", sub) {
                return Redirect::permanent(&format!("https://{}.cochranblock.org", sub))
                    .into_response();
            }
        }
    }

    next.run(req).await
}
