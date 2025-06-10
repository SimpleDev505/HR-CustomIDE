import { invoke } from "@tauri-apps/api/core";
import type { editor } from "monaco-editor";
import { writable } from "svelte/store";
//Status
export const LINE_COLUMN = writable<string>("Ln 0, Col 0");
export const warn_error = writable<string>("Wn 0, Er 0");
export const FILE_LANG = writable<string>("Loading");

//Window
export const is_newwindow = writable<Boolean>(false);

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

//DropDown
export const dropdown_enabled = writable<boolean>(false);
export interface MenuItem {
    label: String;
    action: () => void; // Function to call on click
    split: boolean;
}
//App State
export interface FileNode {
    name: String;
    path: String;
    is_dir: boolean;
    language: String;
}

export interface AppState {
    project_path: String;
    file_tree: FileNode[];
}

export const APP_STATE = writable<AppState>();
export const PROJECT_NAME = writable<String>("");
export async function Fetch_APPSTATE() {
    const appstate: AppState = await invoke("get_app_state", {});
    APP_STATE.set(appstate);
    appstate.file_tree.find((file) => {
        if (file.is_dir) {
            PROJECT_NAME.set(file.name);
        }
    });
}
//FileTree -Selected File
// export const SelectedFile_Name = writable<String>("");
export const SelectedFile_Path = writable<String>("");
export const FileTreeMenu_Enabled = writable<Boolean>(false);

//Tabs
export interface TabState {
    tab_name: String;
    tab_path: String;
    tab_lang: String;
}
export const OPEN_TABS = writable<TabState[]>([]);
export const OPEN_TABS_CURRENT_SELECTED = writable<number>(-1);
export const OPEN_TABS_SELECTED_FILEPATH = writable<String>("");

//Editor Area
export let EDITOR_AREA = writable<editor.IStandaloneCodeEditor>();

//Output
export const OUTPUTS = writable<String[]>([]);
