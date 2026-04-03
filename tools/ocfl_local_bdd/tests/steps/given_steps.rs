use cucumber::given;

use super::{OcflWorld, run_cli};

#[given("an empty OCFL repository")]
async fn empty_ocfl_repo(world: &mut OcflWorld) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "init"]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}

#[given(expr = "object {string} has been added with content {string}")]
async fn object_added_with_content(world: &mut OcflWorld, object_id: String, content: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let src = world._repo_dir.path().join(format!("src_{}.txt", object_id));
    std::fs::write(&src, content.as_bytes()).expect("failed to write src file");
    let src_str = src.to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "add", &object_id, &src_str]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}
