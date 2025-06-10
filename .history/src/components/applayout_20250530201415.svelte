<script>
  import Menu from "./menu.svelte";
</script>

<div class="applayout">
  <Menu></Menu>
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
  }
</style>
