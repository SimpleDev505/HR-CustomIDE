<script lang="ts">
  import { basicSetup } from "codemirror/";
  import { EditorView } from "@codemirror/view";
  import { oneDark } from "@codemirror/theme-one-dark";
  //lang support
  import { python } from "@codemirror/lang-python";
  import { onMount } from "svelte";
  import { EditorState } from "@codemirror/state";

  let { container: HTMLDivElement } = $props();
  let view: EditorView;

  const state = EditorState.create({
    doc: "Welcome,Starting Editing File by Opening a file!", // Initial document content
    extensions: [basicSetup, python(), oneDark],
  });
  view = new EditorView({
    state,
    parent: container,
  });
  onMount(() => {
    return () => {
      view.destroy();
    };
  });
</script>
