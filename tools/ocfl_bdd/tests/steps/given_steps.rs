use cucumber::given;

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

#[given(expr = "the file {string} does not exist")]
async fn file_does_not_exist(_world: &mut OcflWorld, path: String) {
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir_all(&path);
}

#[given(expr = "an empty OCFL repository")]
async fn empty_ocfl_repo(world: &mut OcflWorld) {
    let url = format!("{}/init", world.base_url);
    world.client.post(&url).send().await.expect("POST /init failed");
}

#[given(expr = "an OCFL repository with object {string} at version {string}")]
async fn repo_with_object_at_version(world: &mut OcflWorld, object_id: String, version: String) {
    // Always (re-)initialize the repo
    let url = format!("{}/init", world.base_url);
    world.client.post(&url).send().await.expect("POST /init failed");

    // Add v1
    let file_v1 = format!("/tmp/ocfl_bdd_{}_v1.txt", object_id);
    std::fs::write(&file_v1, b"version1").expect("failed to write v1 file");
    let url_add = format!("{}/add", world.base_url);
    world.client.post(&url_add)
        .json(&serde_json::json!({ "object_id": object_id, "src_path": file_v1 }))
        .send().await.expect("POST /add failed");

    if version == "v2" {
        // Add v2
        let file_v2 = format!("/tmp/ocfl_bdd_{}_v2.txt", object_id);
        std::fs::write(&file_v2, b"version2").expect("failed to write v2 file");
        let url_add_version = format!("{}/add_version", world.base_url);
        world.client.post(&url_add_version)
            .json(&serde_json::json!({ "object_id": object_id, "src_path": file_v2 }))
            .send().await.expect("POST /add_version failed");
    }
}
