use cucumber::then;
use super::OcflWorld;
use serde_json::Value;

#[then(expr = "the response body is {string}")]
async fn response_body_is(world: &mut OcflWorld, expected: String) {
	let actual = world
		.last_response_text
		.as_deref()
		.expect("no response recorded");
	assert_eq!(actual.trim(), expected.trim());
}

#[then(expr = "the response body is not {string}")]
async fn response_body_is_not(world: &mut OcflWorld, not_expected: String) {
	let actual = world
		.last_response_text
		.as_deref()
		.expect("no response recorded");
	assert_ne!(actual.trim(), not_expected.trim());
}

#[then(expr = "the object list contains {string}")]
async fn object_list_contains(world: &mut OcflWorld, object_id: String) {
	let actual = world.last_response_text.as_ref().expect("no response");
	let objects: Vec<String> = serde_json::from_str(actual).unwrap_or_default();
	assert!(objects.contains(&object_id), "expected '{}' in list, got {:?}", object_id, objects);
}

#[then("the object list is empty")]
async fn object_list_is_empty(world: &mut OcflWorld) {
	let actual = world.last_response_text.as_ref().expect("no response");
	let objects: Vec<String> = serde_json::from_str(actual).unwrap_or_default();
	assert!(objects.is_empty(), "expected empty list, got {:?}", objects);
}

#[then(expr = "the file {string} exists and contains {string}")]
async fn file_exists_and_contains(_world: &mut OcflWorld, path: String, expected: String) {
	let content = std::fs::read_to_string(&path).expect("file does not exist");
	assert_eq!(content, expected);
}

#[then(expr = "the object {string} should exist in the repository")]
async fn object_should_exist_in_repo(world: &mut OcflWorld, object_id: String) {
	let repo_root = world._repo_dir.path();
	let object_path = repo_root.join(&object_id);
	assert!(object_path.exists(), "Object root does not exist: {}", object_path.display());
}

#[then(expr = "the inventory for object {string} should exist")]
async fn inventory_should_exist(world: &mut OcflWorld, object_id: String) {
	let repo_root = world._repo_dir.path();
	let inventory_path = repo_root.join(&object_id).join("inventory.json");
	assert!(inventory_path.exists(), "inventory.json does not exist: {}", inventory_path.display());
}

#[then(expr = "the inventory for object {string} should indicate version {string}")]
async fn inventory_should_indicate_version(world: &mut OcflWorld, object_id: String, version: String) {
	let repo_root = world._repo_dir.path();
	let inventory_path = repo_root.join(&object_id).join("inventory.json");
	let data = std::fs::read_to_string(&inventory_path).expect("Failed to read inventory.json");
	let v: Value = serde_json::from_str(&data).expect("Invalid JSON");
	assert_eq!(v["head"], version, "Inventory head does not match version");
}

#[then(expr = "the inventory for object {string} should list both versions {string} and {string}")]
async fn inventory_should_list_both_versions(world: &mut OcflWorld, object_id: String, v1: String, v2: String) {
	let repo_root = world._repo_dir.path();
	let inventory_path = repo_root.join(&object_id).join("inventory.json");
	let data = std::fs::read_to_string(&inventory_path).expect("Failed to read inventory.json");
	let v: Value = serde_json::from_str(&data).expect("Invalid JSON");
	let versions = v["versions"].as_object().expect("No versions object");
	assert!(versions.contains_key(&v1), "Inventory missing version {}", v1);
	assert!(versions.contains_key(&v2), "Inventory missing version {}", v2);
}

#[then(expr = "the inventory should be valid per OCFL 1.1 spec")]
async fn inventory_should_be_valid(world: &mut OcflWorld) {
	let data = world.last_response_text.as_ref().expect("No inventory response");
	let v: Value = serde_json::from_str(data).expect("Invalid JSON");
	assert!(v["id"].is_string(), "Missing id");
	assert!(v["type"].is_string(), "Missing type");
	assert!(v["digestAlgorithm"].is_string(), "Missing digestAlgorithm");
	assert!(v["head"].is_string(), "Missing head");
	assert!(v["manifest"].is_object(), "Missing manifest");
	assert!(v["versions"].is_object(), "Missing versions");
}

#[then(expr = "the inventory should list all versions and content digests")]
async fn inventory_should_list_versions_and_digests(world: &mut OcflWorld) {
	let data = world.last_response_text.as_ref().expect("No inventory response");
	let v: Value = serde_json::from_str(data).expect("Invalid JSON");
	let versions = v["versions"].as_object().expect("No versions object");
	let manifest = v["manifest"].as_object().expect("No manifest object");
	assert!(!versions.is_empty(), "No versions listed");
	assert!(!manifest.is_empty(), "No content digests listed");
}

#[then(expr = "the result should include {string} and {string}")]
async fn result_should_include_versions(world: &mut OcflWorld, v1: String, v2: String) {
	let data = world.last_response_text.as_ref().expect("No response");
	let v: Value = serde_json::from_str(data).expect("Invalid JSON");
	let versions = v["versions"].as_array().expect("No versions array");
	let v1_found = versions.iter().any(|x| x == &v1);
	let v2_found = versions.iter().any(|x| x == &v2);
	assert!(v1_found, "Version {} not found in result: {:?}", v1, versions);
	assert!(v2_found, "Version {} not found in result: {:?}", v2, versions);
}

#[then(expr = "the object {string} should not exist in the repository")]
async fn object_should_not_exist_in_repo(world: &mut OcflWorld, object_id: String) {
	let repo_root = world._repo_dir.path();
	let object_path = repo_root.join(&object_id);
	assert!(!object_path.exists(), "Object root still exists: {}", object_path.display());
}

#[then(expr = "the inventory for object {string} should not exist")]
async fn inventory_should_not_exist(world: &mut OcflWorld, object_id: String) {
	let repo_root = world._repo_dir.path();
	let inventory_path = repo_root.join(&object_id).join("inventory.json");
	assert!(!inventory_path.exists(), "inventory.json still exists: {}", inventory_path.display());
}

#[then(expr = "the inventory for object {string} should not list version {string}")]
async fn inventory_should_not_list_version(world: &mut OcflWorld, object_id: String, version: String) {
	let repo_root = world._repo_dir.path();
	let inventory_path = repo_root.join(&object_id).join("inventory.json");
	let data = std::fs::read_to_string(&inventory_path).expect("Failed to read inventory.json");
	let v: Value = serde_json::from_str(&data).expect("Invalid JSON");
	let versions = v["versions"].as_object().expect("No versions object");
	assert!(!versions.contains_key(&version), "Inventory still lists version {}", version);
}

#[then(expr = "the content for version {string} of object {string} should not exist")]
async fn content_for_version_should_not_exist(world: &mut OcflWorld, version: String, object_id: String) {
	let repo_root = world._repo_dir.path();
	let content_path = repo_root.join(&object_id).join(&version);
	assert!(!content_path.exists(), "Content for version {} still exists: {}", version, content_path.display());
}
