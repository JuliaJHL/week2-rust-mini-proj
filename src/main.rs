use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// the pattern to look for
    pattern: String,
    /// the path to the file to read
    path: std::path::PathBuf,
}

// fn main() {
//     let args = Cli::parse();
//     let content = std::fs::read_to_string(&args.path).expect("Could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    grrs_hj::find_matches(&content, &args.pattern);

    Ok(())
}
