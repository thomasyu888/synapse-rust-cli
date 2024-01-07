
use std::env;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Error;

// Three / will add to the cli helper
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

// fn read_file(file_path: std::path::PathBuf, pattern: String) {
//     let content = std::fs::read_to_string(file_path).expect("could not read file");
//     for line in content.lines() {
//         if line.contains(pattern) {
//             println!("{}", line);
//         }
//     }
// }

#[tokio::main]
async fn test_execution() -> Result<(), Error> {
    let token = env::var("SYNAPSE_AUTH_TOKEN").unwrap();
    // let bearer = "Bearer your_token_here";
    let client = reqwest::Client::new();
    let res = client.get("https://repo-prod.prod.sagebase.org/repo/v1/userProfile")
        .header("Accept", "application/json")
        .header("User-Agent", "Rust")
        .bearer_auth(token)
        .send()
        .await?;

    let text = res.text().await?;
    println!("{}", text);

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    test_execution()?;
    // let content = std::fs::read_to_string(&args.path)?;
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}