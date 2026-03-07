fn main() {
    // run asset synchronization if possible
    if std::process::Command::new("sh")
        .arg("-c")
        .arg("$(pwd)/scripts/sync_assets.sh")
        .status()
        .is_ok()
    {
        println!("cargo:rerun-if-changed=scripts/sync_assets.sh");
    }
}
