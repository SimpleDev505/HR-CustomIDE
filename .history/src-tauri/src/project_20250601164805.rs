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
use crate::models::{FileMetadata, ProjectMetadata};
use std::fs;
use std::path::Path;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{command, AppHandle};

#[command]
pub async fn create_project(app: AppHandle) -> Result<ProjectMetadata, String> {
    // Create a directory picker dialog
    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .set_title("Select Project Directory")
        .pick_folder(move |result| {
            let _ = tx.send(result);
        });

    // Wait for dialog result
    let dialog_result = rx.recv().map_err(|e| format!("Dialog error: {}", e))?;

    let project_path = match dialog_result {
        Some(path) => path,
        None => return Err("No directory selected".to_string()),
    };

    // Convert FilePath to PathBuf
    let project_path_buf = project_path
        .into_path_buf()
        .map_err(|e| format!("Failed to convert path: {}", e))?;

    // Get project name from the directory name
    let project_name = project_path_buf
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unnamed Project")
        .to_string();

    // Collect .hre files
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(&project_path_buf) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("hre") {
                let file_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .to_string();
                let file_path = path.to_str().ok_or("Invalid file path")?.to_string();
                let language_type = "hre".to_string(); // Adjust if needed

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
        project_path: project_path_buf
            .to_str()
            .ok_or("Invalid project path")?
            .to_string(),
        files,
    })
}
