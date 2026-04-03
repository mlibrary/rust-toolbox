use axum::{extract::State, routing::{get, post}, Json, Router};
use ocfl_lib::{OcflRepo, OcflRepoImpl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use serde_json;

#[derive(Clone)]
pub struct AppState {
    pub repo_root: Arc<String>,
}

#[derive(Deserialize)]
pub struct AddObjectRequest {
    pub object_id: String,
    pub src_path: String,
}

#[derive(Serialize)]
pub struct ListObjectsResponse {
    pub objects: Vec<String>,
}

pub fn build_router(repo_root: String) -> Router {
    let state = AppState { repo_root: Arc::new(repo_root) };
    Router::new()
        .route("/init", post(init_repo))
        .route("/add", post(add_object))
        .route("/list", get(list_objects))
        .route("/get", get(get_object_endpoint))
        .route("/add_version", post(add_object_version))
        .route("/inventory", get(get_inventory_endpoint))
        .with_state(state)
}

async fn init_repo(State(state): State<AppState>) -> Json<&'static str> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    match repo.init_repo(state.repo_root.as_str()) {
        Ok(_) => Json("ok"),
        Err(_) => Json("error"),
    }
}

async fn add_object(
    State(state): State<AppState>,
    Json(req): Json<AddObjectRequest>,
) -> Json<&'static str> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    match repo.add_object(&req.object_id, &req.src_path) {
        Ok(_) => Json("ok"),
        Err(_) => Json("error"),
    }
}

async fn add_object_version(
    State(state): State<AppState>,
    Json(req): Json<AddObjectRequest>,
) -> Json<&'static str> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    match repo.add_object_version(&req.object_id, &req.src_path) {
        Ok(_) => Json("ok"),
        Err(_) => Json("error"),
    }
}

async fn list_objects(State(state): State<AppState>) -> Json<ListObjectsResponse> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    let objects = repo.list_objects().unwrap_or_default();
    Json(ListObjectsResponse { objects })
}

use axum::extract::Query;

async fn get_object_endpoint(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<&'static str> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    let object_id = match params.get("object_id") {
        Some(v) => v,
        None => return Json("error"),
    };
    let dest_path = match params.get("dest_path") {
        Some(v) => v,
        None => return Json("error"),
    };
    match repo.get_object(object_id, dest_path) {
        Ok(_) => Json("ok"),
        Err(_) => Json("error"),
    }
}

async fn get_inventory_endpoint(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    let object_id = match params.get("object_id") {
        Some(v) => v,
        None => return Json(serde_json::json!({"error": "missing object_id"})),
    };
    match repo.get_inventory(object_id) {
        Ok(inv) => Json(inv),
        Err(_) => Json(serde_json::json!({"error": "not found"})),
    }
}
