<script>
  import { getContext } from "svelte";
  import { onMount } from "svelte";

  // --- Props ---
  let { id, title, tabclass } = $props();
  //   /** @type {string} */
  //   export let id;
  //   /** @type {string} */
  //   export let title;
  //   /** @type {string} */
  //   export let tabClass = "";

  const { registerTab, activeTabId } = getContext("tabs_context");

  // --- Register with TabWrapper on Mount ---
  onMount(() => {
    registerTab(id, title);
  });

  // The slot content will only be rendered if this tab is active.
</script>

{#if $activeTabId === id}
  <div class="tab-pane {tabclass}">
    <slot></slot>
  </div>
{/if}

<!-- <style>
  /* Default minimal styles for a tab pane */
  .tab-pane {
    /* Add any default styling for the content area of a tab here */
  }
</style> -->
