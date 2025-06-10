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
//Above Solid history - dont worry
//------------------------------------------------------------------------------
//---------------------Version before notify - Down-----------------------------
//------------------------------------------------------------------------------

// fn scan_project_directory(project_root: PathBuf) -> Result<AppState, String> {
//     // Acquire a lock on the global APP_STATE mutex.
//     // This ensures exclusive access to the state to prevent data races.
//     let mut current_app_state = APP_STATE
//         .lock()
//         .map_err(|e| format!("Failed to lock APP_STATE mutex: {}", e))?;

//     let mut file_tree: Vec<FileNode> = Vec::new();

//     // Ensure the provided project_root path exists and is a directory.
//     if !project_root.exists() || !project_root.is_dir() {
//         return Err(format!(
//             "Project directory does not exist or is not a directory: {:?}",
//             project_root
//         ));
//     }

//     // --- CHANGE START ---
//     // Walk the directory tree starting from the project_root.
//     // Removed `min_depth(1)` so it includes the root directory itself.
//     for entry in WalkDir::new(&project_root) {
//         let entry = entry.map_err(|e| format!("Error walking directory: {}", e))?;
//         let path = entry.path(); // This `path` is already the full absolute path

//         let full_abs_path = path.to_path_buf();

//         // Skip the project.hre file itself so it doesn't appear in the file tree.
//         // We need to check against the full absolute path of project.hre
//         let hre_file_abs_path = project_root.join("project.hre");
//         if full_abs_path == hre_file_abs_path {
//             continue;
//         }

//         // Determine if the current entry is a directory.
//         let is_dir = path.is_dir();

//         // Extract the file or directory name.
//         // For the project root itself, use its directory name.
//         let name = if path == project_root {
//             project_root
//                 .file_name()
//                 .and_then(|n| n.to_str())
//                 .unwrap_or("Project Root") // Fallback name for the root
//                 .to_string()
//         } else {
//             path.file_name()
//                 .and_then(|n| n.to_str())
//                 .unwrap_or_default()
//                 .to_string()
//         };

//         // Determine the language/type based on extension for files, or "folder" for directories.
//         let language = if is_dir {
//             "directory".to_string() // Use "folder" for directories, including the root
//         } else {
//             path.extension()
//                 .and_then(|ext| ext.to_str())
//                 .map(|s| s.to_lowercase())
//                 .unwrap_or("text".to_string())
//         };

//         // Create a new FileNode and add it to the file_tree.
//         file_tree.push(FileNode {
//             name,
//             path: full_abs_path, // Store the full absolute path here
//             is_dir,
//             language,
//         });
//     }
//     // --- CHANGE END ---

//     // Update the global AppState with the newly scanned project path and file tree.
//     current_app_state.project_path = Some(project_root.clone());
//     current_app_state.file_tree = file_tree;

//     // Save the updated AppState to the .hre file.
//     // The `?` operator propagates any error from `save_state`.
//     save_state(&current_app_state, &project_root)?;

//     // Clone the updated state to return it.
//     let updated_state = current_app_state.clone();

//     // Explicitly drop the mutex lock.
//     // This is important to release the lock as soon as it's no longer needed.
//     drop(current_app_state);

//     // Return the updated AppState.
//     Ok(updated_state)
// }
