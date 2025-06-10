import { writable } from "svelte/store";
//Status
export const current_lang = writable<string>("text");
export const line_cloumn = writable<string>("Ln 0, Col 0");
export const warn_error = writable<string>("Wn 0, Er 0");

//Window
export const is_newwindow = writable<Boolean>(true);

//Project Creation/Opening

export interface FileMetadata {
    file_path: string;
    language_type: string;
    file_name: string;
}

export interface ProjectMetadata {
    project_name: string;
    project_path: string;
    files: FileMetadata[];
}

export const projectStore = writable<ProjectMetadata | null>(null);
