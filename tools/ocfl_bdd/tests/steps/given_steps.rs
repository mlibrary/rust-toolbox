use cucumber::given;
use std::fs;

use super::OcflWorld;

#[given("the OCFL endpoint is running")]
async fn endpoint_is_running(_world: &mut OcflWorld) {
    // Server started in World::new(); this step documents intent in Gherkin.
}

#[given("the repository is initialized")]
async fn repo_is_initialized(world: &mut OcflWorld) {
    let url = format!("{}/init", world.base_url);
    world.client.post(&url).send().await.expect("POST /init failed");
}

#[given(expr = "a source file exists at {string}")]
async fn source_file_exists(_world: &mut OcflWorld, path: String) {
    std::fs::write(&path, b"hello").expect("failed to write source file");
}

#[given(expr = "object {string} has been added from {string}")]
async fn object_added(world: &mut OcflWorld, object_id: String, src_path: String) {
    std::fs::write(&src_path, b"hello").ok();
    let url = format!("{}/add", world.base_url);
    world
        .client
        .post(&url)
        .json(&serde_json::json!({ "object_id": object_id, "src_path": src_path }))
        .send()
        .await
        .expect("POST /add failed");
}
