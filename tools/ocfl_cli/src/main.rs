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
    AddVersion {
        object_id: String,
        src_path: String,
    },
    List,
    Get {
        object_id: String,
        dest_path: String,
    },
    Inventory {
        object_id: String,
    },
    Versions {
        object_id: String,
    },
    DeleteObject {
        object_id: String,
    },
    DeleteVersion {
        object_id: String,
        version: String,
    },
}

#[derive(Serialize)]
struct AddObjectRequest {
    object_id: String,
    src_path: String,
}

#[derive(Serialize)]
struct DeleteObjectRequest {
    object_id: String,
}

#[derive(Serialize)]
struct DeleteObjectVersionRequest {
    object_id: String,
    version: String,
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
        Commands::AddVersion { object_id, src_path } => {
            let req = AddObjectRequest { object_id, src_path };
            let resp = client.post("http://127.0.0.1:3000/add_version").json(&req).send().await?;
            println!("AddVersion: {}", resp.text().await?);
        }
        Commands::List => {
            let resp = client.get("http://127.0.0.1:3000/list").send().await?;
            let list: ListObjectsResponse = resp.json().await?;
            println!("Objects: {:?}", list.objects);
        }
        Commands::Get { object_id, dest_path } => {
            let resp = client
                .get("http://127.0.0.1:3000/get")
                .query(&[("object_id", &object_id), ("dest_path", &dest_path)])
                .send()
                .await?;
            println!("Get: {}", resp.text().await?);
        }
        Commands::Inventory { object_id } => {
            let resp = client
                .get("http://127.0.0.1:3000/inventory")
                .query(&[("object_id", &object_id)])
                .send()
                .await?;
            let inv: serde_json::Value = resp.json().await?;
            println!("Inventory: {}", serde_json::to_string_pretty(&inv)?);
        }
        Commands::Versions { object_id } => {
            let resp = client
                .get("http://127.0.0.1:3000/versions")
                .query(&[("object_id", &object_id)])
                .send()
                .await?;
            let versions: serde_json::Value = resp.json().await?;
            println!("Versions: {}", serde_json::to_string_pretty(&versions)?);
        }
        Commands::DeleteObject { object_id } => {
            let req = DeleteObjectRequest { object_id };
            let resp = client
                .post("http://127.0.0.1:3000/delete_object")
                .json(&req)
                .send()
                .await?;
            println!("DeleteObject: {}", resp.text().await?);
        }
        Commands::DeleteVersion { object_id, version } => {
            let req = DeleteObjectVersionRequest { object_id, version };
            let resp = client
                .post("http://127.0.0.1:3000/delete_version")
                .json(&req)
                .send()
                .await?;
            println!("DeleteVersion: {}", resp.text().await?);
        }
    }
    Ok(())
}
