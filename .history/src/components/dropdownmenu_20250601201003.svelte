<script lang="ts">
  interface MenuItem {
    label: String;
    action: () => void; // Function to call on click
    split: boolean;
  }
</script>
