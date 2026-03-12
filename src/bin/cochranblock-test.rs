// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use cochranblock::{screenshot, tests};

async fn run_once() -> bool {
    let v0 = tests::f30().await;
    if v0 {
        println!("\n=== Screenshot Capture ===");
        let _ = screenshot::f53().await;
    }
    v0
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let ok = exopack::triple_sims::f60(run_once).await;
    std::process::exit(if ok { 0 } else { 1 });
}
