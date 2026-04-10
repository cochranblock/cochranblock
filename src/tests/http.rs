#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::time::Instant;
use tempfile::TempDir;

use super::assert_ok;
use super::t24;

use crate::t0;
use crate::web::router;

async fn run(p0: &str, p1: impl std::future::Future<Output = Result<(), String>>) -> t24 {
    let v0 = Instant::now();
    match p1.await {
        Ok(()) => t24 {
            s30: p0.into(),
            s31: true,
            s32: v0.elapsed().as_millis() as u64,
            s33: None,
        },
        Err(msg) => t24 {
            s30: p0.into(),
            s31: false,
            s32: v0.elapsed().as_millis() as u64,
            s33: Some(msg),
        },
    }
}

async fn server() -> (String, reqwest::Client, TempDir) {
    let v0 = TempDir::new().unwrap();
    let p0 = t0 { intake_pool: None };
    let v1 = router::f1(p0);
    let v2 = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let v3 = v2.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(v2, v1.into_make_service()).await.unwrap();
    });
    let v4 = format!("http://{}", v3);
    let v5 = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    (v4, v5, v0)
}

pub async fn f51() -> Vec<t24> {
    let mut v0 = Vec::new();
    let (v1, v2, _) = server().await;

    v0.push(
        run("index_200", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {} not 2xx", status))?;
            assert_ok(
                v4.contains("CochranBlock"),
                "home page missing 'CochranBlock'",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("services_200", async {
            let v3 = v2
                .get(format!("{}/services", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("/services status {}", status))?;
            assert_ok(
                v4.contains("Pricing") && v4.contains("3,500"),
                "/services must have pricing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("about_200", async {
            let v3 = v2
                .get(format!("{}/about", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(
                v4.contains("About CochranBlock") || v4.contains("About"),
                "about page missing about content",
            )?;
            assert_ok(
                v4.contains("Mission") || v4.contains("Credentials"),
                "about page missing Mission or Credentials",
            )?;
            assert_ok(
                v4.contains("Credentials"),
                "about page missing 'Credentials'",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("contact_200", async {
            let v3 = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(v4.contains("Contact"), "contact page missing 'Contact'")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("intake_redirect", async {
            let v3 = v2
                .get(format!("{}/intake", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            assert_ok(
                status.as_u16() == 308 || status.is_success(),
                format!("/intake must redirect or 200, got {}", status),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("community_grant_200", async {
            let v3 = v2
                .get(format!("{}/community-grant", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(
                v4.contains("Community Grant") || v4.contains("$500"),
                "community grant page missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("deploy_confirmed_legacy", async {
            let v3 = v2
                .get(format!("{}/deploy/confirmed", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            assert_ok(
                status.is_success(),
                format!("deploy/confirmed must 200, got {}", status),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("community_grant_confirmed_200", async {
            let v3 = v2
                .get(format!("{}/community-grant/confirmed", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(
                v4.contains("Application Received") || v4.contains("submitted"),
                "community grant confirmed missing success message",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("intake_redirects_to_deploy", async {
            let v3 = v2
                .get(format!("{}/intake", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().as_u16() == 308,
                format!("/intake must 308 → /deploy, got {}", v3.status()),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("community_grant_form_fields", async {
            let v3 = v2
                .get(format!("{}/community-grant", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("name=\"org_name\""),
                "community grant missing org_name",
            )?;
            assert_ok(
                v4.contains("name=\"mission\""),
                "community grant missing mission",
            )?;
            assert_ok(
                v4.contains("name=\"technical_objective\""),
                "community grant missing technical_objective",
            )?;
            assert_ok(
                v4.contains("consent_grant") && v4.contains("$500"),
                "community grant missing $500 consent",
            )?;
            assert_ok(
                v4.contains("action=\"/community-grant\""),
                "community grant form missing action",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("products_200", async {
            let v3 = v2
                .get(format!("{}/products", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(v4.contains("Products"), "products page missing 'Products'")?;
            assert_ok(
                v4.contains("Coming Soon"),
                "products page missing 'Coming Soon'",
            )?;
            assert_ok(
                v4.contains("Rogue Repo") || v4.contains("CochranBlock"),
                "products missing Rogue Repo",
            )?;
            assert_ok(v4.contains("Ronin Sites"), "products missing Ronin Sites")?;
            assert_ok(v4.contains("Kova"), "products missing Kova")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("book_200", async {
            let v3 = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(
                v4.contains("Schedule a Discovery Call"),
                "book page missing heading",
            )?;
            assert_ok(
                v4.contains("Braintrust"),
                "book page missing Braintrust context",
            )?;
            assert_ok(v4.contains("30 minutes"), "book page missing duration")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("book_calendar_structure", async {
            let v3 = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("booking-grid"), "calendar missing booking grid")?;
            assert_ok(v4.contains("booking-month"), "calendar missing month label")?;
            assert_ok(v4.contains("booking-prev"), "calendar missing prev button")?;
            assert_ok(v4.contains("booking-next"), "calendar missing next button")?;
            assert_ok(
                v4.contains("booking-slots-data"),
                "calendar missing slots JSON",
            )?;
            assert_ok(v4.contains("booking.js"), "calendar missing booking.js")?;
            assert_ok(v4.contains("booking-hint"), "calendar missing hint")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("book_slots_mailto", async {
            let v3 = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("mailto:"), "book page missing mailto links")?;
            assert_ok(
                v4.contains("Discovery%20Call"),
                "mailto missing Discovery Call subject",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("robots_txt", async {
            let v3 = v2
                .get(format!("{}/robots.txt", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("robots.txt status {}", status))?;
            assert_ok(
                v4.contains("User-agent: *"),
                "robots.txt missing User-agent",
            )?;
            assert_ok(v4.contains("Sitemap:"), "robots.txt missing Sitemap")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("sitemap_xml", async {
            let v3 = v2
                .get(format!("{}/sitemap.xml", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                status.is_success(),
                format!("sitemap.xml status {}", status),
            )?;
            assert_ok(v4.contains("<urlset"), "sitemap missing urlset")?;
            assert_ok(v4.contains("cochranblock.org/"), "sitemap missing URLs")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("json_ld_org", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("application/ld+json"),
                "home missing JSON-LD script",
            )?;
            assert_ok(
                v4.contains("schema.org") && v4.contains("Organization"),
                "home missing Organization schema",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("health_ok", async {
            let v3 = v2
                .get(format!("{}/health", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(v4 == "OK", format!("health returned '{}' not 'OK'", v4))?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("static_css", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(ct.contains("css"), format!("content-type {} not css", ct))?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("css_mobile_hero_cta", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains(".hero-cta") && v4.contains("flex-direction: column"),
                "hero-cta must stack vertically on mobile",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("css_mobile_tagline", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains(".tagline") && v4.contains("1.1rem"),
                "tagline must have mobile font-size reduction",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("css_mobile_pricing_cards", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains(".pricing-cards") && v4.contains("grid-template-columns: 1fr"),
                "pricing-cards must go single-column on mobile",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("css_mobile_cost_table", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains(".cost-bar-cell") && v4.contains("display: none"),
                "analytics volume bar must hide on small phones",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("css_nav_dropdown_clamp", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains(".nav-group-links") && v4.contains("max-width: calc(100vw"),
                "nav dropdown must clamp to viewport",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("css_resume_overflow", async {
            let v3 = v2
                .get(format!("{}/assets/css/main.css", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains(".resume-raw") && v4.contains("overflow-wrap: anywhere"),
                "resume-raw must break long unicode lines",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("calendar_js", async {
            let v3 = v2
                .get(format!("{}/assets/js/calendar.js", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(v4.contains("parseSlots"), "calendar.js missing parseSlots")?;
            assert_ok(
                v4.contains("renderMonth"),
                "calendar.js missing renderMonth",
            )?;
            assert_ok(v4.contains("renderWeek"), "calendar.js missing renderWeek")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("favicon_svg", async {
            let v3 = v2
                .get(format!("{}/assets/favicon.svg", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(v4.contains("<svg"), "favicon not valid SVG")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("resume_pdf", async {
            let v3 = v2
                .get(format!("{}/assets/resume.pdf", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(ct.contains("pdf"), format!("content-type {} not pdf", ct))?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("index_business", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("status {}", status))?;
            assert_ok(
                v4.contains("Rust") && v4.contains("SaaS"),
                "home missing Rust SaaS messaging",
            )?;
            assert_ok(
                v4.contains("Get in Touch") || v4.contains("Book a Call"),
                "home missing CTA",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("404", async {
            let v3 = v2
                .get(format!("{}/nonexistent", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            assert_ok(
                status.as_u16() == 404,
                format!("expected 404 got {}", status),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("federal_partners_redirect", async {
            let v3 = v2
                .get(format!("{}/federal-partners", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().as_u16() == 308,
                format!(
                    "/federal-partners must 308 → /products, got {}",
                    v3.status()
                ),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("contact_links", async {
            let v3 = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("mailto:mcochran@cochranblock.org"),
                "contact missing email link",
            )?;
            assert_ok(v4.contains("Book a Call"), "contact missing Book a Call")?;
            assert_ok(
                v4.contains("CochranBlock%20Inquiry") || v4.contains("mailto:"),
                "contact mailto missing",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("about_tabs", async {
            let v3 = v2
                .get(format!("{}/about", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Mission") || v4.contains("Profile"),
                "about missing Mission tab",
            )?;
            assert_ok(v4.contains("Credentials"), "about missing Credentials tab")?;
            assert_ok(
                v4.contains("copy-resume") || v4.contains("resume-raw"),
                "about missing resume block",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("home_ctas", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"/contact\""),
                "home missing contact link",
            )?;
            assert_ok(v4.contains("href=\"/book\""), "home missing book link")?;
            assert_ok(v4.contains("href=\"/about\""), "home missing about link")?;
            assert_ok(
                v4.contains("href=\"/products\""),
                "home missing products link",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("book_slots_json", async {
            let v3 = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let v5 = v4
                .find("id=\"booking-slots-data\"")
                .ok_or("no slots element")?;
            let v6 = v4[v5..].find('>').ok_or("no content start")?;
            let v7_start = v5 + v6 + 1;
            let v7_end = v4[v7_start..].find("</script>").ok_or("no script end")? + v7_start;
            let v8 = v4[v7_start..v7_end].trim();
            let v9: Vec<serde_json::Value> = serde_json::from_str(v8).map_err(|e| e.to_string())?;
            assert_ok(!v9.is_empty(), "slots must not be empty")?;
            let v10 = &v9[0];
            let v11 = v10
                .get("date")
                .and_then(|x| x.as_str())
                .ok_or("slot missing date")?;
            assert_ok(v11.len() >= 10, "date must be YYYY-MM-DD")?;
            let v12 = v10
                .get("times")
                .and_then(|x| x.as_array())
                .ok_or("slot missing times")?;
            assert_ok(!v12.is_empty(), "slot must have times")?;
            let v13 = v12[0]
                .get("mailto")
                .and_then(|x| x.as_str())
                .ok_or("time missing mailto")?;
            assert_ok(v13.starts_with("mailto:"), "mailto must be valid")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("booking_js", async {
            let v3 = v2
                .get(format!("{}/assets/js/booking.js", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("parseSlots"), "booking.js missing parseSlots")?;
            assert_ok(v4.contains("renderMonth"), "booking.js missing renderMonth")?;
            assert_ok(
                v4.contains("booking-slots-data"),
                "booking.js missing slots data id",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("logo_svg", async {
            let v3 = v2
                .get(format!("{}/assets/cochranblock-logo.svg", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(v3.status().is_success(), "logo must 200")?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("<svg"), "logo must be valid SVG")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("assets_404", async {
            let v3 = v2
                .get(format!("{}/assets/nonexistent.file", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().as_u16() == 404,
                format!("assets 404 expected, got {}", v3.status()),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("removed_routes_404", async {
            for path in [
                "/ai-orchestration",
                "/prompts",
                "/cursor-rules",
                "/executive-summary",
            ] {
                let v3 = v2
                    .get(format!("{}{}", v1, path))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                assert_ok(
                    v3.status().as_u16() == 404,
                    format!("{} must 404, got {}", path, v3.status()),
                )?;
            }
            Ok(())
        })
        .await,
    );
    #[cfg(feature = "dev")]
    v0.push(
        run("dev_routes_200", async {
            for path in [
                "/dev/source",
                "/dev/exec-summary",
                "/dev/rules",
                "/dev/ai-orchestration",
                "/dev/prompts",
            ] {
                let v3 = v2
                    .get(format!("{}{}", v1, path))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                assert_ok(
                    v3.status().is_success(),
                    format!("{} must 200, got {}", path, v3.status()),
                )?;
            }
            Ok(())
        })
        .await,
    );
    v0.push(
        run("nav_footer_links", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"/products\""),
                "nav/footer missing /products",
            )?;
            assert_ok(v4.contains("href=\"/about\""), "nav/footer missing /about")?;
            assert_ok(
                v4.contains("href=\"/contact\""),
                "nav/footer missing /contact",
            )?;
            assert_ok(
                v4.contains("href=\"/deploy\"") || v4.contains("href=\"/intake\""),
                "nav/footer missing /deploy",
            )?;
            assert_ok(v4.contains("href=\"/book\""), "nav/footer missing /book")?;
            assert_ok(
                v4.contains("href=\"/\"") || v4.contains("href=\"https://cochranblock.org\""),
                "nav/footer missing home link",
            )?;
            Ok(())
        })
        .await,
    );
    // federal-partners collapsed into /products — redirect tested above
    v0.push(
        run("gzip_encoding", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .header("Accept-Encoding", "gzip")
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let enc = v3
                .headers()
                .get("Content-Encoding")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(
                enc.contains("gzip") || enc.is_empty(),
                format!("gzip or identity ok, got encoding: {}", enc),
            )?;
            assert_ok(v3.status().is_success(), "must succeed")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("hero_product_status", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Product in development") || v4.contains("hero-status"),
                "hero must show product status",
            )?;
            assert_ok(
                v4.contains("Consulting") && (v4.contains("open") || v4.contains("capacity")),
                "hero must show consulting availability",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("products_all_coming_soon", async {
            let v3 = v2
                .get(format!("{}/products", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("Rogue Repo"), "products must list Rogue Repo")?;
            assert_ok(v4.contains("Ronin Sites"), "products must list Ronin Sites")?;
            assert_ok(v4.contains("Kova"), "products must list Kova")?;
            assert_ok(v4.contains("Coming Soon"), "products must show Coming Soon")?;
            assert_ok(
                v4.contains("Get Notified") || v4.contains("href=\"/contact\""),
                "products must have contact CTA",
            )?;
            assert_ok(
                v4.contains("/assets/img/") && v4.contains(".png"),
                "products must have product images",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("contact_product_launch_cta", async {
            let v3 = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Product Launch") || v4.contains("Product%20Launch"),
                "contact must have product launch CTA",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("about_founded_by", async {
            let v3 = v2
                .get(format!("{}/about", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Michael Cochran") || v4.contains("Founded"),
                "about must have founder",
            )?;
            Ok(())
        })
        .await,
    );
    // services collapsed into /products — redirect tested above
    v0.push(
        run("semantic_main_nav", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("<main") || v4.contains("id=\"main\""),
                "page must have main landmark",
            )?;
            assert_ok(
                v4.contains("<nav") || v4.contains("class=\"nav\""),
                "page must have nav",
            )?;
            assert_ok(
                v4.contains("skip-link") || v4.contains("Skip to main"),
                "page must have skip link for a11y",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("html_doctype", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.trim_start().starts_with("<!DOCTYPE html>"),
                "page must have DOCTYPE",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("meta_viewport", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("viewport") && v4.contains("width=device-width"),
                "page must have viewport meta for mobile",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("product_images_200", async {
            for path in [
                "/assets/img/rogue-repo.png",
                "/assets/img/ronin-sites.png",
                "/assets/img/kova.png",
                "/assets/img/pixel-forge.png",
            ] {
                let v3 = v2
                    .get(format!("{}{}", v1, path))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                assert_ok(v3.status().is_success(), format!("{} must 200", path))?;
                let ct = v3
                    .headers()
                    .get("content-type")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("");
                assert_ok(ct.contains("png"), format!("{} must be png", path))?;
            }
            Ok(())
        })
        .await,
    );
    // products_services_link: /services collapsed into /products
    v0.push(
        run("products_roguerepo_link", async {
            let v3 = v2
                .get(format!("{}/products", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"https://roguerepo.io\""),
                "products must link Rogue Repo to roguerepo.io",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("products_ronin_link", async {
            let v3 = v2
                .get(format!("{}/products", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"https://ronin-sites.pro\""),
                "products must link Ronin Sites to ronin-sites.pro",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("book_weekdays_only", async {
            let v3 = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let v5 = v4.find("id=\"booking-slots-data\"").ok_or("no slots")?;
            let v6 = v4[v5..].find('>').ok_or("no content")?;
            let v7_start = v5 + v6 + 1;
            let v7_end = v4[v7_start..].find("</script>").ok_or("no end")? + v7_start;
            let v8: Vec<serde_json::Value> =
                serde_json::from_str(v4[v7_start..v7_end].trim()).map_err(|e| e.to_string())?;
            assert_ok(!v8.is_empty(), "slots must not be empty")?;
            let v9 = v8[0].get("day_name").and_then(|x| x.as_str()).unwrap_or("");
            assert_ok(
                !v9.is_empty()
                    && (v9.starts_with("Mon")
                        || v9.starts_with("Tue")
                        || v9.starts_with("Wed")
                        || v9.starts_with("Thu")
                        || v9.starts_with("Fri")),
                "slots must be weekdays only",
            )?;
            Ok(())
        })
        .await,
    );
    // services_product_details + services_consulting_details: collapsed into /products
    v0.push(
        run("index_tagline", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Sovereign Intelligence") || v4.contains("Public Domain"),
                "home must have mission tagline",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("contact_testimonial", async {
            let v3 = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("brightest") || v4.contains("USCYBERCOM") || v4.contains("testimonial"),
                "contact must have trust signal",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("buttons_nav_all_200", async {
            for (path, needle) in [
                ("/", "CochranBlock"),
                ("/products", "Products"),
                ("/about", "About"),
                ("/contact", "Contact"),
                ("/deploy", "Deploy With"),
                ("/book", "Schedule"),
            ] {
                let v3 = v2
                    .get(format!("{}{}", v1, path))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                assert_ok(v3.status().is_success(), format!("{} must 200", path))?;
                let v4 = v3.text().await.map_err(|e| e.to_string())?;
                assert_ok(
                    v4.contains(needle),
                    format!("{} missing expected content", path),
                )?;
            }
            Ok(())
        })
        .await,
    );
    v0.push(
        run("buttons_hero_ctas_200", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"/products\""),
                "hero missing products link",
            )?;
            assert_ok(
                v4.contains("href=\"/book\""),
                "hero missing Book a Call link",
            )?;
            assert_ok(
                v4.contains("href=\"/contact\""),
                "hero missing Get in Touch link",
            )?;
            assert_ok(v4.contains("href=\"/about\""), "hero missing About link")?;
            let r_products = v2
                .get(format!("{}/products", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let r_book = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let r_contact = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let r_about = v2
                .get(format!("{}/about", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                r_products.status().is_success(),
                "hero /products link must 200",
            )?;
            assert_ok(r_book.status().is_success(), "hero /book link must 200")?;
            assert_ok(
                r_contact.status().is_success(),
                "hero /contact link must 200",
            )?;
            assert_ok(r_about.status().is_success(), "hero /about link must 200")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("buttons_footer_links_200", async {
            for path in ["/", "/products", "/about", "/contact", "/deploy", "/book"] {
                let v3 = v2
                    .get(format!("{}{}", v1, path))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                assert_ok(
                    v3.status().is_success(),
                    format!("footer {} must 200", path),
                )?;
            }
            Ok(())
        })
        .await,
    );
    v0.push(
        run("buttons_products_ctas_200", async {
            let v3 = v2
                .get(format!("{}/products", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"/contact\""),
                "products missing Get Notified link",
            )?;
            let r_contact = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                r_contact.status().is_success(),
                "products Get Notified must 200",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("buttons_contact_ctas_200", async {
            let v3 = v2
                .get(format!("{}/contact", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("mailto:") || v4.contains("href=\"/book\""),
                "contact must have Email or Book link",
            )?;
            assert_ok(
                v4.contains("href=\"/book\""),
                "contact must have Book a Call link",
            )?;
            assert_ok(
                v4.contains("/assets/resume.pdf"),
                "contact must have resume download",
            )?;
            let r_book = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let r_resume = v2
                .get(format!("{}/assets/resume.pdf", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(r_book.status().is_success(), "contact Book link must 200")?;
            assert_ok(
                r_resume.status().is_success(),
                "contact resume download must 200",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("buttons_book_nav_200", async {
            let v3 = v2
                .get(format!("{}/book", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("booking-prev"), "book must have prev button")?;
            assert_ok(v4.contains("booking-next"), "book must have next button")?;
            assert_ok(v4.contains("mailto:"), "book must have mailto slots")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("deploy_200", async {
            let v3 = v2
                .get(format!("{}/deploy", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = v3.status();
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(status.is_success(), format!("deploy status {}", status))?;
            assert_ok(v4.contains("intake-form"), "deploy missing intake-form")?;
            assert_ok(
                v4.contains("intake-section"),
                "deploy missing intake-section",
            )?;
            assert_ok(v4.contains("intake-doc"), "deploy missing intake-doc")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("deploy_form_fields", async {
            let v3 = v2
                .get(format!("{}/deploy", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("name=\"deploy_class\""),
                "deploy missing deploy_class field",
            )?;
            assert_ok(
                v4.contains("name=\"full_name\""),
                "deploy missing full_name field",
            )?;
            assert_ok(v4.contains("name=\"email\""), "deploy missing email field")?;
            assert_ok(
                v4.contains("name=\"consent_fee\""),
                "deploy missing consent_fee",
            )?;
            assert_ok(
                v4.contains("name=\"consent_hardware\""),
                "deploy missing consent_hardware",
            )?;
            assert_ok(
                v4.contains("name=\"consent_terms\""),
                "deploy missing consent_terms",
            )?;
            assert_ok(
                v4.contains("action=\"/deploy\""),
                "deploy form missing action",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("deploy_intake_content", async {
            let v3 = v2
                .get(format!("{}/deploy", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Product"),
                "deploy missing Product class option",
            )?;
            assert_ok(
                v4.contains("Consulting"),
                "deploy missing Consulting class option",
            )?;
            assert_ok(
                v4.contains("Partnership"),
                "deploy missing Partnership class option",
            )?;
            assert_ok(v4.contains("$3,500"), "deploy missing base price")?;
            assert_ok(
                v4.contains("Cloudflare"),
                "deploy missing Cloudflare reference",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("deploy_confirmed_200", async {
            let v3 = v2
                .get(format!("{}/deploy/confirmed", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("deploy/confirmed must 200, got {}", v3.status()),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("deploy_honeypot", async {
            let v3 = v2
                .get(format!("{}/deploy", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("website_url"), "deploy missing honeypot field")?;
            Ok(())
        })
        .await,
    );

    // === PAGES: full route coverage ===
    v0.push(
        run("downloads_200", async {
            let v3 = v2
                .get(format!("{}/downloads", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/downloads status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Download") || v4.contains("Binary") || v4.contains("ARM"),
                "/downloads missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("stats_200", async {
            let v3 = v2
                .get(format!("{}/stats", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/stats status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("cloud") || v4.contains("cost") || v4.contains("$"),
                "/stats missing cost content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("provenance_200", async {
            let v3 = v2
                .get(format!("{}/provenance", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/provenance status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Provenance") || v4.contains("SBIR") || v4.contains("documentation"),
                "/provenance missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("sbir_same_as_provenance", async {
            let v3 = v2
                .get(format!("{}/sbir", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/sbir status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Provenance") || v4.contains("SBIR") || v4.contains("documentation"),
                "/sbir missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("codeskillz_200", async {
            let v3 = v2
                .get(format!("{}/codeskillz", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/codeskillz status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("velocity") || v4.contains("repo") || v4.contains("Rust"),
                "/codeskillz missing content",
            )?;
            assert_ok(
                v4.contains("api/velocity"),
                "/codeskillz must reference velocity API",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("govdocs_200", async {
            let v3 = v2
                .get(format!("{}/govdocs", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/govdocs status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("NAICS") || v4.contains("capability") || v4.contains("government"),
                "/govdocs missing content",
            )?;
            assert_ok(v4.contains("NanoSign"), "/govdocs missing NanoSign")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("govdocs_faq_redirect", async {
            let v3 = v2
                .get(format!("{}/govdocs/faq", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().as_u16() == 308 || v3.status().is_success(),
                format!("/govdocs/faq must 308 or 200, got {}", v3.status()),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("tinybinaries_200", async {
            let v3 = v2
                .get(format!("{}/tinybinaries", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/tinybinaries status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("MB") || v4.contains("binary") || v4.contains("KB"),
                "/tinybinaries missing size content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("vre_200", async {
            let v3 = v2
                .get(format!("{}/vre", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/vre status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("VR&E") || v4.contains("VA") || v4.contains("veteran"),
                "/vre missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("source_200", async {
            let v3 = v2
                .get(format!("{}/source", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/source status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("source") || v4.contains("code") || v4.contains("Rust"),
                "/source missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("search_200", async {
            let v3 = v2
                .get(format!("{}/search", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/search status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("search") || v4.contains("Search") || v4.contains("query"),
                "/search missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("search_with_query", async {
            let v3 = v2
                .get(format!("{}/search?q=rust", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/search?q=rust status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(!v4.is_empty(), "/search?q=rust must return content")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("speed_redirect", async {
            let v3 = v2
                .get(format!("{}/speed", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/speed redirect status {}", v3.status()),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("openbooks_200", async {
            let v3 = v2
                .get(format!("{}/openbooks", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/openbooks status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("IR&D") || v4.contains("hours") || v4.contains("audit"),
                "/openbooks missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("dcaa_same_as_openbooks", async {
            let v3 = v2
                .get(format!("{}/dcaa", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/dcaa status {}", v3.status()),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("privacy_200", async {
            let v3 = v2
                .get(format!("{}/privacy", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/privacy status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Privacy") || v4.contains("data") || v4.contains("collection"),
                "/privacy missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("changelog_200", async {
            let v3 = v2
                .get(format!("{}/changelog", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/changelog status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("changelog") || v4.contains("Changelog") || v4.contains("commit"),
                "/changelog missing content",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("analytics_200", async {
            let v3 = v2
                .get(format!("{}/analytics", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/analytics status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains("Analytics"), "/analytics missing heading")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("analytics_no_token_fallback", async {
            // Test server has no CF_TOKEN — must show fallback, never a zero-filled table.
            let v3 = v2
                .get(format!("{}/analytics", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Live data requires Cloudflare integration"),
                "/analytics must show fallback when CF_TOKEN absent, not a zero table",
            )?;
            assert_ok(
                !v4.contains(">0<") && !v4.contains(">0.0 MB<"),
                "/analytics must not render zero-filled table rows when no data",
            )?;
            assert_ok(
                v4.contains("Request a Live Demo") || v4.contains("mcochran@cochranblock.org"),
                "/analytics fallback must have a contact CTA",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("analytics_fallback_has_json_link", async {
            let v3 = v2
                .get(format!("{}/analytics", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("/api/analytics"),
                "/analytics fallback must link to JSON endpoint",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("analytics_fallback_has_nav_ctas", async {
            let v3 = v2
                .get(format!("{}/analytics", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("href=\"/stats\""),
                "/analytics fallback missing /stats link",
            )?;
            assert_ok(
                v4.contains("href=\"/openbooks\""),
                "/analytics fallback missing /openbooks link",
            )?;
            Ok(())
        })
        .await,
    );

    // === API ENDPOINTS ===
    v0.push(
        run("api_openbooks_json", async {
            let v3 = v2
                .get(format!("{}/api/openbooks", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/openbooks status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("json"),
                format!("/api/openbooks content-type must be json, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let parsed: serde_json::Value = serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            assert_ok(
                parsed.get("repos").is_some(),
                "/api/openbooks missing repos field",
            )?;
            assert_ok(
                parsed.get("total_hours").is_some(),
                "/api/openbooks missing total_hours",
            )?;
            assert_ok(
                parsed.get("total_value").is_some(),
                "/api/openbooks missing total_value",
            )?;
            assert_ok(parsed.get("rate").is_some(), "/api/openbooks missing rate")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_dcaa_same_as_openbooks", async {
            let v3 = v2
                .get(format!("{}/api/dcaa", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/dcaa status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let parsed: serde_json::Value = serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            assert_ok(
                parsed.get("repos").is_some(),
                "/api/dcaa missing repos field",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_analytics_json", async {
            let v3 = v2
                .get(format!("{}/api/analytics", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/analytics status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("json"),
                format!("/api/analytics must be json, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let _parsed: serde_json::Value =
                serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_site_stats_json", async {
            let v3 = v2
                .get(format!("{}/api/site-stats", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/site-stats status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("json"),
                format!("/api/site-stats must be json, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let parsed: serde_json::Value = serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            assert_ok(
                parsed.get("repo_count").is_some(),
                "/api/site-stats missing repo_count",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_site_stats_repo_count_nonzero", async {
            let v3 = v2
                .get(format!("{}/api/site-stats", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let parsed: serde_json::Value = serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            let count = parsed["repo_count"].as_u64().unwrap_or(0);
            assert_ok(count > 0, format!("repo_count must be > 0, got {}", count))?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_stats_json", async {
            let v3 = v2
                .get(format!("{}/api/stats", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/stats status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("json"),
                format!("/api/stats must be json, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let parsed: serde_json::Value = serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            assert_ok(
                parsed.get("binary_size_arm").is_some(),
                "/api/stats missing binary_size_arm",
            )?;
            assert_ok(parsed.get("repos").is_some(), "/api/stats missing repos")?;
            assert_ok(
                parsed.get("timestamp").is_some(),
                "/api/stats missing timestamp",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_velocity_json", async {
            let v3 = v2
                .get(format!("{}/api/velocity", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/velocity status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("json"),
                format!("/api/velocity must be json, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let _parsed: serde_json::Value =
                serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_summary_json", async {
            let v3 = v2
                .get(format!("{}/api/summary", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/api/summary status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("json"),
                format!("/api/summary must be json, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            let parsed: serde_json::Value = serde_json::from_str(&v4).map_err(|e| e.to_string())?;
            assert_ok(
                parsed.get("company").is_some(),
                "/api/summary missing company",
            )?;
            assert_ok(
                parsed.get("innovations").is_some(),
                "/api/summary missing innovations",
            )?;
            assert_ok(parsed.get("naics").is_some(), "/api/summary missing naics")?;
            let innovations = parsed["innovations"]
                .as_array()
                .ok_or("innovations must be array")?;
            assert_ok(
                innovations.iter().any(|i| i.as_str() == Some("NanoSign")),
                "/api/summary innovations missing NanoSign",
            )?;
            Ok(())
        })
        .await,
    );

    // === WELL-KNOWN / TEXT FILES ===
    v0.push(
        run("llms_txt", async {
            let v3 = v2
                .get(format!("{}/llms.txt", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/llms.txt status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("text/plain"),
                format!("/llms.txt must be text/plain, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("CochranBlock"),
                "/llms.txt missing CochranBlock",
            )?;
            assert_ok(
                v4.contains("veteran") || v4.contains("Veteran"),
                "/llms.txt missing veteran context",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("llms_full_txt", async {
            let v3 = v2
                .get(format!("{}/llms-full.txt", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/llms-full.txt status {}", v3.status()),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("CochranBlock"),
                "/llms-full.txt missing CochranBlock",
            )?;
            assert_ok(v4.len() > v4.len() / 2, "llms-full.txt must have content")?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("humans_txt", async {
            let v3 = v2
                .get(format!("{}/humans.txt", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/humans.txt status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("text/plain"),
                format!("/humans.txt must be text/plain, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("TEAM") || v4.contains("Developer"),
                "/humans.txt missing TEAM section",
            )?;
            assert_ok(
                v4.contains("Michael Cochran"),
                "/humans.txt missing developer name",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("security_txt", async {
            let v3 = v2
                .get(format!("{}/.well-known/security.txt", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/.well-known/security.txt status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("text/plain"),
                format!("security.txt must be text/plain, got {}", ct),
            )?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(
                v4.contains("Contact:"),
                "security.txt missing Contact field",
            )?;
            assert_ok(
                v4.contains("Expires:"),
                "security.txt missing Expires field",
            )?;
            assert_ok(
                v4.contains("Canonical:"),
                "security.txt missing Canonical field",
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("sw_js", async {
            let v3 = v2
                .get(format!("{}/sw.js", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            assert_ok(
                v3.status().is_success(),
                format!("/sw.js status {}", v3.status()),
            )?;
            let ct = v3
                .headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            assert_ok(
                ct.contains("javascript"),
                format!("/sw.js must be javascript, got {}", ct),
            )?;
            Ok(())
        })
        .await,
    );

    // === SECURITY HEADERS ===
    v0.push(
        run("security_headers", async {
            let v3 = v2
                .get(format!("{}/", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let headers = v3.headers();
            let xcto = headers
                .get("x-content-type-options")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(
                xcto == "nosniff",
                format!("X-Content-Type-Options must be nosniff, got '{}'", xcto),
            )?;
            let xfo = headers
                .get("x-frame-options")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(
                xfo == "SAMEORIGIN",
                format!("X-Frame-Options must be SAMEORIGIN, got '{}'", xfo),
            )?;
            let rp = headers
                .get("referrer-policy")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(
                rp == "strict-origin-when-cross-origin",
                format!("Referrer-Policy wrong, got '{}'", rp),
            )?;
            Ok(())
        })
        .await,
    );

    // === API CACHE HEADERS ===
    v0.push(
        run("api_openbooks_cache_header", async {
            let v3 = v2
                .get(format!("{}/api/openbooks", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let cc = v3
                .headers()
                .get("cache-control")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(
                cc.contains("max-age"),
                format!("/api/openbooks missing cache-control, got '{}'", cc),
            )?;
            Ok(())
        })
        .await,
    );
    v0.push(
        run("api_summary_cache_header", async {
            let v3 = v2
                .get(format!("{}/api/summary", v1))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let cc = v3
                .headers()
                .get("cache-control")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert_ok(
                cc.contains("max-age"),
                format!("/api/summary missing cache-control, got '{}'", cc),
            )?;
            Ok(())
        })
        .await,
    );

    v0
}
