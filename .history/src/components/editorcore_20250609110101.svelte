<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { editor, KeyCode, KeyMod } from "monaco-editor";
  import {
    EDITOR_AREA,
    FILE_LANG,
    FileTreeMenu_Enabled,
    LINE_COLUMN,
    OPEN_TABS,
    OPEN_TABS_CURRENT_SELECTED,
    OPEN_TABS_SELECTED_FILEPATH,
    PROJECT_NAME,
  } from "$lib/store";
  import { readTextFile } from "@tauri-apps/plugin-fs";
  import { invoke } from "@tauri-apps/api/core";

  let container: HTMLDivElement;
  let current_tab_content = $state("");
  let editor_path_formated = $state("");
  let _editor_content = $state("");

  function parsePathForBreadcrumbs() {
    let path = $OPEN_TABS_SELECTED_FILEPATH;
    let projectName = $PROJECT_NAME.toString();
    if (!path) {
      return "No file selected."; // Return a default message if path is empty
    }

    // 1. Normalize slashes for consistent processing
    let displayPath = path.replace(/\\/g, "/");

    // 2. Check if path contains the project name and trim everything before it
    const projectIndex = displayPath.indexOf(projectName);
    if (projectIndex !== -1) {
      displayPath = displayPath.substring(projectIndex);
      // 3. Replace the project name with 'root' at the beginning
      displayPath = displayPath.replace(projectName, "root");
    } else {
      // If project name not found, just normalize leading/trailing slashes
      displayPath = displayPath.replace(/^\/|\/$/g, "");
    }

    // 4. Split into segments and filter out any empty segments
    const segments = displayPath.split("/").filter((segment) => segment !== "");

    // 5. Join segments with ' > ' for the final display string
    return segments.join(" > ");
  }
  async function LoadTab_Content() {
    try {
      $OPEN_TABS_SELECTED_FILEPATH =
        $OPEN_TABS[$OPEN_TABS_CURRENT_SELECTED].tab_path;
      current_tab_content = await readTextFile(
        $OPEN_TABS_SELECTED_FILEPATH.toString()
      );
      if (current_tab_content.trim() === "") {
        current_tab_content = `// File seems empty! Start typing your awesome code here...`;
      }
      const languageId = GetLanguageExtension();
      const newModel = editor.createModel(current_tab_content, languageId);
      $EDITOR_AREA.setModel(newModel);
      $FILE_LANG = languageId ?? "Loading";
      console.log("New Language : ", newModel);
      console.warn(
        "Editor Path : ",
        $OPEN_TABS_SELECTED_FILEPATH,
        "Content: ",
        current_tab_content
      );
    } catch (e) {
      console.error(
        "Cannot Read Selected Tab : ",
        e,
        "Path : ",
        $OPEN_TABS_SELECTED_FILEPATH.toString()
      );
    }
  }
  function GetLanguageExtension() {
    let path = $OPEN_TABS_SELECTED_FILEPATH
      ? $OPEN_TABS_SELECTED_FILEPATH.toString()
      : "";
    if (path) {
      const fileExtension = path.split(".").pop()?.toLowerCase();
      if (fileExtension) {
        switch (fileExtension) {
          case "js":
          case "ts":
            return "javascript"; // 'javascript' is the ID for JS/TS
          case "py":
            return "python";
          case "html":
          case "htm":
            return "html";
          case "css":
            return "css";
          case "json":
            return "json";
          case "xml":
            return "xml";
          case "md":
            return "markdown";
          // Add more cases for other languages you want to support.
          // You can find Monaco's built-in language IDs in their documentation.
          default:
            return "plaintext"; // Default to plain text for unrecognized extensions.
        }
      }
    }
  }
  async function SaveFile() {
    const result = await invoke("save_file", {
      path: $OPEN_TABS_SELECTED_FILEPATH,
      contents: _editor_content,
    });
  }
  async function SaveTab_Content() {}
  // let comp = new Compartment();
  onMount(() => {
    const initialMessage = "Welcome, Starting Editing File by Opening a file!";
    const emptyLines = "\n".repeat(45);
    const fullInitialDoc = initialMessage + emptyLines;
    $EDITOR_AREA = editor.create(container, {
      value: fullInitialDoc,
      language: "python",
      theme: "vs-dark",
      fontSize: 20,
      lightbulb: {
        enabled: editor.ShowLightbulbIconMode.On,
      },
      suggest: {
        showIcons: true,
      },
    });
    editor.defineTheme("dracula", {
      base: "vs-dark",
      inherit: true,
      rules: [
        { token: "", background: "282a36", foreground: "f8f8f2" },
        { token: "comment", foreground: "6272a4" },
        { token: "keyword", foreground: "ff79c6" },
        { token: "string", foreground: "f1fa8c" },
      ],
      colors: {
        "editor.background": "#282a36",
      },
    });
  });
  onDestroy(() => {
    $EDITOR_AREA.dispose();
  });
  $effect(() => {
    if ($EDITOR_AREA && $EDITOR_AREA.getValue() !== current_tab_content) {
      $EDITOR_AREA.setValue(current_tab_content);
    }
    $EDITOR_AREA.onDidChangeCursorPosition((e) => {
      let currentLine, currentCol;
      currentLine = e.position.lineNumber;
      currentCol = e.position.column;
      $LINE_COLUMN = `Ln: ${currentLine}, Col: ${currentCol}`;
    });
    $EDITOR_AREA.addCommand(KeyMod.CtrlCmd | KeyCode.KeyS, () => {
      SaveTab_Content(); // Call your save function when Ctrl+S is pressed
    });
    editor_path_formated = parsePathForBreadcrumbs();
  });
</script>

<div class="editor-core-layout">
  <div class="editor-core-opentabs">
    <!-- <span>Tab1</span>
    <span class="tab-selected">Tab1</span>
    <span>Tab1</span>
    <span>Tab1</span>
    <span>Tab1</span> -->
    {#each $OPEN_TABS as tab, tab_index}
      <span
        role="button"
        tabindex={tab_index}
        onmousedown={() => {
          $OPEN_TABS_CURRENT_SELECTED = tab_index;
          console.log("Editor : ", $OPEN_TABS_CURRENT_SELECTED);
          LoadTab_Content();
        }}
        class="tab {tab_index === $OPEN_TABS_CURRENT_SELECTED
          ? 'tab-selected'
          : ''}">{tab.tab_name}</span
      >
    {/each}
  </div>
  <div class="editor-core-opentab-path">{editor_path_formated}</div>
  <div bind:this={container} class="editor-core"></div>
</div>

<style>
  .editor-core-layout {
    display: grid;
    grid-template-columns: repeat(1);
    grid-template-rows: auto auto 1fr;
    width: 100%;
    height: 100%;
  }
  .editor-core-opentabs {
    grid-column: 1;
    grid-row: 1;
    display: flex;
    flex-direction: row;
    overflow-x: hidden;
    overflow-y: auto;
    width: 100%;
    height: 1.8em;
  }
  /* temp */
  .tab {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    min-width: 6em;
    max-width: fit-content;
    font-size: var(--font-sm);
    border-left: 1px solid grey;
    border-right: 1px solid grey;
    user-select: none;
    border-bottom: 1px solid grey;
    white-space: nowrap;
    overflow: hidden;
    padding: 5px;
  }
  .tab-selected {
    background-color: rgba(255, 255, 255, 0.1);
    border-top: 2px solid var(--blue-bg) !important;
    border-bottom: 0 !important;
  }

  /* --------- */
  .editor-core-opentab-path {
    grid-column: 1;
    grid-row: 2;
    width: 100%;
    height: 2em;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    padding-left: 10px;
    font-size: 14px;
    color: grey;
    box-shadow: 2px 9px 10px 0px rgba(0, 0, 0, 0.75);
    -webkit-box-shadow: 2px 9px 10px 0px rgba(0, 0, 0, 0.75);
    -moz-box-shadow: 2px 9px 10px 0px rgba(0, 0, 0, 0.75);
    border-bottom: 2px solid rgba(255, 255, 255, 0.1);
  }
  .editor-core {
    grid-column: 1;
    grid-row: 3;
    align-self: center;
    height: 100%;
    width: 100%;
    overflow: scroll;
    padding: 0;
    margin: 0;
    scrollbar-color: transparent transparent;
    scrollbar-width: thin;
    scroll-behavior: smooth;
    cursor: text;
  }
</style>
