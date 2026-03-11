// Unlicense — cochranblock.org
// Packs assets/ with zstd for smaller binary.

fn main() {
    include_packed::Config::new("assets")
        .level(19)
        .build()
        .expect("pack assets");
}
