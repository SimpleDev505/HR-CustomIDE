use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_path: String,
    pub language_type: String,
    pub file_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub project_name: String,
    pub project_path: String,
    pub files: Vec<FileMetadata>,
}
