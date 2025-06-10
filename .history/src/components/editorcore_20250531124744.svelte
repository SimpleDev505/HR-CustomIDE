<script lang="ts">
  import { basicSetup } from "codemirror/";
  import { EditorView } from "@codemirror/view";
  //lang support
  import { python } from "@codemirror/lang-python";

  const view = new EditorView({
    doc: "Start document",
    parent: document.body,
    extensions: [basicSetup, python()],
  });
</script>
