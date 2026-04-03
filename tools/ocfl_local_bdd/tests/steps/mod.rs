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

/// Locate the compiled ocfl_local_cli binary in the workspace target directory.
pub fn cli_bin() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR = tools/ocfl_local_bdd
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest.parent().unwrap().parent().unwrap();
    let bin = workspace.join("target").join("debug").join("ocfl_local_cli");
    assert!(
        bin.exists(),
        "ocfl_local_cli binary not found at {}. Run `cargo build -p ocfl_local_cli` first.",
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
