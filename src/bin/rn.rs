use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "rn")]
#[command(about = "A fast, safe, intent-aware rename utility", long_about = None)]
struct Args {
    /// The new filename to rename to
    new_name: String,

    /// Force rename even if target exists
    #[arg(short, long)]
    force: bool,
}

fn main() {
    let args = Args::parse();

    match rename_file(&args.new_name, args.force) {
        Ok(msg) => println!("{}", msg),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn rename_file(new_name: &str, force: bool) -> Result<String, String> {
    // Get current directory
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    // Check if target already exists
    let target_path = current_dir.join(new_name);
    if target_path.exists() && !force {
        return Err(format!(
            "Target '{}' already exists. Use --force to overwrite.",
            new_name
        ));
    }

    // Read directory and find matching files
    let entries =
        fs::read_dir(&current_dir).map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut candidates = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        // Skip directories, only consider files
        if !path.is_file() {
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?;

        // Skip the target name itself if it exists
        if filename == new_name {
            continue;
        }

        // Check if this file matches expansion pattern
        if snipren::matches_expansion(filename, new_name) {
            candidates.push(filename.to_string());
        }
        // Also check reduction (reverse: old expands to new becomes new reduces from old)
        else if snipren::matches_expansion(new_name, filename) {
            candidates.push(filename.to_string());
        }
    }

    // Handle based on number of candidates
    match candidates.len() {
        0 => Err(format!("No matching files found for '{}'", new_name)),
        1 => {
            let old_name = &candidates[0];
            let old_path = current_dir.join(old_name);

            // Perform the rename
            fs::rename(&old_path, &target_path).map_err(|e| format!("Failed to rename: {}", e))?;

            Ok(format!("{} → {}", old_name, new_name))
        }
        _ => {
            let mut msg = format!("Multiple candidates found for '{}':\n", new_name);
            for candidate in &candidates {
                msg.push_str(&format!("  {}\n", candidate));
            }
            msg.push_str("\nCannot proceed - ambiguous match.");
            Err(msg)
        }
    }
}
