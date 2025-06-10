<script lang="ts">
  import { basicSetup } from "codemirror/";
  import { EditorView } from "@codemirror/view";
  import { oneDark } from "@codemirror/theme-one-dark";
  //lang support
  import { python } from "@codemirror/lang-python";
  import { onMount } from "svelte";
  import { EditorState } from "@codemirror/state";

  onMount(() => {
    const state = EditorState.create({
      doc: "Welcome,Starting Editing File by Opening a file!", // Initial document content
      extensions: [basicSetup, python(), oneDark],
    });
  });
</script>
