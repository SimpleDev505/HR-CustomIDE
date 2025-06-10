use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize)]
struct HreProjectFile {
    file_name: String,
    file_path: PathBuf,
    file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HreProjectMetadata {
    project_name: String,
    project_path: PathBuf,
    files: Vec<HreProjectFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectCreatedResponse {
    success: bool,
    path: Option<String>,
    error: Option<String>,
}
#[tauri::command]
pub fn createProject(name: &str) {}
