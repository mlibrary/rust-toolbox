use cucumber::when;

use super::{OcflWorld, run_cli};

#[when("I initialize the repository")]
async fn init_repo(world: &mut OcflWorld) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "init"]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}

#[when(expr = "I add an object with id {string} and content file {string}")]
async fn add_object(world: &mut OcflWorld, object_id: String, file_name: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let src = world._repo_dir.path().join(&file_name);
    std::fs::write(&src, b"test content").expect("failed to write src file");
    let src_str = src.to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "add", &object_id, &src_str]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}

#[when("I list all objects")]
async fn list_all_objects(world: &mut OcflWorld) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, stdout) = run_cli(&["--repo", &root, "list"]);
    if ok {
        // CLI prints: Objects: ["obj1", "obj2"]  (Rust Debug fmt == valid JSON for Vec<String>)
        let json = stdout.trim().trim_start_matches("Objects: ");
        let objects: Vec<String> = serde_json::from_str(json).unwrap_or_default();
        world.last_object_list = objects.clone();
        world.last_response_text = Some(serde_json::to_string(&objects).unwrap());
    } else {
        world.last_object_list = vec![];
        world.last_response_text = Some("[]".to_string());
    }
}

#[when(expr = "I retrieve object {string} to path {string}")]
async fn retrieve_object(world: &mut OcflWorld, object_id: String, dest: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let dest_path = std::path::Path::new(&dest);
    if dest_path.exists() {
        std::fs::remove_file(dest_path).ok();
    }
    let (ok, _) = run_cli(&["--repo", &root, "get", &object_id, &dest]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}

#[when(expr = "I retrieve the inventory for object {string}")]
async fn retrieve_inventory(world: &mut OcflWorld, object_id: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, stdout) = run_cli(&["--repo", &root, "inventory", &object_id]);
    if ok {
        // CLI prints: Inventory: {...json...}
        let json = stdout.trim().trim_start_matches("Inventory: ");
        world.last_response_text = Some(json.to_string());
    } else {
        world.last_response_text = Some("error".to_string());
    }
}

#[when(expr = "I list versions for object {string}")]
async fn list_versions(world: &mut OcflWorld, object_id: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, stdout) = run_cli(&["--repo", &root, "versions", &object_id]);
    if ok {
        // CLI prints: Versions: ["v1", "v2"]
        let json = stdout.trim().trim_start_matches("Versions: ");
        world.last_response_text = Some(
            serde_json::to_string(&serde_json::json!({ "versions": serde_json::from_str::<serde_json::Value>(json).unwrap_or(serde_json::json!([])) })).unwrap()
        );
    } else {
        world.last_response_text = Some("error".to_string());
    }
}

#[when(expr = "I delete object {string}")]
async fn delete_object(world: &mut OcflWorld, object_id: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "delete-object", &object_id]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}

#[when(expr = "I delete version {string} of object {string}")]
async fn delete_version(world: &mut OcflWorld, version: String, object_id: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "delete-version", &object_id, &version]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}

#[when(expr = "I add a new version to object {string} with content file {string}")]
async fn add_new_version(world: &mut OcflWorld, object_id: String, file_name: String) {
    let root = world._repo_dir.path().to_string_lossy().to_string();
    let src = world._repo_dir.path().join(&file_name);
    std::fs::write(&src, b"version content").expect("failed to write src file");
    let src_str = src.to_string_lossy().to_string();
    let (ok, _) = run_cli(&["--repo", &root, "add-version", &object_id, &src_str]);
    world.last_response_text = Some(if ok { "ok".to_string() } else { "error".to_string() });
}
