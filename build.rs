// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

// Packs assets/ with zstd for smaller binary.

fn main() {
    include_packed::Config::new("assets")
        .level(19)
        .build()
        .expect("pack assets");
}
