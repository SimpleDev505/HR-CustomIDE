<script lang="ts">
  import { basicSetup } from "codemirror/";
  import { EditorView } from "@codemirror/view";
  //lang support
  import { pythonLanguage } from "@codemirror/lang-python";

  const view = new EditorView({
    doc: "Start document",
    parent: document.body,
    extensions: [basicSetup],
    pythonLanguage()
  });
</script>
