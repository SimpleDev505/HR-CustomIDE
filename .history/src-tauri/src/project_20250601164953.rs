use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_path: String,
    pub language_type: String,
    pub file_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub project_name: String,
    pub project_path: String,
    pub files: Vec<FileMetadata>,
}
use std::fs;
use std::path::Path;
use tauri::{command, AppHandle};

#[command]
pub fn create_project(directory_path: String) -> Result<ProjectMetadata, String> {
    // Convert the input string to a Path
    let project_path = Path::new(&directory_path);
    if !project_path.is_dir() {
        return Err(format!("Invalid directory: {}", directory_path));
    }

    // Get project name from the directory name
    let project_name = project_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unnamed Project")
        .to_string();

    // Collect .hre files
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(&project_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("hre") {
                let file_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .to_string();
                let file_path = path.to_str().ok_or("Invalid file path")?.to_string();
                let language_type = "hre".to_string(); // Adjust if language detection is needed

                files.push(FileMetadata {
                    file_path,
                    language_type,
                    file_name,
                });
            }
        }
    }

    Ok(ProjectMetadata {
        project_name,
        project_path: directory_path,
        files,
    })
}
