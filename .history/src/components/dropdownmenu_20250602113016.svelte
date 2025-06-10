<script lang="ts">
  import type { MenuItem } from "$lib/store";
  import { onMount } from "svelte";

  let { items }: { items: MenuItem[] } = $props();
  function handleClickOutside(event: MouseEvent) {
        // Check if the clicked element is NOT part of the dropdownElement or its children.
        // `dropdownElement.contains(event.target as Node)` returns true if event.target
        // is dropdownElement itself, or one of its descendants.
        if (dropdownElement && !dropdownElement.contains(event.target as Node)) {
            $isDropdownOpen = false; // Set the store value to false to close the dropdown
            console.log('Clicked outside dropdown. Closing...');
        }
  onMount(() => {
    document.addEventListener('click', handleClickOutside);
  })
</script>

<div
  class="dropdown-menu"
  style="position: absolute; top: 40px; left: 10px; background-color: #2d2d2d; color: #d4d4d4; border: 1px solid #3c3c3c; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.3); z-index: 1000;"
>
  {#each items as item}
    <div
      role="button"
      tabindex="999"
      class="dropdown-item"
      style={item.split == true ? "border-bottom: 2px solid black;" : ""}
      onmousedown={item.action}
    >
      {item.label}
    </div>
  {/each}
</div>

<style>
  .dropdown-item {
    padding: 8px 16px;
    cursor: pointer;
    font-family: "Segoe UI", Arial, sans-serif;
    font-size: 14px;
  }

  .dropdown-item:hover {
    background-color: black;
  }
</style>
