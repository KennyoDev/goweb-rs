pub fn getFilePath() -> String {
    return format!("{}/json/webSites.json", env!("CARGO_MANIFEST_DIR"));
}
