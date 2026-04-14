// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

// Packs assets/ with zstd for smaller binary.
// Also emits GIT_COMMIT + BUILD_TIMESTAMP for the /sovereignty page.

use std::process::Command;

fn main() {
    include_packed::Config::new("assets")
        .level(19)
        .build()
        .expect("pack assets");

    // Emit git commit (short + full) so the running binary can prove what source
    // it was built from. If git is unavailable (e.g. tarball build), fall back
    // to "unknown" — the binary still works, the sovereignty proof just can't
    // cite a specific commit.
    let git_full = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() { String::from_utf8(o.stdout).ok() } else { None })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let git_short = git_full.chars().take(10).collect::<String>();
    println!("cargo:rustc-env=GIT_COMMIT={}", git_full);
    println!("cargo:rustc-env=GIT_COMMIT_SHORT={}", git_short);

    // Build timestamp (UTC RFC3339) so the sovereignty page can show when the
    // running binary was compiled. Useful as a "freshness" signal independent
    // of server uptime.
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", ts);

    // Rerun if .git/HEAD moves (branch switch or new commit).
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");
}
