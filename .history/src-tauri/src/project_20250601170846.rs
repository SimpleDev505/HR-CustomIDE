use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Manager;

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
    let project = HREMetadata {
        projectName: "Demo Project".into(),
        projectPath: "/home/user/projects/DemoProject".into(),
        files: vec![
            HREFile {
                filePath: "/home/user/projects/DemoProject/main.rs".into(),
                fileName: "main.rs".into(),
                languageType: "Rust".into(),
            },
            HREFile {
                filePath: "/home/user/projects/DemoProject/App.svelte".into(),
                fileName: "App.svelte".into(),
                languageType: "Svelte".into(),
            },
        ],
    };

    Ok(project)
}
