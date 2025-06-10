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
          "&.cm-editor": { fontSize: "18px" },
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

<div bind:this={container} class="editor-core"></div>

<style>
  .editor-core {
    align-self: center;
    height: 100%;
    width: 100%;
    overflow: scroll;
    padding: 0;
    margin: 0;
    scrollbar-color: transparent transparent;
    scrollbar-width: thin;
    scroll-behavior: smooth;
  }
</style>
