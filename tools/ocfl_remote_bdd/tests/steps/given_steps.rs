use cucumber::given;
use std::process::Command;
use super::OcflWorld;

#[given("the OCFL endpoint is running")]
async fn endpoint_is_running(_world: &mut OcflWorld) {
    // For CLI, assume endpoint is running externally. No-op.
}

#[given("the repository is initialized")]
async fn repo_is_initialized(_world: &mut OcflWorld) {
    let status = Command::new("cargo")
        .args(["run", "-p", "ocfl_remote_cli", "--", "init"])
        .status()
        .expect("failed to run ocfl_remote_cli init");
    assert!(status.success(), "ocfl_remote_cli init failed");
}

#[given(expr = "a source file exists at {string}")]
async fn source_file_exists(_world: &mut OcflWorld, path: String) {
    std::fs::write(&path, b"hello").expect("failed to write source file");
}

#[given(expr = "object {string} has been added from {string}")]
async fn object_added(_world: &mut OcflWorld, object_id: String, src_path: String) {
    std::fs::write(&src_path, b"hello").ok();
    let status = Command::new("cargo")
        .args(["run", "-p", "ocfl_remote_cli", "--", "add", &object_id, &src_path])
        .status()
        .expect("failed to run ocfl_remote_cli add");
    assert!(status.success(), "ocfl_remote_cli add failed");
}

#[given(expr = "the file {string} does not exist")]
async fn file_does_not_exist(_world: &mut OcflWorld, path: String) {
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir_all(&path);
}

#[given(expr = "an empty OCFL repository")]
async fn empty_ocfl_repo(_world: &mut OcflWorld) {
    let status = Command::new("cargo")
        .args(["run", "-p", "ocfl_remote_cli", "--", "init"])
        .status()
        .expect("failed to run ocfl_remote_cli init");
    assert!(status.success(), "ocfl_remote_cli init failed");
}

#[given(expr = "an OCFL repository with object {string} at version {string}")]
async fn repo_with_object_at_version(_world: &mut OcflWorld, object_id: String, version: String) {
    // Always (re-)initialize the repo
    let status = Command::new("cargo")
        .args(["run", "-p", "ocfl_remote_cli", "--", "init"])
        .status()
        .expect("failed to run ocfl_remote_cli init");
    assert!(status.success(), "ocfl_remote_cli init failed");

    // Add v1
    let file_v1 = format!("/tmp/ocfl_bdd_{}_v1.txt", object_id);
    std::fs::write(&file_v1, b"version1").expect("failed to write v1 file");
    let status = Command::new("cargo")
        .args(["run", "-p", "ocfl_remote_cli", "--", "add", &object_id, &file_v1])
        .status()
        .expect("failed to run ocfl_remote_cli add");
    assert!(status.success(), "ocfl_remote_cli add failed");

    if version == "v2" {
        // Add v2
        let file_v2 = format!("/tmp/ocfl_bdd_{}_v2.txt", object_id);
        std::fs::write(&file_v2, b"version2").expect("failed to write v2 file");
        let status = Command::new("cargo")
            .args(["run", "-p", "ocfl_remote_cli", "--", "add-version", &object_id, &file_v2])
            .status()
            .expect("failed to run ocfl_remote_cli add-version");
        assert!(status.success(), "ocfl_remote_cli add-version failed");
    }
}
