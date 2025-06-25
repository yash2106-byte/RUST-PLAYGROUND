use std::fs;
use std::path::Path;

fn main() {
    println!("File Deduplicator - Step 1: Basic File Reading");
    
    // Test with a simple file first
    let test_file = "test.txt";
    
    // Create a test file if it doesn't exist
    if !Path::new(test_file).exists() {
        fs::write(test_file, "Hello, this is a test file for deduplication!")
            .expect("Failed to create test file");
        println!("Created test file: {}", test_file);
    }
    
    // Read file and show basic info
    match read_file_info(test_file) {
        Ok(info) => {
            println!("File: {}", info.name);
            println!("Size: {} bytes", info.size);
            println!("Content preview: {}", info.preview);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

struct FileInfo {
    name: String,
    size: u64,
    preview: String,
}

fn read_file_info(file_path: &str) -> Result<FileInfo, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(file_path)?;
    let content = fs::read_to_string(file_path)?;
    let preview = if content.len() > 50 {
        format!("{}...", &content[..50])
    } else {
        content
    };
    
    Ok(FileInfo {
        name: file_path.to_string(),
        size: metadata.len(),
        preview,
    })
}