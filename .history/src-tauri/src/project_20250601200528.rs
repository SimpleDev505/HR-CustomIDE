use lazy_static::lazy_static;
use rfd::FileDialog;
use serde::Deserialize;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
#[derive(Clone, PartialEq, Serialize, Deserialize)]
// File node structure
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub language: String, // e.g., "python", "text"
}

// App state structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub project_path: Option<PathBuf>,
    pub file_tree: Vec<FileNode>,
}

// Global state
lazy_static! {
    pub static ref APP_STATE: Mutex<AppState> = Mutex::new(AppState {
        project_path: None,
        file_tree: vec![],
    });
}
lazy_static! {
    pub static ref PROJECT_LOADED: Mutex<bool> = Mutex::new(false);
}

// Save app state to .hre file
pub fn save_state(state: &AppState, project_path: &PathBuf) {
    let json = serde_json::to_string(state).unwrap();
    let file_path = project_path.join("project.hre");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{}", json).unwrap();
}

// Load app state from .hre file
pub fn load_state(project_path: &PathBuf) -> Option<AppState> {
    let file_path = project_path.join("project.hre");
    if file_path.exists() {
        let mut file = File::open(&file_path).ok()?;
        let mut json = String::new();
        file.read_to_string(&mut json).ok()?;
        serde_json::from_str(&json).ok()
    } else {
        None
    }
}
#[tauri::command]
pub fn create_project() {
    // Step 1: Prompt for parent directory using file dialog
    let base_path = FileDialog::new()
        .set_directory(std::env::var("HOME").unwrap_or_default())
        .pick_folder();

    let base_path = match base_path {
        Some(path) => path,
        None => {
            println!("Project creation cancelled.");
            return;
        }
    };

    // Step 2: Prompt for project name (simulated, extend with rfd::MessageDialog if needed)
    let project_name = "UntitledProject"; // Replace with dialog if desired

    // Step 3: Create project folder
    let project_path = base_path.join(project_name);
    fs::create_dir_all(&project_path).unwrap();

    // Step 4: Create welcome.txt with premade info
    let welcome_file = project_path.join("welcome.txt");
    let mut file = File::create(&welcome_file).unwrap();
    writeln!(
        file,
        "Welcome to your new project!\n\nSteps to create a project:\n1. Click 'New Project' in the menu.\n2. Choose a project name and location.\n3. Start adding files using 'New File'.\n4. Edit files in the editor and run them with the 'Run' button."
    ).unwrap();

    // Step 5: Update global AppState
    let mut state = APP_STATE.lock().unwrap();
    state.project_path = Some(project_path.clone());
    state.file_tree = vec![
        FileNode {
            name: project_name.to_string(),
            path: project_path.clone(),
            is_dir: true,
            language: "directory".to_string(),
        },
        FileNode {
            name: "welcome.txt".to_string(),
            path: welcome_file,
            is_dir: false,
            language: "text".to_string(),
        },
    ];

    // Step 6: Save project metadata to project.hre
    save_state(&state, &project_path);

    println!("Created project '{}' at {:?}", project_name, project_path);
}

pub fn open_project() {
    // Step 1: Prompt for project folder
    let project_path = FileDialog::new()
        .set_directory(std::env::var("HOME").unwrap_or_default())
        .pick_folder();

    let project_path = match project_path {
        Some(path) => path,
        None => {
            println!("Project opening cancelled.");
            return;
        }
    };
    let mut loaded_guard = PROJECT_LOADED.lock().unwrap();
    // Step 2: Try to load from project.hre
    let mut state = APP_STATE.lock().unwrap();
    if let Some(loaded_state) = load_state(&project_path) {
        *state = loaded_state;
        println!(
            "Loaded project from '{}' at {:?}",
            project_path.display(),
            project_path
        );
        // app_state_trigger.set(());
        *loaded_guard = true;
        // println!("Opened project Signal '{:?}'", app_state_trigger);
        return;
    }
    *loaded_guard = false;
    // Step 3: If no project.hre, scan the folder
    let project_name = project_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("UnnamedProject")
        .to_string();
    let mut file_tree = vec![FileNode {
        name: project_name.clone(),
        path: project_path.clone(),
        is_dir: true,
        language: "directory".to_string(),
    }];
    if let Ok(entries) = fs::read_dir(&project_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if !name.eq("project.hre") {
                // Skip project.hre
                let is_dir = path.is_dir();
                let language = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| match ext.to_lowercase().as_str() {
                        "txt" => "text".to_string(),
                        "py" => "python".to_string(),
                        "js" => "javascript".to_string(),
                        _ => "unknown".to_string(),
                    })
                    .unwrap_or("text".to_string());
                file_tree.push(FileNode {
                    name,
                    path,
                    is_dir,
                    language,
                });
            }
        }
    }

    // Step 4: Update global AppState
    state.project_path = Some(project_path.clone());
    state.file_tree = file_tree;
    *loaded_guard = true;
    // Step 5: Save to project.hre for future loads
    save_state(&state, &project_path);
    drop(state);

    println!("Opened project '{}' at {:?}", project_name, project_path);
}
