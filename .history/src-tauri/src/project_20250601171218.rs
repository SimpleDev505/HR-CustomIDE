use serde::{Deserialize, Serialize};
use serde_json::json;
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
    // Step 1: Ask for directory
    let base_dir = FileDialogBuilder::new()
        .set_title("Select a folder to create the project")
        .pick_folder()
        .ok_or("No folder selected")?;

    // Step 2: Ask for project name
    let project_name = ask_for_project_name().ok_or("Project name not provided")?;

    // Step 3: Create project folder
    let project_dir = base_dir.join(&project_name);
    create_dir_all(&project_dir).map_err(|e| e.to_string())?;

    // Step 4: Create welcome.txt
    let welcome_path = project_dir.join("welcome.txt");
    let mut welcome_file = File::create(&welcome_path).map_err(|e| e.to_string())?;
    welcome_file
        .write_all(b"Welcome to your new project!")
        .map_err(|e| e.to_string())?;

    // Step 5: Prepare metadata
    let metadata = HREMetadata {
        projectName: project_name.clone(),
        projectPath: project_dir.to_string_lossy().into(),
        files: vec![HREFile {
            filePath: welcome_path.to_string_lossy().into(),
            fileName: "welcome.txt".into(),
            languageType: "Text".into(),
        }],
    };

    // Step 6: Save .hre file
    let hre_path = project_dir.join(format!("{}.hre", project_name));
    let json_data = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    fs::write_string(hre_path, json_data).map_err(|e| e.to_string())?;

    Ok(metadata)
}
fn ask_for_project_name() -> Option<String> {
    let input = blocking::ask(Some("Project Name"), "Enter your project name:");
    input
}
