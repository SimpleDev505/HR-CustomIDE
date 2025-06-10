use rfd::FileDialog;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
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
