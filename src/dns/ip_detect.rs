#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// All Rights Reserved — The Cochran Block, LLC
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use crate::error::t18;

pub async fn f13(ipify_url: &str) -> Result<String, t18> {
    let res = reqwest::get(ipify_url)
        .await
        .map_err(|e| t18::E4(e.to_string()))?;
    let text = res.text().await.map_err(|e| t18::E4(e.to_string()))?;
    Ok(text.trim().to_string())
}
