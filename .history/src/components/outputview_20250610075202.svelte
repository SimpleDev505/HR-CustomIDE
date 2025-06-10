<script>
  import { OUTPUTS } from "$lib/store";
</script>

<div class="out-cnt">
  {#each $OUTPUTS as output}{/each}
</div>

<style>
  .out-cnt {
    display: flex;
    flex-direction: column;
    row-gap: 2px;
    width: 100%;
    height: 100%;
    z-index: -1;
  }
</style>
