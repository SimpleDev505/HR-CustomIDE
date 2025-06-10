<script lang="ts">
  interface MenuItem {
    label: String;
    action: () => void; // Function to call on click
    split: boolean;
  }
  function default_action() {
    console.log("No Action Specified For Menu Item!");
  }
  let { items }: { items: MenuItem[] } = $props();
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
