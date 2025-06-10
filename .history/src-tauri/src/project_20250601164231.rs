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
use std::fs::{self};
use std::path::Path;
use tauri_plugin_dialog::DialogExt;
#[tauri::command]
pub async fn create_project(app: tauri::AppHandle) -> Result<ProjectMetadata, String> {
    // Open a directory picker dialog
    let dialog_result = app
        .dialog()
        .file()
        .add_filter("HREditor", ".hre")
        .await
        .map_err(|e| format!("Failed to open dialog: {}", e))?;

    let project_path = match dialog_result {
        Some(path) => path,
        None => return Err("No directory selected".to_string()),
    };

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
                let file_path = path.to_str().unwrap_or("").to_string();
                let language_type = "hre".to_string(); // Adjust based on actual language detection logic

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
        project_path: project_path.to_str().unwrap_or("").to_string(),
        files,
    })
}
