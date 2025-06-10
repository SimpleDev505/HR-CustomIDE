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
