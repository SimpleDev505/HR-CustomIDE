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
      extensions: [basicSetup, python(), oneDark],
    });
    view = new EditorView({
      state,
      parent: container,
    });
    return () => {
      view.destroy();
    };
  });
  function basicIntro(amount: number) {
    let msg = "Welcome,Starting Editing File by Opening a file!";
    for (let index = 0; index < amount; index++) {
      if (index != 0) {
        msg.concat("\n");
      }
    }
    return msg;
  }
</script>

<div bind:this={container} class="editor-core"></div>

<style>
  .editor-core {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    padding: 0;
    margin: 0;
  }
</style>
