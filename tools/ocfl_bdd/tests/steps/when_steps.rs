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
    world.last_response_text = Some(resp.text().await.expect("no body"));
}
