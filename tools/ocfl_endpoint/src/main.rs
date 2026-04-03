use anyhow::Result;
use ocfl_endpoint::build_router;

#[tokio::main]
async fn main() -> Result<()> {
    let app = build_router("/tmp/ocfl".to_string());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("OCFL endpoint listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
