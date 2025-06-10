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

// pub fn open_project(app_handle: AppHandle) {
//     // Step 1: Prompt for project folder
//     let project_path = FileDialog::new()
//         .set_directory(std::env::var("HOME").unwrap_or_default())
//         .pick_folder();

//     let project_path = match project_path {
//         Some(path) => path,
//         None => {
//             println!("Project opening cancelled.");
//             return;
//         }
//     };
//     let mut loaded_guard = PROJECT_LOADED.lock().unwrap();
//     // Step 2: Try to load from project.hre
//     let mut state = APP_STATE.lock().unwrap();
//     if let Some(loaded_state) = load_state(&project_path) {
//         *state = loaded_state;
//         println!(
//             "Loaded project from '{}' at {:?}",
//             project_path.display(),
//             project_path
//         );
//         // app_state_trigger.set(());
//         *loaded_guard = true;
//         // println!("Opened project Signal '{:?}'", app_state_trigger);
//         return;
//     }
//     *loaded_guard = false;
//     // Step 3: If no project.hre, scan the folder
//     let project_name = project_path
//         .file_name()
//         .and_then(|s| s.to_str())
//         .unwrap_or("UnnamedProject")
//         .to_string();
//     let mut file_tree = vec![FileNode {
//         name: project_name.clone(),
//         path: project_path.clone(),
//         is_dir: true,
//         language: "directory".to_string(),
//     }];
//     if let Ok(entries) = fs::read_dir(&project_path) {
//         for entry in entries.flatten() {
//             let path = entry.path();
//             let name = path
//                 .file_name()
//                 .and_then(|s| s.to_str())
//                 .unwrap_or("")
//                 .to_string();
//             if !name.eq("project.hre") {
//                 // Skip project.hre
//                 let is_dir = path.is_dir();
//                 let language = path
//                     .extension()
//                     .and_then(|s| s.to_str())
//                     .map(|ext| match ext.to_lowercase().as_str() {
//                         "txt" => "text".to_string(),
//                         "py" => "python".to_string(),
//                         "js" => "javascript".to_string(),
//                         _ => "unknown".to_string(),
//                     })
//                     .unwrap_or("text".to_string());
//                 file_tree.push(FileNode {
//                     name,
//                     path,
//                     is_dir,
//                     language,
//                 });
//             }
//         }
//     }

//     // Step 4: Update global AppState
//     state.project_path = Some(project_path.clone());
//     state.file_tree = file_tree;
//     *loaded_guard = true;
//     // Step 5: Save to project.hre for future loads

//     let _ = save_state(&state, &project_path);
//     drop(state);
//     tauri::async_runtime::spawn(async move {
//         // Spawn awake_filewatcher
//         let _ = awake_filewatcher(
//             app_handle,
//             state
//                 .project_path
//                 .clone()
//                 .unwrap()
//                 .to_string_lossy()
//                 .into_owned(),
//         )
//         .await;
//     });

//     println!("Opened project '{}' at {:?}", project_name, project_path);
// }
