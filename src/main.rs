fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current working directory
    let current_dir = std::env::current_dir()?;
    
    // Read the directory contents
    let entries = std::fs::read_dir(&current_dir)?;
    
    // Get the first command line argument if available
    let args: Vec<String> = std::env::args().collect();
    let replacement_char = match args.get(1) {
        Some(arg) if arg == "snake" => "_",
        Some(arg) if arg == "train" => "-",
        _ => {
            eprintln!("Usage: {} [snake|train]", args[0]);
            return Ok(());
        }
    };
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Check if the entry is a file
        if path.is_file() {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            
            // Check if the filename contains spaces
            if filename.contains(' ') {
                let new_filename = filename.replace(" ", replacement_char);
                let new_path = current_dir.join(std::path::Path::new(&new_filename));
                
                // Rename the file
                std::fs::rename(&path, &new_path)?;
            }
        }
    }
    
    Ok(())
}

