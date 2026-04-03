pub mod given_steps;
pub mod when_steps;
pub mod then_steps;

use cucumber::World;
use reqwest::Client;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct OcflWorld {
    pub base_url: String,
    pub client: Client,
    pub last_response_text: Option<String>,
    pub last_object_list: Vec<String>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    _server_handle: JoinHandle<()>,
    _repo_dir: TempDir,
}

impl OcflWorld {
    async fn new() -> Self {
        let repo_dir = tempfile::tempdir().expect("failed to create tempdir");
        let repo_root = repo_dir.path().to_string_lossy().to_string();

        let app = ocfl_endpoint::build_router(repo_root);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind listener");
        let port = listener.local_addr().expect("no local addr").port();
        let base_url = format!("http://127.0.0.1:{port}");

        let (tx, rx) = oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move { rx.await.ok(); })
                .await
                .ok();
        });

        Self {
            base_url,
            client: Client::new(),
            last_response_text: None,
            last_object_list: Vec::new(),
            shutdown_tx: Some(tx),
            _server_handle: handle,
            _repo_dir: repo_dir,
        }
    }
}

impl Drop for OcflWorld {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}
