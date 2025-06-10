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

<div class: "dropdown-menu",
style: "position: absolute; top: 40px; left: 10px; background-color: #2d2d2d; color: #d4d4d4; border: 1px solid #3c3c3c; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.3); z-index: 1000;">

</div>
    
    for item in items {
        div {
            class: "dropdown-item",
            style: if item.split == Some(true) { "border-bottom: 2px solid black;" } else { "" },
            onmouseover: move |_| {},
            onclick: move |_| (item.action)(), // Call the item's action
            "{item.label}"
        
        }
    }
}
