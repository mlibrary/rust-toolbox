use axum::{extract::State, routing::{get, post}, Json, Router};
use ocfl_lib::{OcflRepo, OcflRepoImpl};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
        .with_state(state)
}

async fn init_repo(State(state): State<AppState>) -> Json<&'static str> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    let _ = repo.init_repo(state.repo_root.as_str());
    Json("ok")
}

async fn add_object(
    State(state): State<AppState>,
    Json(req): Json<AddObjectRequest>,
) -> Json<&'static str> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    let _ = repo.add_object(&req.object_id, &req.src_path);
    Json("ok")
}

async fn list_objects(State(state): State<AppState>) -> Json<ListObjectsResponse> {
    let repo = OcflRepoImpl::new(state.repo_root.as_str());
    let objects = repo.list_objects().unwrap_or_default();
    Json(ListObjectsResponse { objects })
}
