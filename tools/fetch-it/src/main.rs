use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(about = "Fetch a URL and print the HTTP status code")]
struct Args {
    /// URL to fetch
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let response = reqwest::get(&args.url).await?;
    println!("{}", response.status());
    Ok(())
}
