<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { browser } from "$app/environment";

  // Prop to specify which side the resizer is on
  export let position: "top" | "bottom" = "bottom";
  export let initialHeightPx: number = 100; // Initial height in pixels

  let containerElement: HTMLElement; // Reference to the main resizable div
  let currentHeight: number = initialHeightPx;
  let startY: number = 0;
  let initialContainerHeight: number = 0;

  let isResizing: boolean = false;

  function startResize(event: MouseEvent) {
    if (!browser) return;
    isResizing = true;
    startY = event.clientY;
    initialContainerHeight = containerElement.offsetHeight; // Get current pixel height

    window.addEventListener("mousemove", handleResize);
    window.addEventListener("mouseup", stopResize);

    document.body.style.cursor = "ns-resize";
    document.body.style.userSelect = "none";
    document.body.style.pointerEvents = "none";
    containerElement.style.pointerEvents = "auto";
  }

  function handleResize(event: MouseEvent) {
    if (!isResizing) return;

    const dy = event.clientY - startY;
    let newHeight: number;

    if (position === "bottom") {
      newHeight = initialContainerHeight + dy;
    } else {
      // position === 'top'
      newHeight = initialContainerHeight - dy;
    }

    // Ensure minimum height
    const minHeight = 50; // pixels
    currentHeight = Math.max(minHeight, newHeight);

    containerElement.style.height = `${currentHeight}px`; // Apply directly
  }

  function stopResize() {
    if (!browser) return;
    isResizing = false;
    window.removeEventListener("mousemove", handleResize);
    window.removeEventListener("mouseup", stopResize);

    document.body.style.cursor = "default";
    document.body.style.userSelect = "auto";
    document.body.style.pointerEvents = "auto";
  }

  onMount(() => {
    if (browser) {
      containerElement.style.height = `${initialHeightPx}px`; // Set initial height
    }
  });

  onDestroy(() => {
    stopResize();
  });
</script>

<div
  bind:this={containerElement}
  class="resizable-vertical"
  class:resizer-top={position === "top"}
  class:resizer-bottom={position === "bottom"}
>
  <slot></slot>
  <div
    role="separator"
    aria-orientation="horizontal"
    aria-label="Vertical resizer"
    class="resizer-handle vertical-handle"
    class:handle-top={position === "top"}
    class:handle-bottom={position === "bottom"}
    on:mousedown={startResize}
  ></div>
</div>

<style>
  .resizable-vertical {
    position: relative;
    display: flex; /* Or block, depending on content. Flex helps with handle positioning. */
    overflow: hidden; /* Hide content that goes beyond current height */
    min-height: 50px; /* Enforce min height */
    box-sizing: border-box; /* Include padding/border in height calculation */
    width: 100%; /* Take full width of parent if in a grid column */
  }

  /* Positioning for the resizer handle */
  .resizer-handle {
    position: absolute;
    left: 0;
    height: 8px; /* Thickness of the handle */
    width: 100%;
    cursor: ns-resize; /* Vertical resize cursor */
    z-index: 10;
    background-color: rgba(0, 0, 0, 0.1);
  }

  .handle-bottom {
    bottom: 0;
  }

  .handle-top {
    top: 0;
  }

  .resizer-handle:hover {
    background-color: rgba(0, 0, 0, 0.3);
  }
</style>
