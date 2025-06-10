<script lang="ts">
  import { basicSetup } from "codemirror/";
  import { EditorView } from "@codemirror/view";
  import { oneDark } from "@codemirror/theme-one-dark";
  //lang support
  import { python } from "@codemirror/lang-python";

  const view = new EditorView({
    doc: "Start document",
    parent: document.body,
    extensions: [basicSetup, python(), oneDark],
  });
</script>
