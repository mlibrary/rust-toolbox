pub mod given_steps;
pub mod when_steps;
pub mod then_steps;

use cucumber::World;
use tempfile::TempDir;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct OcflWorld {
    pub last_response_text: Option<String>,
    pub last_object_list: Vec<String>,
    pub _repo_dir: TempDir,
}

impl OcflWorld {
    async fn new() -> Self {
        let repo_dir = tempfile::tempdir().expect("failed to create tempdir");
        Self {
            last_response_text: None,
            last_object_list: Vec::new(),
            _repo_dir: repo_dir,
        }
    }
}

/// Locate the compiled ocfl_local_cli binary.
///
/// The path is baked in at compile time by build.rs, which derives it from `OUT_DIR`.
/// This means it always resolves to the correct target directory — including when
/// tarpaulin uses a non-default one. If the binary is absent (e.g. a fresh coverage
/// run), it is built on-demand via the `CARGO` env var that cargo injects at test runtime.
pub fn cli_bin() -> std::path::PathBuf {
    let bin = std::path::PathBuf::from(env!("OCFL_LOCAL_CLI_BIN"));
    if !bin.exists() {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let status = std::process::Command::new(&cargo)
            .args(["build", "-p", "ocfl_local_cli"])
            .status()
            .expect("failed to invoke cargo build for ocfl_local_cli");
        assert!(status.success(), "cargo build -p ocfl_local_cli failed");
    }
    assert!(
        bin.exists(),
        "ocfl_local_cli binary not found at {} after build",
        bin.display()
    );
    bin
}

/// Run the CLI with the given args and return (success, stdout).
pub fn run_cli(args: &[&str]) -> (bool, String) {
    let output = std::process::Command::new(cli_bin())
        .args(args)
        .output()
        .expect("failed to execute ocfl_local_cli");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    (output.status.success(), stdout)
}
