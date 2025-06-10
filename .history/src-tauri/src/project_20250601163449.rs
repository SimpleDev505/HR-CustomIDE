use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for potential future use
struct HreProjectFile {
    file_name: String,
    file_path: PathBuf,
    file_type: String, // e.g., "text", "python", "image", etc.
}

// Defines the structure for your .hre project metadata file
#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
struct HreProjectMetadata {
    project_name: String,
    project_path: PathBuf,
    files: Vec<HreProjectFile>, // An array of files
}

// Define the structure for the return value from the Rust command to Svelte
#[derive(Debug, Serialize, Deserialize)]
struct ProjectCreatedResponse {
    success: bool,
    path: Option<String>, // Path to the new project directory
    error: Option<String>,
    // Optionally, return the full metadata to the frontend
    // metadata: Option<HreProjectMetadata>,
}

#[tauri::command]
async fn create_project(app_handle: tauri::AppHandle) -> Result<ProjectCreatedResponse, String> {
    println!("Received create_project command");

    // 1. Open a save file dialog to get the desired project name and location
    let dialog_result = app_handle
        .dialog() // Access the dialog API via app_handle
        .file() // File dialog specific methods
        .blocking() // Use blocking version for simplicity in commands
        .save_file(None); // `None` for default starting directory. You could set a PathBuf for a default path.

    let (save_path, project_name_from_dialog) = match dialog_result {
        Some(path_buf) => {
            // Extract the suggested file name (e.g., "MyProject" from "MyProject.txt")
            let suggested_file_name = path_buf
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "NewProject".to_string());
            (path_buf, suggested_file_name)
        }
        None => {
            // User cancelled the dialog
            println!("Project creation cancelled by user.");
            return Ok(ProjectCreatedResponse {
                success: false,
                path: None,
                error: Some("Project creation cancelled by user.".to_string()),
            });
        }
    };

    // The project folder will be created in the chosen directory, named after the suggested file name
    let project_folder_path = save_path
        .parent()
        .ok_or_else(|| "Invalid project save path: no parent directory".to_string())?
        .join(&project_name_from_dialog);

    println!(
        "Attempting to create project folder at: {:?}",
        project_folder_path
    );

    // 2. Create the project folder
    if let Err(e) = fs::create_dir_all(&project_folder_path) {
        eprintln!("Failed to create project folder: {}", e);
        return Err(format!("Failed to create project folder: {}", e));
    }

    // 3. Create the first (default text) file inside the new project folder
    let first_file_name = format!("{}.txt", project_name_from_dialog); // e.g., "MyProject.txt"
    let first_file_path = project_folder_path.join(&first_file_name);

    let mut first_file = match fs::File::create(&first_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create first text file: {}", e);
            return Err(format!("Failed to create first text file: {}", e));
        }
    };
    if let Err(e) = first_file.write_all(b"// This is your first file in the project.\n\n") {
        eprintln!("Failed to write to first text file: {}", e);
        return Err(format!("Failed to write to first text file: {}", e));
    }

    // 4. Populate the `files` array for the .hre metadata
    let mut project_files: Vec<HreProjectFile> = Vec::new();
    project_files.push(HreProjectFile {
        file_name: first_file_name,
        file_path: first_file_path,
        file_type: "text".to_string(), // Initial file type
    });

    // 5. Create and write to the .hre metadata file inside the project folder
    let hre_metadata_file_name = format!(".{}.hre", project_name_from_dialog); // Hidden file, e.g., ".MyProject.hre"
    let hre_metadata_file_path = project_folder_path.join(&hre_metadata_file_name);

    let metadata = HreProjectMetadata {
        project_name: project_name_from_dialog.clone(),
        project_path: project_folder_path.clone(),
        files: project_files, // The array of files
    };

    let metadata_json = match serde_json::to_string_pretty(&metadata) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to serialize metadata: {}", e);
            return Err(format!("Failed to serialize metadata: {}", e));
        }
    };

    let mut hre_file = match fs::File::create(&hre_metadata_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create .hre metadata file: {}", e);
            return Err(format!("Failed to create .hre metadata file: {}", e));
        }
    };
    if let Err(e) = hre_file.write_all(metadata_json.as_bytes()) {
        eprintln!("Failed to write to .hre metadata file: {}", e);
        return Err(format!("Failed to write to .hre metadata file: {}", e));
    }

    println!(
        "Project '{}' created successfully at {:?}",
        project_name_from_dialog, project_folder_path
    );

    // Return the path to the newly created project folder (and optionally metadata)
    Ok(ProjectCreatedResponse {
        success: true,
        path: Some(project_folder_path.to_string_lossy().into_owned()),
        error: None,
        // metadata: Some(metadata), // Uncomment this if you want to return full metadata
    })
}
