fn main() {
    // OUT_DIR = <target_dir>/<profile>/build/<pkg-hash>/out
    // Ascending 3 levels gives us <target_dir>/<profile>/
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_profile = std::path::Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap();
    let bin = target_profile.join("ocfl_local_cli");
    println!("cargo:rustc-env=OCFL_LOCAL_CLI_BIN={}", bin.display());

    // Rebuild when the CLI source changes
    println!("cargo:rerun-if-changed=../ocfl_local_cli/src/main.rs");
}
