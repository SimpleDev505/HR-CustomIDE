use lazy_static::lazy_static;
use rfd::FileDialog;
use serde::Deserialize;
use serde::Serialize;
use std::fs::{self, File};
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
