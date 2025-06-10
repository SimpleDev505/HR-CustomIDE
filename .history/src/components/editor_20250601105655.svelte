<script>
  import { is_newwindow } from "../lib/store";
  import Editorcore from "./editorcore.svelte";
  import Newwindow from "./newwindow.svelte";
</script>

<div class="editor-cnt grid-item">
  {#if $is_newwindow}
    <Newwindow></Newwindow>
  {:else}
    <Editorcore></Editorcore>
  {/if}
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
    /* font-size: var(--font-mm); */
  }
</style>
