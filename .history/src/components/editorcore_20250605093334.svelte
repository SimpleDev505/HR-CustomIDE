<script lang="ts">
  import { basicSetup } from "codemirror";
  import { EditorView } from "@codemirror/view";
  import { oneDark } from "@codemirror/theme-one-dark";
  //lang support
  import { python } from "@codemirror/lang-python";
  import { onMount } from "svelte";
  import { EditorState } from "@codemirror/state";

  let container: HTMLDivElement;
  let view: EditorView;
  onMount(() => {
    const initialMessage = "Welcome, Starting Editing File by Opening a file!";
    const emptyLines = "\n".repeat(45);
    const fullInitialDoc = initialMessage + emptyLines;
    const state = EditorState.create({
      doc: fullInitialDoc, // Initial document content
      extensions: [
        basicSetup,
        python(),
        oneDark,
        EditorState.allowMultipleSelections.of(true),
        EditorView.theme({
          "&.cm-editor": { fontSize: "17px" },
          ".cm-content": { lineHeight: "1.5" },
        }),
      ],
    });
    view = new EditorView({
      state,
      parent: container,
    });
    return () => {
      view.destroy();
    };
  });
</script>

<div class="editor-core-layout">
  <div class="editor-core-opentabs">
    <span>Tab1</span>
    <span class="tab-selected">Tab1</span>
    <span>Tab1</span>
    <span>Tab1</span>
    <span>Tab1</span>
  </div>
  <div class="editor-core-opentab-path">path path path path</div>
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
  .editor-core-opentabs span {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    width: 6em;
    font-size: var(--font-sm);
    border-left: 1px solid grey;
    border-right: 1px solid grey;
    user-select: none;
    border-bottom: 1px solid grey;
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
    height: 5em;
    display: flex;
    flex-direction: column;
    justify-content: baseline;
    align-items: flex-start;
    padding-top: 10px;
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
