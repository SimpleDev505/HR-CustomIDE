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
// pub fn save_state(state: &AppState, project_path: &PathBuf) {
//     let json = serde_json::to_string(state).unwrap();
//     let file_path = project_path.join("project.hre");
//     let mut file = File::create(&file_path).unwrap();
//     writeln!(file, "{}", json).unwrap();
// }
pub fn save_state(state: &AppState, project_path: &PathBuf) -> Result<(), String> {
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| format!("Failed to serialize app state: {}", e))?;
    let file_path = project_path.join("project.hre");
    let mut file = File::create(&file_path)
        .map_err(|e| format!("Failed to create .hre file at {:?}: {}", file_path, e))?;
    writeln!(file, "{}", json)
        .map_err(|e| format!("Failed to write to .hre file at {:?}: {}", file_path, e))?;
    println!("App state saved to {:?}", file_path);
    Ok(())
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
fn get_compiler(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "js" => "javascript".to_string(),
        "py" => "python".to_string(),
        "html" => "htmlmixed".to_string(),
        "css" => "css".to_string(),
        _ => "text".to_string(), // Default for unknown extensions
    }
}
#[tauri::command]
pub fn get_app_state() -> AppState {
    return APP_STATE
        .lock()
        .expect("Failed to lock APP_STATE mutex")
        .clone();
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
#[tauri::command]
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

//Update File (Rename or creating)
use walkdir::WalkDir;
fn scan_project_directory(project_root: PathBuf) -> Result<AppState, String> {
    // Acquire a lock on the global APP_STATE mutex.
    // This ensures exclusive access to the state to prevent data races.
    let mut current_app_state = APP_STATE
        .lock()
        .map_err(|e| format!("Failed to lock APP_STATE mutex: {}", e))?;

    let mut file_tree: Vec<FileNode> = Vec::new();

    // Ensure the provided project_root path exists and is a directory.
    if !project_root.exists() || !project_root.is_dir() {
        return Err(format!(
            "Project directory does not exist or is not a directory: {:?}",
            project_root
        ));
    }

    // --- CHANGE START ---
    // Walk the directory tree starting from the project_root.
    // Removed `min_depth(1)` so it includes the root directory itself.
    for entry in WalkDir::new(&project_root) {
        let entry = entry.map_err(|e| format!("Error walking directory: {}", e))?;
        let path = entry.path(); // This `path` is already the full absolute path

        let full_abs_path = path.to_path_buf();

        // Skip the project.hre file itself so it doesn't appear in the file tree.
        // We need to check against the full absolute path of project.hre
        let hre_file_abs_path = project_root.join("project.hre");
        if full_abs_path == hre_file_abs_path {
            continue;
        }

        // Determine if the current entry is a directory.
        let is_dir = path.is_dir();

        // Extract the file or directory name.
        // For the project root itself, use its directory name.
        let name = if path == project_root {
            project_root
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Project Root") // Fallback name for the root
                .to_string()
        } else {
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string()
        };

        // Determine the language/type based on extension for files, or "folder" for directories.
        let language = if is_dir {
            "folder".to_string() // Use "folder" for directories, including the root
        } else {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or("text".to_string())
        };

        // Create a new FileNode and add it to the file_tree.
        file_tree.push(FileNode {
            name,
            path: full_abs_path, // Store the full absolute path here
            is_dir,
            language,
        });
    }
    // --- CHANGE END ---

    // Update the global AppState with the newly scanned project path and file tree.
    current_app_state.project_path = Some(project_root.clone());
    current_app_state.file_tree = file_tree;

    // Save the updated AppState to the .hre file.
    // The `?` operator propagates any error from `save_state`.
    save_state(&current_app_state, &project_root)?;

    // Clone the updated state to return it.
    let updated_state = current_app_state.clone();

    // Explicitly drop the mutex lock.
    // This is important to release the lock as soon as it's no longer needed.
    drop(current_app_state);

    // Return the updated AppState.
    Ok(updated_state)
}
// fn scan_project_directory(project_root: PathBuf) -> Result<AppState, String> {
//     let mut current_app_state = APP_STATE
//         .lock()
//         .map_err(|e| format!("Failed to lock APP_STATE mutex: {}", e))?;

//     let mut file_tree: Vec<FileNode> = Vec::new();

//     // Ensure the project_root exists
//     if !project_root.exists() || !project_root.is_dir() {
//         return Err(format!(
//             "Project directory does not exist or is not a directory: {:?}",
//             project_root
//         ));
//     }

//     for entry in WalkDir::new(&project_root).min_depth(1) {
//         let entry = entry.map_err(|e| format!("Error walking directory: {}", e))?;
//         let path = entry.path();

//         let relative_path = path
//             .strip_prefix(&project_root)
//             .map_err(|e| format!("Failed to strip prefix for {:?}: {}", path, e))?
//             .to_path_buf();

//         if relative_path == PathBuf::from("project.hre") {
//             continue;
//         }

//         let is_dir = path.is_dir();
//         let name = path
//             .file_name()
//             .and_then(|n| n.to_str())
//             .unwrap_or_default()
//             .to_string();

//         let language = if is_dir {
//             "folder".to_string()
//         } else {
//             path.extension()
//                 .and_then(|ext| ext.to_str())
//                 .map(|s| s.to_lowercase())
//                 .unwrap_or("text".to_string())
//         };

//         file_tree.push(FileNode {
//             name,
//             path: relative_path,
//             is_dir,
//             language,
//         });
//     }

//     current_app_state.project_path = Some(project_root.clone());
//     current_app_state.file_tree = file_tree;

//     save_state(&current_app_state, &project_root)?;

//     let updated_state = current_app_state.clone();
//     drop(current_app_state); // Release the mutex lock

//     // Removed: app_handle.emit_all("app_state_updated", updated_state.clone())
//     Ok(updated_state)
// }

#[tauri::command(rename_all = "snake_case")]
pub async fn rename_file_or_dir(
    old_relative_path: String,
    new_name: String,
) -> Result<AppState, String> {
    // Removed app_handle
    let app_state_lock = APP_STATE
        .lock()
        .map_err(|e| format!("Failed to lock APP_STATE mutex: {}", e))?;
    let project_path = app_state_lock
        .project_path
        .clone()
        .ok_or("Project path not set. Cannot rename file/directory.")?;
    drop(app_state_lock);

    let old_abs_path = project_path.join(&old_relative_path);
    let parent_abs_path = old_abs_path
        .parent()
        .ok_or("Invalid old path for renaming.")?;
    let new_abs_path = parent_abs_path.join(&new_name);

    if !old_abs_path.exists() {
        return Err(format!(
            "Original path '{:?}' does not exist.",
            old_abs_path
        ));
    }
    if new_abs_path.exists() {
        return Err(format!(
            "'{}' already exists at the target location.",
            new_abs_path.display()
        ));
    }

    fs::rename(&old_abs_path, &new_abs_path).map_err(|e| {
        format!(
            "Failed to rename {:?} to {:?}: {}",
            old_abs_path, new_abs_path, e
        )
    })?;

    scan_project_directory(project_path)
}

//Tabs
use std::path::Path;
use uuid::Uuid;
lazy_static! {
    pub static ref TABS: Mutex<Vec<Tab>> = Mutex::new(Vec::new());
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tab {
    pub id: String,
    pub name: String,
    pub path: String,
    pub compiler: String,
}

// Create and store tab in global tabs array
pub fn create_tab(file_path: String) {
    let path = Path::new(&file_path);
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();
    let extension = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let compiler = get_compiler(extension);
    // let mut tab_state_trigger = use_context::<Signal<()>>();
    // Access and modify global TABS
    let mut tabs = TABS.lock().unwrap();
    if tabs.iter().any(|tab| tab.path == file_path) {
        return; // Skip if file is already open
    }

    // Create new tab
    let new_tab = Tab {
        id: Uuid::new_v4().to_string(),
        name,
        path: file_path,
        compiler,
    };

    // Update global tabs array
    tabs.push(new_tab);
    println!("Tabs: {:?}", *tabs);
    drop(tabs);
    // tab_state_trigger.set(());
    // Debug print all tabs
}
