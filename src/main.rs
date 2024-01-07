
use anyhow::{Context, Result};
use clap::Parser;

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

fn main() -> Result<()> {
    let args = Cli::parse();

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