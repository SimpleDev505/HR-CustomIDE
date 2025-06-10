<script>
  import Editorcore from "./editorcore.svelte";
</script>

<div class="editor-cnt">
  <Editorcore></Editorcore>
</div>

<style>
  .editor-cnt {
    display: flex;
    width: 100%;
    height: 100%;
    justify-content: center;
    align-items: center;
    grid-column: 2;
    grid-row: 2;
    overflow: hidden;
  }
</style>
