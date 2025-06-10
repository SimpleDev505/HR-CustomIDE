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
    <span>Tab1</span>
    <span>Tab1</span>
    <span>Tab1</span>
    <span>Tab1</span>
  </div>
  <div bind:this={container} class="editor-core"></div>
</div>

<style>
  .editor-core-layout {
    display: grid;
    grid-template-columns: repeat(1);
    grid-template-rows: auto 1fr;
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
    height: fit-content;
  }
  /* temp */
  .editor-core-opentabs span {
    height: 2em;
    width: 4em;
    text-align: center;
    border-left: 1px solid grey;
    border-right: 1px solid grey;
  }
  /* --------- */
  .editor-core {
    grid-column: 1;
    grid-row: 2;
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
