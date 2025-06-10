<script>
  import Editor from "./editor.svelte";
  import Filetree from "./filetree.svelte";
  import Menu from "./menu.svelte";
  import Resizerhorizontal from "./resizerhorizontal.svelte";
  import Statusbar from "./statusbar.svelte";
</script>

<div class="applayout">
  <Menu></Menu>
  <Filetree></Filetree>
  <Editor></Editor>
  <Statusbar></Statusbar>
</div>

<style>
  .applayout {
    display: grid;
    grid-template-columns: auto 1fr;
    grid-template-rows: auto 1fr auto;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;
  }
</style>
