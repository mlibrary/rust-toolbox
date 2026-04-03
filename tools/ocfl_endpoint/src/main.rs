use axum::{routing::post, routing::get, Router, Json};
use ocfl_lib::{OcflRepo, OcflRepoImpl};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize)]
struct AddObjectRequest {
    object_id: String,
    src_path: String,
}

#[derive(Serialize)]
struct ListObjectsResponse {
    objects: Vec<String>,
}

async fn init_repo() -> Json<&'static str> {
    let repo = OcflRepoImpl::new("/tmp/ocfl");
    let _ = repo.init_repo("/tmp/ocfl");
    Json("ok")
}

async fn add_object(Json(req): Json<AddObjectRequest>) -> Json<&'static str> {
    let repo = OcflRepoImpl::new("/tmp/ocfl");
    let _ = repo.add_object(&req.object_id, &req.src_path);
    Json("ok")
}

async fn list_objects() -> Json<ListObjectsResponse> {
    let repo = OcflRepoImpl::new("/tmp/ocfl");
    let objects = repo.list_objects().unwrap_or_default();
    Json(ListObjectsResponse { objects })
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/init", post(init_repo))
        .route("/add", post(add_object))
        .route("/list", get(list_objects));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("OCFL endpoint listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
