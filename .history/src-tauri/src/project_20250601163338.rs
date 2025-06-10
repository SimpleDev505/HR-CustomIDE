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
pub fn createProject(name: &str) {}
