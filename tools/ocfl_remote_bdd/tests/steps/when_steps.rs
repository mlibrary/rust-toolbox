// ...existing code from ocfl_bdd/tests/steps/when_steps.rs...

use cucumber::when;
use std::process::Command;
use super::OcflWorld;

fn run_cli(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .args(["run", "-p", "ocfl_remote_cli", "--"])
        .args(args)
        .output()
        .expect("failed to run ocfl_remote_cli");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[when("I POST to \"/init\"")]
async fn post_init(world: &mut OcflWorld) {
    let out = run_cli(&["init"]);
    world.last_response_text = Some(out);
}

#[when(expr = "I add object {string} from src_path {string}")]
async fn post_add(world: &mut OcflWorld, object_id: String, src_path: String) {
    let out = run_cli(&["add", &object_id, &src_path]);
    world.last_response_text = Some(out);
}

#[when(expr = "I GET {string}")]
async fn get_with_path(world: &mut OcflWorld, path: String) {
    let out = if path.starts_with("/list") {
        run_cli(&["list"])
    } else if path.starts_with("/get") {
        let url = url::Url::parse(&format!("http://dummy{path}")).unwrap();
        let object_id = url.query_pairs().find(|(k,_)| k=="object_id").map(|(_,v)| v.to_string()).unwrap();
        let dest_path = url.query_pairs().find(|(k,_)| k=="dest_path").map(|(_,v)| v.to_string()).unwrap();
        run_cli(&["get", &object_id, &dest_path])
    } else if path.starts_with("/inventory") {
        let url = url::Url::parse(&format!("http://dummy{path}")).unwrap();
        let object_id = url.query_pairs().find(|(k,_)| k=="object_id").map(|(_,v)| v.to_string()).unwrap();
        run_cli(&["inventory", &object_id])
    } else if path.starts_with("/versions") {
        let url = url::Url::parse(&format!("http://dummy{path}")).unwrap();
        let object_id = url.query_pairs().find(|(k,_)| k=="object_id").map(|(_,v)| v.to_string()).unwrap();
        run_cli(&["versions", &object_id])
    } else {
        String::new()
    };
    world.last_response_text = Some(out);
}

#[when(expr = "I add an object with id {string} and content file {string}")]
async fn add_object_with_id_and_file(world: &mut OcflWorld, object_id: String, file: String) {
    std::fs::write(&file, b"version1").ok();
    let out = run_cli(&["add", &object_id, &file]);
    world.last_response_text = Some(out);
}

#[when(expr = "I add a new version to object {string} with content file {string}")]
async fn add_new_version_to_object(world: &mut OcflWorld, object_id: String, file: String) {
    std::fs::write(&file, b"version2").ok();
    let out = run_cli(&["add-version", &object_id, &file]);
    world.last_response_text = Some(out);
}

#[when(expr = "I retrieve the inventory for object {string}")]
async fn retrieve_inventory_for_object(world: &mut OcflWorld, object_id: String) {
    let out = run_cli(&["inventory", &object_id]);
    world.last_response_text = Some(out);
}

#[when(expr = "I list versions for object {string}")]
async fn i_list_versions_for_object(world: &mut OcflWorld, object_id: String) {
    let out = run_cli(&["versions", &object_id]);
    world.last_response_text = Some(out);
}

#[when(expr = "I delete object {string}")]
async fn i_delete_object(world: &mut OcflWorld, object_id: String) {
    let out = run_cli(&["delete-object", &object_id]);
    world.last_response_text = Some(out);
}

#[when(expr = "I delete version {string} of object {string}")]
async fn i_delete_version_of_object(world: &mut OcflWorld, version: String, object_id: String) {
    let out = run_cli(&["delete-version", &object_id, &version]);
    world.last_response_text = Some(out);
}
