fn main() {
    // run asset synchronization if possible
    let script_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("scripts/sync_assets.sh");
    
    if std::process::Command::new("sh")
        .arg(script_path)
        .status()
        .is_ok()
    {
        println!("cargo:rerun-if-changed=scripts/sync_assets.sh");
    }
}
