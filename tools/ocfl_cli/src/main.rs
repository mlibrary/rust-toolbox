use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "ocfl-cli")]
#[command(about = "CLI for OCFL endpoint", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        object_id: String,
        src_path: String,
    },
    List,
}

#[derive(Serialize)]
struct AddObjectRequest {
    object_id: String,
    src_path: String,
}

#[derive(Deserialize)]
struct ListObjectsResponse {
    objects: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();
    match cli.command {
        Commands::Init => {
            let resp = client.post("http://127.0.0.1:3000/init").send().await?;
            println!("Init: {}", resp.text().await?);
        }
        Commands::Add { object_id, src_path } => {
            let req = AddObjectRequest { object_id, src_path };
            let resp = client.post("http://127.0.0.1:3000/add").json(&req).send().await?;
            println!("Add: {}", resp.text().await?);
        }
        Commands::List => {
            let resp = client.get("http://127.0.0.1:3000/list").send().await?;
            let list: ListObjectsResponse = resp.json().await?;
            println!("Objects: {:?}", list.objects);
        }
    }
    Ok(())
}
