use cucumber::when;
use serde::Deserialize;

use super::OcflWorld;

#[when("I POST to \"/init\"")]
async fn post_init(world: &mut OcflWorld) {
    let url = format!("{}/init", world.base_url);
    let resp = world.client.post(&url).send().await.expect("POST /init failed");
    world.last_response_text = Some(resp.text().await.expect("no body"));
}

#[when(expr = "I add object {string} from src_path {string}")]
async fn post_add(world: &mut OcflWorld, object_id: String, src_path: String) {
    let url = format!("{}/add", world.base_url);
    let resp = world
        .client
        .post(&url)
        .json(&serde_json::json!({ "object_id": object_id, "src_path": src_path }))
        .send()
        .await
        .expect("POST /add failed");
    world.last_response_text = Some(resp.text().await.expect("no body"));
}

#[when(expr = "I GET {string}")]
async fn get_with_path(world: &mut OcflWorld, path: String) {
    let url = format!("{}{}", world.base_url, path);
    let resp = world.client.get(&url).send().await.expect("GET failed");
    if path.starts_with("/list") {
        let content_type = resp.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
        if content_type.contains("application/json") {
            #[derive(Deserialize)]
            struct ListObjectsResponse {
                objects: Vec<String>,
            }
            let body: ListObjectsResponse = resp.json().await.expect("invalid JSON from /list");
            world.last_object_list = body.objects;
            world.last_response_text = None;
            return;
        }
    }
    world.last_response_text = Some(resp.text().await.expect("no body"));
}

#[when(expr = "I add an object with id {string} and content file {string}")]
async fn add_object_with_id_and_file(world: &mut OcflWorld, object_id: String, file: String) {
    std::fs::write(&file, b"version1").ok();
    let url = format!("{}/add", world.base_url);
    let resp = world
        .client
        .post(&url)
        .json(&serde_json::json!({ "object_id": object_id, "src_path": file }))
        .send()
        .await
        .expect("POST /add failed");
    world.last_response_text = Some(resp.text().await.expect("no body"));
}

#[when(expr = "I add a new version to object {string} with content file {string}")]
async fn add_new_version_to_object(world: &mut OcflWorld, object_id: String, file: String) {
    // Write new content to the file
    std::fs::write(&file, b"version2").ok();
    // Call the endpoint or directly invoke the library (if available)
    // For now, call the add_object_version method via a custom endpoint or CLI
    // Here, we assume a custom endpoint /add_version exists (to be implemented)
    let url = format!("{}/add_version", world.base_url);
    let resp = world
        .client
        .post(&url)
        .json(&serde_json::json!({ "object_id": object_id, "src_path": file }))
        .send()
        .await
        .expect("POST /add_version failed");
    world.last_response_text = Some(resp.text().await.expect("no body"));
}

#[when(expr = "I retrieve the inventory for object {string}")]
async fn retrieve_inventory_for_object(world: &mut OcflWorld, object_id: String) {
    let url = format!("{}/inventory?object_id={}", world.base_url, object_id);
    let resp = world.client.get(&url).send().await.expect("GET /inventory failed");
    world.last_response_text = Some(resp.text().await.expect("no body"));
}

#[when(expr = "I list versions for object {string}")]
async fn i_list_versions_for_object(world: &mut OcflWorld, object_id: String) {
    let url = format!("{}/versions?object_id={}", world.base_url, object_id);
    let resp = world.client.get(&url).send().await.expect("GET /versions failed");
    let text = resp.text().await.expect("no response body");
    world.last_response_text = Some(text);
}

#[when(expr = "I delete object {string}")]
async fn i_delete_object(world: &mut OcflWorld, object_id: String) {
    // TODO: Call the delete object endpoint or library method
    unimplemented!("delete object");
}

#[when(expr = "I delete version {string} of object {string}")]
async fn i_delete_version_of_object(world: &mut OcflWorld, version: String, object_id: String) {
    // TODO: Call the delete version endpoint or library method
    unimplemented!("delete version");
}
