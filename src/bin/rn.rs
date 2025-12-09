use clap::Parser;
use std::fs;
use std::path::Path;

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
    // Extract the filename and directory from the path
    let new_name_path = Path::new(new_name);
    let new_filename = new_name_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("Invalid filename: '{}'", new_name))?;

    // Determine which directory to search in
    let search_dir = if let Some(parent) = new_name_path.parent() {
        // If a parent path is specified, use it
        if parent.as_os_str().is_empty() {
            // Empty parent means current directory (e.g., "./file" or "file")
            std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
        } else {
            // Use the specified directory
            parent.to_path_buf()
        }
    } else {
        // No parent, use current directory
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?
    };

    // Canonicalize the search directory to handle relative paths
    let search_dir = search_dir
        .canonicalize()
        .map_err(|e| format!("Invalid directory '{}': {}", search_dir.display(), e))?;

    // Check if target already exists
    let target_path = search_dir.join(new_filename);
    if target_path.exists() && !force {
        return Err(format!(
            "Target '{}' already exists. Use --force to overwrite.",
            new_filename
        ));
    }

    // Read directory and find matching files
    let entries =
        fs::read_dir(&search_dir).map_err(|e| format!("Failed to read directory: {}", e))?;

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
        if filename == new_filename {
            continue;
        }

        // Check if this file matches expansion or extension change pattern (either direction)
        if snipren::matches_expansion(filename, new_filename)
            || snipren::matches_expansion(new_filename, filename)
            || snipren::matches_extension_change(filename, new_filename)
            || snipren::matches_extension_change(new_filename, filename)
        {
            candidates.push(filename.to_string());
        }
    }

    // Handle based on number of candidates
    match candidates.len() {
        0 => Err(format!("No matching files found for '{}'", new_filename)),
        1 => {
            let old_name = &candidates[0];
            let old_path = search_dir.join(old_name);

            // Perform the rename
            fs::rename(&old_path, &target_path).map_err(|e| format!("Failed to rename: {}", e))?;

            Ok(format!("{} → {}", old_name, new_filename))
        }
        _ => {
            let mut msg = format!("Multiple candidates found for '{}':\n", new_filename);
            for candidate in &candidates {
                msg.push_str(&format!("  {}\n", candidate));
            }
            msg.push_str("\nCannot proceed - ambiguous match.");
            Err(msg)
        }
    }
}
