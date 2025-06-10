import { invoke } from "@tauri-apps/api/core";
import type { editor } from "monaco-editor";
import { get, writable } from "svelte/store";
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

import axios from "axios";
const RAPIDAPI_HOST = "judge0-ce.p.rapidapi.com";
const RAPIDAPI_KEY = import.meta.env?.VITE_RAPIDAPI_KEY || "";
const JUDGE0_SUBMISSION_URL =
    `https://${RAPIDAPI_HOST}/submissions?base64_encoded=false&wait=true`;
export async function FetchOuput() {
    if (!get(EDITOR_AREA) && get(OPEN_TABS_SELECTED_FILEPATH) === "") {
        return;
    }
    const options = {
        method: "POST",
        url: "https://judge0-ce.p.rapidapi.com/submissions",
        params: {
            base64_encoded: "true",
            wait: "false",
            fields: "*",
        },
        headers: {
            "x-rapidapi-key":
                "32088943a9msh0a19c2a436be580p1031bfjsnff9fa09908b3",
            "x-rapidapi-host": "judge0-ce.p.rapidapi.com",
            "Content-Type": "application/json",
        },
        data: {
            language_id: 52,
            source_code:
                "I2luY2x1ZGUgPHN0ZGlvLmg+CgppbnQgbWFpbih2b2lkKSB7CiAgY2hhciBuYW1lWzEwXTsKICBzY2FuZigiJXMiLCBuYW1lKTsKICBwcmludGYoImhlbGxvLCAlc1xuIiwgbmFtZSk7CiAgcmV0dXJuIDA7Cn0=",
            stdin: "SnVkZ2Uw",
        },
    };

    try {
        const response = await axios.request(options);
        console.log(response.data);
    } catch (error) {
        console.error(error);
    }
}
