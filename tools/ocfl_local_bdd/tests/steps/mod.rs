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
