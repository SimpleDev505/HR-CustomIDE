<script lang="ts">
  import { bsicSetup } from "codemirror";
  import { EditorView } from "@codemirror/view";

  const view = new EditorView({
    doc: "Start document",
    parent: document.body,
    extensions: [basicSetup],
  });
</script>
