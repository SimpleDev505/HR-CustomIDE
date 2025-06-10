use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_dialog::FileDialogBuilder;

#[derive(Serialize, Deserialize)]
struct HREFile {
    filePath: String,
    fileName: String,
    languageType: String,
}

#[derive(Serialize, Deserialize)]
struct HREMetadata {
    projectName: String,
    projectPath: String,
    files: Vec<HREFile>,
}

#[tauri::command]
fn create_project() -> Result<HREMetadata, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    // Ask for folder
    let base_path = FileDialogBuilder::new()
        .set_title("Select folder to create the project")
        .pick_folder()
        .ok_or("No folder selected")?;

    // Hard fail if path is bad
    if !base_path.exists() {
        return Err("Selected directory does not exist.".to_string());
    }

    // We assume frontend gives us a temp file `tauri_project_name.txt` containing name
    let name_file = base_path.join("tauri_project_name.txt");
    if !name_file.exists() {
        return Err(
            "Missing temporary project name file. Please prompt from frontend.".to_string(),
        );
    }

    let project_name = std::fs::read_to_string(&name_file)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();

    std::fs::remove_file(&name_file).ok(); // Clean up temp file

    let project_dir = base_path.join(&project_name);
    create_dir_all(&project_dir).map_err(|e| e.to_string())?;

    // Create welcome.txt
    let welcome_path = project_dir.join("welcome.txt");
    let mut welcome_file = File::create(&welcome_path).map_err(|e| e.to_string())?;
    welcome_file
        .write_all(b"Welcome to your new project!")
        .map_err(|e| e.to_string())?;

    // Metadata
    let metadata = HREMetadata {
        projectName: project_name.clone(),
        projectPath: project_dir.to_string_lossy().to_string(),
        files: vec![HREFile {
            filePath: welcome_path.to_string_lossy().to_string(),
            fileName: "welcome.txt".into(),
            languageType: "Text".into(),
        }],
    };

    // Save .hre
    let hre_path = project_dir.join(format!("{}.hre", project_name));
    let json_data = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    std::fs::write(hre_path, json_data).map_err(|e| e.to_string())?;

    Ok(metadata)
}
