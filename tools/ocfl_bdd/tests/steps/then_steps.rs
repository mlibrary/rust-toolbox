use cucumber::then;

use super::OcflWorld;

#[then(expr = "the response body is {string}")]
async fn response_body_is(world: &mut OcflWorld, expected: String) {
    let actual = world
        .last_response_text
        .as_deref()
        .expect("no response recorded");
    // The endpoint returns a JSON string, e.g. `"ok"` (with quotes).
    assert_eq!(actual, format!("\"{expected}\""));
}

#[then(expr = "the response body is not {string}")]
async fn response_body_is_not(world: &mut OcflWorld, not_expected: String) {
    let actual = world
        .last_response_text
        .as_deref()
        .expect("no response recorded");
    // The endpoint returns a JSON string, e.g. `"ok"` (with quotes).
    assert_ne!(actual, format!("\"{not_expected}\""));
}

#[then(expr = "the object list contains {string}")]
async fn object_list_contains(world: &mut OcflWorld, object_id: String) {
    assert!(
        world.last_object_list.contains(&object_id),
        "expected '{object_id}' in list, got {:?}",
        world.last_object_list
    );
}

#[then("the object list is empty")]
async fn object_list_is_empty(world: &mut OcflWorld) {
    assert!(
        world.last_object_list.is_empty(),
        "expected empty list, got {:?}",
        world.last_object_list
    );
}

#[then(expr = "the file {string} exists and contains {string}")]
async fn file_exists_and_contains(_world: &mut OcflWorld, path: String, expected: String) {
    let content = std::fs::read_to_string(&path).expect("file does not exist");
    assert_eq!(content, expected);
}

#[then(expr = "the object {string} should exist in the repository")]
async fn object_should_exist_in_repo(_world: &mut OcflWorld, object_id: String) {
    let repo_root = std::env::var("OCFL_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
    let object_path = std::path::Path::new(&repo_root).join(&object_id);
    assert!(object_path.exists(), "Object root does not exist: {}", object_path.display());
}

#[then(expr = "the inventory for object {string} should exist")]
async fn inventory_should_exist(_world: &mut OcflWorld, object_id: String) {
    let repo_root = std::env::var("OCFL_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
    let inventory_path = std::path::Path::new(&repo_root).join(&object_id).join("inventory.json");
    assert!(inventory_path.exists(), "inventory.json does not exist: {}", inventory_path.display());
}

#[then(expr = "the inventory for object {string} should indicate version {string}")]
async fn inventory_should_indicate_version(_world: &mut OcflWorld, object_id: String, version: String) {
    let repo_root = std::env::var("OCFL_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
    let inventory_path = std::path::Path::new(&repo_root).join(&object_id).join("inventory.json");
    let data = std::fs::read_to_string(&inventory_path).expect("Failed to read inventory.json");
    let v: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON");
    assert_eq!(v["head"], version, "Inventory head does not match version");
}

#[then(expr = "the inventory for object {string} should list both versions {string} and {string}")]
async fn inventory_should_list_both_versions(_world: &mut OcflWorld, object_id: String, v1: String, v2: String) {
    let repo_root = std::env::var("OCFL_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
    let inventory_path = std::path::Path::new(&repo_root).join(&object_id).join("inventory.json");
    let data = std::fs::read_to_string(&inventory_path).expect("Failed to read inventory.json");
    let v: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON");
    let versions = v["versions"].as_object().expect("No versions object");
    assert!(versions.contains_key(&v1), "Inventory missing version {}", v1);
    assert!(versions.contains_key(&v2), "Inventory missing version {}", v2);
}

#[then(expr = "the inventory should be valid per OCFL 1.1 spec")]
async fn inventory_should_be_valid(_world: &mut OcflWorld) {
    // TODO: Implement OCFL 1.1 inventory validation
    unimplemented!("inventory_should_be_valid");
}

#[then(expr = "the inventory should list all versions and content digests")]
async fn inventory_should_list_versions_and_digests(_world: &mut OcflWorld) {
    // TODO: Implement check for all versions and digests in inventory.json
    unimplemented!("inventory_should_list_versions_and_digests");
}

#[then(expr = "the result should include {string} and {string}")]
async fn result_should_include_versions(_world: &mut OcflWorld, _v1: String, _v2: String) {
    // TODO: Implement check for version list result
    unimplemented!("result_should_include_versions");
}
