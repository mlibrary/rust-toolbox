use anyhow::Result;
use clap::{Parser, Subcommand};
use ocfl_lib::{OcflRepo, OcflRepoImpl};

#[derive(Parser)]
#[command(name = "ocfl-local-cli")]
#[command(about = "CLI for local OCFL operations", long_about = None)]
struct Cli {
    /// Path to the OCFL repository root
    #[arg(long, short, default_value = ".")]
    repo: String,
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    let repo = OcflRepoImpl::new(&cli.repo);
    match cli.command {
        Commands::Init => {
            repo.init_repo(&cli.repo)?;
            println!("Init: ok");
        }
        Commands::Add { object_id, src_path } => {
            repo.add_object(&object_id, &src_path)?;
            println!("Add: ok");
        }
        Commands::AddVersion { object_id, src_path } => {
            repo.add_object_version(&object_id, &src_path)?;
            println!("AddVersion: ok");
        }
        Commands::List => {
            let objects = repo.list_objects()?;
            println!("Objects: {:?}", objects);
        }
        Commands::Get { object_id, dest_path } => {
            repo.get_object(&object_id, &dest_path)?;
            println!("Get: ok");
        }
        Commands::Inventory { object_id } => {
            let inv = repo.get_inventory(&object_id)?;
            println!("Inventory: {}", serde_json::to_string_pretty(&inv)?);
        }
        Commands::Versions { object_id } => {
            let versions = repo.list_versions(&object_id)?;
            println!("Versions: {:?}", versions);
        }
        Commands::DeleteObject { object_id } => {
            repo.delete_object(&object_id)?;
            println!("DeleteObject: ok");
        }
        Commands::DeleteVersion { object_id, version } => {
            repo.delete_object_version(&object_id, &version)?;
            println!("DeleteVersion: ok");
        }
    }
    Ok(())
}
