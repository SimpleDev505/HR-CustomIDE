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
use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tauri::command;

#[command]
pub fn create_project() -> Result<ProjectMetadata, String> {
    // Get the user's home directory
    let home_dir =
        std::env::var("HOME").map_err(|e| format!("Failed to get HOME directory: {}", e))?;
    let base_path = Path::new(&home_dir);

    // Create a unique project folder name (e.g., new_project_20250601_1658)
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let project_name = format!("new_project_{}", timestamp);
    let project_path = base_path.join(&project_name);

    // Create the project directory
    fs::create_dir(&project_path).map_err(|e| {
        format!(
            "Failed to create directory {}: {}",
            project_path.display(),
            e
        )
    })?;

    // Create a .hre file inside the project directory
    let hre_file_name = "example.hre";
    let hre_file_path = project_path.join(hre_file_name);
    let mut file = File::create(&hre_file_path)
        .map_err(|e| format!("Failed to create {}: {}", hre_file_path.display(), e))?;
    // Write some placeholder content to the .hre file
    file.write_all(b"// Sample .hre file content")
        .map_err(|e| format!("Failed to write to {}: {}", hre_file_path.display(), e))?;

    // Build FileMetadata for the .hre file
    let file_metadata = FileMetadata {
        file_path: hre_file_path
            .to_str()
            .ok_or("Invalid file path")?
            .to_string(),
        language_type: "hre".to_string(),
        file_name: hre_file_name.to_string(),
    };

    // Return ProjectMetadata
    Ok(ProjectMetadata {
        project_name,
        project_path: project_path
            .to_str()
            .ok_or("Invalid project path")?
            .to_string(),
        files: vec![file_metadata],
    })
}
