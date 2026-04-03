#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::time::Instant;

use super::t24;
use super::assert_ok;
use crate::crypto::{key, token};
use crate::auth::{password, session};
use crate::config::t1;
use crate::error::t18;
use crate::dns::cloudflare;
use crate::web::intake;
use crate::web::community_grant;
use axum::http::HeaderMap;

async fn run(p0: &str, p1: impl std::future::Future<Output = Result<(), String>>) -> t24 {
    let v0 = Instant::now();
    match p1.await {
        Ok(()) => t24 { s30: p0.into(), s31: true, s32: v0.elapsed().as_millis() as u64, s33: None },
        Err(msg) => t24 { s30: p0.into(), s31: false, s32: v0.elapsed().as_millis() as u64, s33: Some(msg) },
    }
}

pub async fn f49() -> Vec<t24> {
    let mut v0 = Vec::new();
    v0.push(run("crypto_roundtrip", async {
        let v1: [u8; 32] = key::f31("master", "ctx").map_err(|e| e.to_string())?;
        let v2 = token::f14("secret", &v1).map_err(|e| e.to_string())?;
        let v3 = token::f15(&v2, &v1).map_err(|e| e.to_string())?;
        assert_ok(v3 == "secret", "decrypt did not match original")?;
        Ok(())
    }).await);
    v0.push(run("crypto_tamper", async {
        let v1: [u8; 32] = key::f31("m", "c").map_err(|e| e.to_string())?;
        let mut v2 = token::f14("x", &v1).map_err(|e| e.to_string())?.into_bytes();
        v2[20] ^= 0xff;
        assert_ok(token::f15(&String::from_utf8_lossy(&v2), &v1).is_err(), "tampered token should fail decrypt")?;
        Ok(())
    }).await);
    v0.push(run("crypto_wrong_key", async {
        let v1: [u8; 32] = key::f31("a", "c").map_err(|e| e.to_string())?;
        let v2: [u8; 32] = key::f31("b", "c").map_err(|e| e.to_string())?;
        let v3 = token::f14("x", &v1).map_err(|e| e.to_string())?;
        assert_ok(token::f15(&v3, &v2).is_err(), "wrong key should fail decrypt")?;
        Ok(())
    }).await);
    v0.push(run("crypto_determinism", async {
        let v1: [u8; 32] = key::f31("m", "c").map_err(|e| e.to_string())?;
        let v2 = token::f14("x", &v1).map_err(|e| e.to_string())?;
        let v3 = token::f14("x", &v1).map_err(|e| e.to_string())?;
        assert_ok(v2 != v3, "AEAD must produce different ciphertext each time")?;
        Ok(())
    }).await);
    v0.push(run("crypto_not_plaintext", async {
        let v1: [u8; 32] = key::f31("m", "c").map_err(|e| e.to_string())?;
        let v2 = token::f14("my_secret", &v1).map_err(|e| e.to_string())?;
        assert_ok(!v2.contains("my_secret"), "ciphertext must not leak plaintext")?;
        Ok(())
    }).await);
    v0.push(run("key_derivation_isolation", async {
        let v1 = key::f31("m", "ctx1").map_err(|e| e.to_string())?;
        let v2 = key::f31("m", "ctx2").map_err(|e| e.to_string())?;
        assert_ok(v1 != v2, "different context must yield different keys")?;
        assert_ok(key::f31("m", "c").map_err(|e| e.to_string())? != key::f32("m").map_err(|e| e.to_string())?, "f31 vs f32 must differ")?;
        Ok(())
    }).await);
    v0.push(run("password_hash_verify", async {
        let v1 = password::f16("test123").map_err(|e| e.to_string())?;
        assert_ok(password::f17("test123", &v1).map_err(|e| e.to_string())?, "correct password must verify")?;
        Ok(())
    }).await);
    v0.push(run("password_unique_salts", async {
        let v1 = password::f16("x").map_err(|e| e.to_string())?;
        let v2 = password::f16("x").map_err(|e| e.to_string())?;
        assert_ok(v1 != v2, "each hash must use unique salt")?;
        Ok(())
    }).await);
    v0.push(run("password_wrong_returns_false", async {
        let v1 = password::f16("right").map_err(|e| e.to_string())?;
        assert_ok(!password::f17("wrong", &v1).map_err(|e| e.to_string())?, "wrong password must not verify")?;
        Ok(())
    }).await);
    v0.push(run("session_creation", async {
        let v1 = session::t3::f18("user");
        assert_ok(v1.s10 == "user", "session must store username")?;
        assert_ok(v1.f19(), "new session must be valid")?;
        Ok(())
    }).await);
    v0.push(run("session_expired_invalid", async {
        let v1 = session::t3 { s37: "x".into(), s10: "u".into(), s13: "2020-01-01T00:00:00Z".into(), s14: "2020-01-01T00:00:01Z".into() };
        assert_ok(!v1.f19(), "expired session must be invalid")?;
        Ok(())
    }).await);
    v0.push(run("config_defaults", async {
        let v1 = t1::default();
        assert_ok(v1.s16 == 3000, "default port must be 3000")?;
        assert_ok(v1.s18 == "data", "default db dir must be data")?;
        Ok(())
    }).await);
    v0.push(run("config_db_path", async {
        let v1 = t1::default();
        assert_ok(v1.db_path() == "data/cochranblock.db", "db_path must be data/cochranblock.db")?;
        Ok(())
    }).await);
    v0.push(run("config_with_values", async {
        let v1 = t1::with_values(8080, "0.0.0.0", "/var/data", "key", "https://cf", "https://ip", 60);
        assert_ok(v1.s16 == 8080, "port must match")?;
        assert_ok(v1.s17 == "0.0.0.0", "bind must match")?;
        assert_ok(v1.db_path() == "/var/data/cochranblock.db", "db_path must reflect data dir")?;
        Ok(())
    }).await);
    v0.push(run("error_no_internal_leaks", async {
        let v1 = t18::E5("secret db path".into()).public_message();
        assert_ok(!v1.contains("secret"), "public message must not leak internal details")?;
        Ok(())
    }).await);
    v0.push(run("dns_url_build", async {
        let v1 = cloudflare::f47("https://api.cf.com", "zone1", "rec1");
        assert_ok(v1.contains("zone1"), "URL must contain zone")?;
        assert_ok(v1.contains("rec1"), "URL must contain record")?;
        Ok(())
    }).await);
    v0.push(run("dns_ip_changed", async {
        assert_ok(cloudflare::f48("1.2.3.4", "1.2.3.5"), "different IPs must be changed")?;
        assert_ok(!cloudflare::f48("1.2.3.4", "1.2.3.4"), "same IP must not be changed")?;
        Ok(())
    }).await);
    v0.push(run("dns_empty_token", async {
        assert_ok(cloudflare::f46("").is_err(), "empty token must error")?;
        Ok(())
    }).await);
    v0.push(run("key_f32_deterministic", async {
        let v1 = key::f32("seed").map_err(|e| e.to_string())?;
        let v2 = key::f32("seed").map_err(|e| e.to_string())?;
        assert_ok(v1 == v2, "f32 must be deterministic for same input")?;
        assert_ok(v1.len() == 32, "f32 must produce 32 bytes")?;
        Ok(())
    }).await);
    v0.push(run("session_id_format", async {
        let v1 = session::t3::f18("u");
        assert_ok(v1.s37.len() == 36, "session id must be UUID length")?;
        assert_ok(v1.s37.contains('-'), "session id must be UUID format")?;
        Ok(())
    }).await);

    // === CRYPTO: Edge cases ===
    v0.push(run("crypto_empty_plaintext", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        let ct = token::f14("", &k).map_err(|e| e.to_string())?;
        let pt = token::f15(&ct, &k).map_err(|e| e.to_string())?;
        assert_ok(pt.is_empty(), "empty plaintext roundtrip")?;
        Ok(())
    }).await);
    v0.push(run("crypto_unicode_plaintext", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        let ct = token::f14("🔐 秘密 café", &k).map_err(|e| e.to_string())?;
        let pt = token::f15(&ct, &k).map_err(|e| e.to_string())?;
        assert_ok(pt == "🔐 秘密 café", "unicode roundtrip")?;
        Ok(())
    }).await);
    v0.push(run("crypto_long_plaintext", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        let long = "x".repeat(100_000);
        let ct = token::f14(&long, &k).map_err(|e| e.to_string())?;
        let pt = token::f15(&ct, &k).map_err(|e| e.to_string())?;
        assert_ok(pt == long, "100KB plaintext roundtrip")?;
        Ok(())
    }).await);
    v0.push(run("crypto_null_bytes_plaintext", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        let ct = token::f14("\x00\x01\x02", &k).map_err(|e| e.to_string())?;
        let pt = token::f15(&ct, &k).map_err(|e| e.to_string())?;
        assert_ok(pt == "\x00\x01\x02", "null bytes roundtrip")?;
        Ok(())
    }).await);
    v0.push(run("crypto_empty_ciphertext_fails", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        assert_ok(token::f15("", &k).is_err(), "empty ciphertext must error")?;
        Ok(())
    }).await);
    v0.push(run("crypto_garbage_ciphertext_fails", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        assert_ok(token::f15("not-valid-base64!!!", &k).is_err(), "garbage ciphertext must error")?;
        Ok(())
    }).await);
    v0.push(run("crypto_truncated_ciphertext_fails", async {
        let k = key::f31("m", "c").map_err(|e| e.to_string())?;
        let ct = token::f14("test", &k).map_err(|e| e.to_string())?;
        let truncated = &ct[..ct.len() / 2];
        assert_ok(token::f15(truncated, &k).is_err(), "truncated ciphertext must error")?;
        Ok(())
    }).await);

    // === KEY DERIVATION: Edge cases ===
    v0.push(run("key_empty_master", async {
        let r = key::f31("", "ctx");
        assert_ok(r.is_ok(), "empty master key should still derive (HKDF handles it)")?;
        Ok(())
    }).await);
    v0.push(run("key_long_context", async {
        let ctx = "a".repeat(10_000);
        let r = key::f31("master", &ctx).map_err(|e| e.to_string())?;
        assert_ok(r.len() == 32, "long context must produce 32 bytes")?;
        Ok(())
    }).await);
    v0.push(run("key_unicode_context", async {
        let r = key::f31("master", "上下文").map_err(|e| e.to_string())?;
        assert_ok(r.len() == 32, "unicode context must produce 32 bytes")?;
        Ok(())
    }).await);
    v0.push(run("key_same_master_same_ctx_deterministic", async {
        let a = key::f31("x", "y").map_err(|e| e.to_string())?;
        let b = key::f31("x", "y").map_err(|e| e.to_string())?;
        assert_ok(a == b, "same inputs must produce same key")?;
        Ok(())
    }).await);

    // === PASSWORD: Edge cases ===
    v0.push(run("password_empty_string", async {
        let h = password::f16("").map_err(|e| e.to_string())?;
        assert_ok(password::f17("", &h).map_err(|e| e.to_string())?, "empty password must verify")?;
        assert_ok(!password::f17("x", &h).map_err(|e| e.to_string())?, "non-empty must not verify against empty hash")?;
        Ok(())
    }).await);
    v0.push(run("password_unicode", async {
        let h = password::f16("パスワード").map_err(|e| e.to_string())?;
        assert_ok(password::f17("パスワード", &h).map_err(|e| e.to_string())?, "unicode password must verify")?;
        assert_ok(!password::f17("パスワード!", &h).map_err(|e| e.to_string())?, "different unicode must not verify")?;
        Ok(())
    }).await);
    v0.push(run("password_long", async {
        let long = "a".repeat(10_000);
        let h = password::f16(&long).map_err(|e| e.to_string())?;
        assert_ok(password::f17(&long, &h).map_err(|e| e.to_string())?, "long password must verify")?;
        Ok(())
    }).await);
    v0.push(run("password_special_chars", async {
        let pw = r#"!@#$%^&*()_+-=[]{}|;:'",.<>?/~`"#;
        let h = password::f16(pw).map_err(|e| e.to_string())?;
        assert_ok(password::f17(pw, &h).map_err(|e| e.to_string())?, "special char password must verify")?;
        Ok(())
    }).await);
    v0.push(run("password_verify_malformed_hash", async {
        let r = password::f17("pw", "not-a-valid-phc-hash");
        assert_ok(r.is_err() || !r.unwrap_or(true), "malformed hash must error or return false")?;
        Ok(())
    }).await);

    // === SESSION: Edge cases ===
    v0.push(run("session_unicode_username", async {
        let v1 = session::t3::f18("用户");
        assert_ok(v1.s10 == "用户", "unicode username must be stored")?;
        assert_ok(v1.f19(), "unicode session must be valid")?;
        Ok(())
    }).await);
    v0.push(run("session_empty_username", async {
        let v1 = session::t3::f18("");
        assert_ok(v1.s10.is_empty(), "empty username must be stored")?;
        assert_ok(v1.f19(), "empty username session must be valid (expiry-based)")?;
        Ok(())
    }).await);
    v0.push(run("session_long_username", async {
        let long = "u".repeat(10_000);
        let v1 = session::t3::f18(&long);
        assert_ok(v1.s10 == long, "long username must be stored")?;
        Ok(())
    }).await);
    v0.push(run("session_garbage_expiry", async {
        let v1 = session::t3 { s37: "id".into(), s10: "u".into(), s13: "now".into(), s14: "garbage".into() };
        assert_ok(!v1.f19(), "garbage expiry must be invalid")?;
        Ok(())
    }).await);
    v0.push(run("session_unique_ids", async {
        let a = session::t3::f18("u");
        let b = session::t3::f18("u");
        assert_ok(a.s37 != b.s37, "each session must have unique id")?;
        Ok(())
    }).await);

    // === ERROR: All variants ===
    v0.push(run("error_e0_status_401", async {
        assert_ok(t18::E0.status_code() == 401, "E0 must be 401")?;
        assert_ok(t18::E0.public_message() == "Unauthorized", "E0 message")?;
        Ok(())
    }).await);
    v0.push(run("error_e1_status_401", async {
        assert_ok(t18::E1.status_code() == 401, "E1 must be 401")?;
        assert_ok(t18::E1.public_message() == "Invalid credentials", "E1 message")?;
        Ok(())
    }).await);
    v0.push(run("error_e2_status_401", async {
        assert_ok(t18::E2.status_code() == 401, "E2 must be 401")?;
        assert_ok(t18::E2.public_message() == "Session expired", "E2 message")?;
        Ok(())
    }).await);
    v0.push(run("error_e3_status_502", async {
        assert_ok(t18::E3("x".into()).status_code() == 502, "E3 must be 502")?;
        assert_ok(t18::E3("secret".into()).public_message() == "External service error", "E3 no leak")?;
        Ok(())
    }).await);
    v0.push(run("error_e4_status_503", async {
        assert_ok(t18::E4("x".into()).status_code() == 503, "E4 must be 503")?;
        assert_ok(t18::E4("internal".into()).public_message() == "IP lookup failed", "E4 no leak")?;
        Ok(())
    }).await);
    v0.push(run("error_e5_status_500", async {
        assert_ok(t18::E5("x".into()).status_code() == 500, "E5 must be 500")?;
        assert_ok(!t18::E5("path/to/db".into()).public_message().contains("path"), "E5 no path leak")?;
        Ok(())
    }).await);
    v0.push(run("error_e6_status_500", async {
        assert_ok(t18::E6("x".into()).status_code() == 500, "E6 must be 500")?;
        Ok(())
    }).await);
    v0.push(run("error_e7_status_500", async {
        assert_ok(t18::E7("x".into()).status_code() == 500, "E7 must be 500")?;
        Ok(())
    }).await);
    v0.push(run("error_e8_status_307", async {
        assert_ok(t18::E8.status_code() == 307, "E8 must be 307")?;
        assert_ok(t18::E8.public_message() == "Setup required", "E8 message")?;
        Ok(())
    }).await);
    v0.push(run("error_e9_status_429", async {
        assert_ok(t18::E9.status_code() == 429, "E9 must be 429")?;
        assert_ok(t18::E9.public_message() == "Too many requests", "E9 message")?;
        Ok(())
    }).await);
    v0.push(run("error_e10_status_500", async {
        assert_ok(t18::E10("secret".into()).status_code() == 500, "E10 must be 500")?;
        assert_ok(!t18::E10("db_password=xyz".into()).public_message().contains("xyz"), "E10 no leak")?;
        Ok(())
    }).await);

    // === CONFIG: Edge cases ===
    v0.push(run("config_db_path_trailing_slash", async {
        let v1 = t1::with_values(3000, "127.0.0.1", "/data/", "k", "cf", "ip", 60);
        assert_ok(v1.db_path() == "/data//cochranblock.db", "trailing slash preserved in path join")?;
        Ok(())
    }).await);
    v0.push(run("config_with_values_all_fields", async {
        let v1 = t1::with_values(65535, "0.0.0.0", "/tmp", "masterkey1234567", "https://cf", "https://ip", 1);
        assert_ok(v1.s16 == 65535, "max port")?;
        assert_ok(v1.s17 == "0.0.0.0", "wildcard bind")?;
        assert_ok(v1.s9 == 1, "min interval")?;
        Ok(())
    }).await);
    v0.push(run("config_default_master_key_length", async {
        let v1 = t1::default();
        assert_ok(v1.s20.len() >= 16, "default master key must be >= 16 bytes")?;
        Ok(())
    }).await);

    // === INTAKE VALIDATION ===
    v0.push(run("validate_input_valid", async {
        assert_ok(intake::validate_input("John Doe", "john@example.com").is_ok(), "valid input")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_empty_name", async {
        assert_ok(intake::validate_input("", "john@example.com").is_err(), "empty name must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_empty_email", async {
        assert_ok(intake::validate_input("John", "").is_err(), "empty email must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_both_empty", async {
        assert_ok(intake::validate_input("", "").is_err(), "both empty must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_whitespace_name", async {
        assert_ok(intake::validate_input("   ", "j@e.com").is_err(), "whitespace-only name must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_whitespace_email", async {
        assert_ok(intake::validate_input("John", "   ").is_err(), "whitespace-only email must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_name_too_long", async {
        let long = "a".repeat(201);
        assert_ok(intake::validate_input(&long, "j@e.com").is_err(), "name > 200 must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_name_at_boundary", async {
        let exact = "a".repeat(200);
        assert_ok(intake::validate_input(&exact, "j@e.com").is_ok(), "name = 200 must pass")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_email_too_long", async {
        let long = format!("a@{}.com", "b".repeat(250));
        assert_ok(intake::validate_input("John", &long).is_err(), "email > 254 must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_email_at_boundary", async {
        // "a@" + "b" * N + ".com" = 2 + N + 4 = 254, so N = 248
        let exact = format!("a@{}.com", "b".repeat(248));
        assert_ok(exact.len() == 254, format!("boundary email must be 254, got {}", exact.len()))?;
        assert_ok(intake::validate_input("John", &exact).is_ok(), "email = 254 must pass")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_unicode_name", async {
        assert_ok(intake::validate_input("José García", "j@e.com").is_ok(), "unicode name must pass")?;
        Ok(())
    }).await);
    v0.push(run("validate_input_sql_injection", async {
        assert_ok(intake::validate_input("'; DROP TABLE users; --", "j@e.com").is_ok(), "SQL injection in name passes validation (DB is parameterized)")?;
        Ok(())
    }).await);

    // === GRANT VALIDATION ===
    v0.push(run("validate_grant_valid", async {
        assert_ok(community_grant::validate_grant_input("Org", "Jane", "j@o.org", "Mission", "Objective").is_ok(), "valid grant")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_empty_org", async {
        assert_ok(community_grant::validate_grant_input("", "Jane", "j@o.org", "M", "O").is_err(), "empty org must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_empty_contact", async {
        assert_ok(community_grant::validate_grant_input("Org", "", "j@o.org", "M", "O").is_err(), "empty contact must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_empty_email", async {
        assert_ok(community_grant::validate_grant_input("Org", "Jane", "", "M", "O").is_err(), "empty email must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_empty_mission", async {
        assert_ok(community_grant::validate_grant_input("Org", "Jane", "j@o.org", "", "O").is_err(), "empty mission must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_empty_objective", async {
        assert_ok(community_grant::validate_grant_input("Org", "Jane", "j@o.org", "M", "").is_err(), "empty objective must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_whitespace_only", async {
        assert_ok(community_grant::validate_grant_input("  ", "Jane", "j@o.org", "M", "O").is_err(), "whitespace org must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_org_too_long", async {
        let long = "a".repeat(201);
        assert_ok(community_grant::validate_grant_input(&long, "Jane", "j@o.org", "M", "O").is_err(), "org > 200 must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_org_at_boundary", async {
        let exact = "a".repeat(200);
        assert_ok(community_grant::validate_grant_input(&exact, "Jane", "j@o.org", "M", "O").is_ok(), "org = 200 must pass")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_mission_too_long", async {
        let long = "a".repeat(2001);
        assert_ok(community_grant::validate_grant_input("Org", "Jane", "j@o.org", &long, "O").is_err(), "mission > 2000 must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_mission_at_boundary", async {
        let exact = "a".repeat(2000);
        assert_ok(community_grant::validate_grant_input("Org", "Jane", "j@o.org", &exact, "O").is_ok(), "mission = 2000 must pass")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_email_too_long", async {
        let long = format!("a@{}.org", "b".repeat(250));
        assert_ok(community_grant::validate_grant_input("Org", "Jane", &long, "M", "O").is_err(), "email > 254 must error")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_unicode", async {
        assert_ok(community_grant::validate_grant_input("組織", "田中", "t@o.jp", "使命", "目的").is_ok(), "unicode grant fields must pass")?;
        Ok(())
    }).await);
    v0.push(run("validate_grant_all_empty", async {
        assert_ok(community_grant::validate_grant_input("", "", "", "", "").is_err(), "all empty must error")?;
        Ok(())
    }).await);

    // === HTML ESCAPE ===
    v0.push(run("html_escape_empty", async {
        assert_ok(intake::html_escape("").is_empty(), "empty input")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_no_special", async {
        assert_ok(intake::html_escape("hello world") == "hello world", "no special chars")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_ampersand", async {
        assert_ok(intake::html_escape("a&b") == "a&amp;b", "ampersand")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_angle_brackets", async {
        assert_ok(intake::html_escape("<script>alert(1)</script>") == "&lt;script&gt;alert(1)&lt;/script&gt;", "angle brackets")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_quotes", async {
        assert_ok(intake::html_escape(r#"say "hello""#) == "say &quot;hello&quot;", "double quotes")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_all_special", async {
        let r = intake::html_escape(r#"<a href="x&y">"#);
        assert_ok(!r.contains('<') && !r.contains('>') && !r.contains('"') && !r.contains('&') || r.contains("&amp;"), "all special chars escaped")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_unicode_passthrough", async {
        assert_ok(intake::html_escape("café 🎉") == "café 🎉", "unicode passes through")?;
        Ok(())
    }).await);
    v0.push(run("html_escape_already_escaped", async {
        let r = intake::html_escape("&amp;");
        assert_ok(r == "&amp;amp;", "double-escaping must happen (no smart detection)")?;
        Ok(())
    }).await);

    // === DNS: Edge cases ===
    v0.push(run("dns_ip_same_not_changed", async {
        assert_ok(!cloudflare::f48("1.2.3.4", "1.2.3.4"), "same IP = not changed")?;
        assert_ok(!cloudflare::f48("::1", "::1"), "same IPv6 = not changed")?;
        Ok(())
    }).await);
    v0.push(run("dns_ip_empty_vs_empty", async {
        assert_ok(!cloudflare::f48("", ""), "empty vs empty = not changed")?;
        Ok(())
    }).await);
    v0.push(run("dns_ip_empty_vs_value", async {
        assert_ok(cloudflare::f48("", "1.2.3.4"), "empty vs value = changed")?;
        Ok(())
    }).await);
    v0.push(run("dns_url_components", async {
        let url = cloudflare::f47("https://api.cf.com/client/v4", "z123", "r456");
        assert_ok(url.starts_with("https://"), "URL must start with https")?;
        assert_ok(url.contains("zones/z123"), "URL must contain zone")?;
        assert_ok(url.contains("dns_records/r456"), "URL must contain record")?;
        Ok(())
    }).await);
    v0.push(run("dns_token_whitespace", async {
        assert_ok(cloudflare::f46("  ").is_err(), "whitespace token must error")?;
        Ok(())
    }).await);
    v0.push(run("dns_token_valid", async {
        assert_ok(cloudflare::f46("abc123token").is_ok(), "non-empty token must pass")?;
        Ok(())
    }).await);

    // === CLIENT IP EXTRACTION ===
    v0.push(run("client_ip_from_socket", async {
        let addr: std::net::SocketAddr = "127.0.0.1:1234".parse().unwrap();
        let headers = HeaderMap::new();
        let ip = community_grant::client_ip(addr, &headers);
        assert_ok(ip == "127.0.0.1", format!("socket IP must be 127.0.0.1, got {}", ip))?;
        Ok(())
    }).await);
    v0.push(run("client_ip_from_xff", async {
        let addr: std::net::SocketAddr = "127.0.0.1:1234".parse().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.1".parse().unwrap());
        let ip = community_grant::client_ip(addr, &headers);
        assert_ok(ip == "203.0.113.1", format!("XFF IP must be 203.0.113.1, got {}", ip))?;
        Ok(())
    }).await);
    v0.push(run("client_ip_xff_first_of_chain", async {
        let addr: std::net::SocketAddr = "10.0.0.1:80".parse().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "1.2.3.4, 5.6.7.8, 9.10.11.12".parse().unwrap());
        let ip = community_grant::client_ip(addr, &headers);
        assert_ok(ip == "1.2.3.4", format!("must take first XFF entry, got {}", ip))?;
        Ok(())
    }).await);
    v0.push(run("client_ip_xff_trimmed", async {
        let addr: std::net::SocketAddr = "10.0.0.1:80".parse().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "  9.8.7.6  , 1.2.3.4".parse().unwrap());
        let ip = community_grant::client_ip(addr, &headers);
        assert_ok(ip == "9.8.7.6", format!("XFF must be trimmed, got {}", ip))?;
        Ok(())
    }).await);
    v0.push(run("client_ip_ipv6_socket", async {
        let addr: std::net::SocketAddr = "[::1]:8080".parse().unwrap();
        let headers = HeaderMap::new();
        let ip = community_grant::client_ip(addr, &headers);
        assert_ok(ip == "::1", format!("IPv6 socket must return ::1, got {}", ip))?;
        Ok(())
    }).await);

    // === COMMUNITY_GRANT HTML ESCAPE (dedup coverage via grant module) ===
    v0.push(run("grant_html_escape_script_tag", async {
        let r = community_grant::html_escape("<script>alert('xss')</script>");
        assert_ok(!r.contains('<') && !r.contains('>'), "grant html_escape must remove angle brackets")?;
        assert_ok(r.contains("&lt;script&gt;"), "grant html_escape must encode script tag")?;
        Ok(())
    }).await);

    v0
}