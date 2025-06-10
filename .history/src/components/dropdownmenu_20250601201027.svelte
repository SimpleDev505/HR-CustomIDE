<script lang="ts">
  interface MenuItem {
    label: String;
    action: () => void; // Function to call on click
    split: boolean;
  }
  function default_action() {
    console.log("No Action Specified For Menu Item!");
  }
</script>
