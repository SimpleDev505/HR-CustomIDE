<script lang="ts">
  // import "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.52.2/min/vs/loader.min.js";
  // import { basicSetup } from "codemirror";
  // import { EditorView, keymap } from "@codemirror/view";
  // import {
  //   oneDark,
  //   oneDarkHighlightStyle,
  //   oneDarkTheme,
  // } from "@codemirror/theme-one-dark";
  //lang support
  // import { python, pythonLanguage } from "@codemirror/lang-python";
  // import {
  //   syntaxHighlighting,
  //   defaultHighlightStyle,
  //   bracketMatching,
  //   foldKeymap,
  //   HighlightStyle,
  //   LanguageSupport,
  // } from "@codemirror/language";
  // import { highlightCode, classHighlighter } from "@lezer/highlight";
  // import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
  // import { Compartment, EditorState, type Extension } from "@codemirror/state";
  import { onMount } from "svelte";
  import { editor } from "monaco-editor";
  import {
    FileTreeMenu_Enabled,
    LINE_COLUMN,
    OPEN_TABS,
    OPEN_TABS_CURRENT_SELECTED,
    OPEN_TABS_SELECTED_FILEPATH,
    PROJECT_NAME,
  } from "$lib/store";
  import { readTextFile } from "@tauri-apps/plugin-fs";
  // import {
  //   autocompletion,
  //   closeBrackets,
  //   closeBracketsKeymap,
  //   CompletionContext,
  //   completionKeymap,
  // } from "@codemirror/autocomplete";
  // import { lintKeymap } from "@codemirror/lint";
  // import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";

  let container: HTMLDivElement;
  // let view: EditorView;
  let editor_content = $state("");
  let editor_path_formated = $state("");
  // let currentLanguageExtension: LanguageSupport | Extension[] =
  //   $state(python());
  //Styles

  // ----

  const handleMouseMove = (event: MouseEvent) => {
    //   if (!view) return;
    //   // Get coordinates relative to the editor container
    //   const editorRect = view.dom.getBoundingClientRect();
    //   const clientX = event.clientX;
    //   const clientY = event.clientY;
    //   // Check if mouse is within editor bounds
    //   if (
    //     clientX >= editorRect.left &&
    //     clientX <= editorRect.right &&
    //     clientY >= editorRect.top &&
    //     clientY <= editorRect.bottom
    //   ) {
    //     // Get editor position from coordinates
    //     const pos = view.posAtCoords({ x: clientX, y: clientY });
    //     if (pos !== null) {
    //       // Get line information from the position
    //       const line = view.state.doc.lineAt(pos);
    //       let currentLine, currentCol, line_col;
    //       currentLine = line.number;
    //       currentCol = pos - line.from + 1; // Calculate column (1-based)
    //       line_col = `Ln: ${currentLine}, Col: ${currentCol}`;
    //       $LINE_COLUMN = line_col;
    //     }
    //   }
  };
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
      editor_content = await readTextFile(
        $OPEN_TABS_SELECTED_FILEPATH.toString()
      );
      if (editor_content.trim() === "") {
        editor_content = `// File seems empty! Start typing your awesome code here...`;
      }
      setLanguageExtension();
      console.warn(
        "Editor Path : ",
        $OPEN_TABS_SELECTED_FILEPATH,
        "Content: ",
        editor_content
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
  function setLanguageExtension() {
    let path = $OPEN_TABS_SELECTED_FILEPATH
      ? $OPEN_TABS_SELECTED_FILEPATH.toString()
      : "";
    if (path) {
      const fileExtension = path.split(".").pop()?.toLowerCase();
      // let newExtension;
      if (fileExtension) {
        // switch (fileExtension) {
        //   case "py":
        //     newExtension = python();
        //     break;
        //   // Add more cases for other languages
        //   default:
        //     newExtension = []; // Empty array (plain text)
        //     break;
        // }
        switch (fileExtension) {
          case "js":
          case "ts":
            return "javascript";
          case "py":
            return "python";
          case "html":
          case "htm":
            return "html";
          case "css":
            return "css";
          case "json":
            return "json";
          default:
            return "plaintext"; // Fallback for unrecognized extensions
        }
        // currentLanguageExtension = newExtension;
        // console.log("Choosen Lang: ", currentLanguageExtension);
      }
    }
  }
  async function SaveTab_Content() {}
  // let comp = new Compartment();
  let EditorArea: editor.IStandaloneCodeEditor;
  onMount(() => {
    const initialMessage = "Welcome, Starting Editing File by Opening a file!";
    const emptyLines = "\n".repeat(45);
    const fullInitialDoc = initialMessage + emptyLines;
    EditorArea = editor.create(container, {
      value: fullInitialDoc,
      language: "python",
      theme: "vs-dark",
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
    return () => {
      // view.destroy();
    };
  });
  $effect(() => {
    // if (view && editor_content !== view.state.doc.toString()) {
    //   view.dispatch({
    //     changes: {
    //       from: 0,
    //       to: view.state.doc.length,
    //       insert: editor_content,
    //     },
    //     effects: comp.reconfigure(currentLanguageExtension),
    //   });

    // }
    EditorArea.onDidChangeCursorPosition((e) => {
      currentLine = e.position.lineNumber;
      currentCol = e.position.column;
      formattedLnCol = `Ln: ${currentLine}, Col: ${currentCol}`;
      $LINE_COLUMN = `Ln: ${currentLine}, Col: ${currentCol}`;
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
