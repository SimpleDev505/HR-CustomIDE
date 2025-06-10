<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { browser } from "$app/environment";

  // Prop to specify which side the resizer is on
  export let position: "left" | "right" = "right";
  export let initialWidthPx: number = 200; // Initial width in pixels
  export let minimumWidthPx: number = 100;
  export let maximumWidthPx: number = 200;

  let containerElement: HTMLElement; // Reference to the main resizable div
  let currentWidth: number = initialWidthPx;
  let startX: number = 0;
  let initialContainerWidth: number = 0;

  let isResizing: boolean = false;

  function startResize(event: MouseEvent) {
    if (!browser) return;
    isResizing = true;
    startX = event.clientX;
    initialContainerWidth = containerElement.offsetWidth; // Get current pixel width

    window.addEventListener("mousemove", handleResize);
    window.addEventListener("mouseup", stopResize);

    // Add styles to body/html to prevent text selection and improve drag feel
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";
    document.body.style.pointerEvents = "none"; // Prevent interaction with other elements during drag
    containerElement.style.pointerEvents = "auto"; // Ensure resizer itself can be interacted with
  }

  function handleResize(event: MouseEvent) {
    if (!isResizing) return;

    const dx = event.clientX - startX;
    let newWidth: number;

    if (position === "right") {
      newWidth = initialContainerWidth + dx;
    } else {
      // position === 'left'
      newWidth = initialContainerWidth - dx;
    }

    // Ensure minimum width
    const minWidth = 50; // pixels
    currentWidth = Math.max(minWidth, newWidth);

    containerElement.style.width = `${currentWidth}px`; // Apply directly
  }

  function stopResize() {
    if (!browser) return;
    isResizing = false;
    window.removeEventListener("mousemove", handleResize);
    window.removeEventListener("mouseup", stopResize);

    // Reset body/html styles
    document.body.style.cursor = "default";
    document.body.style.userSelect = "auto";
    document.body.style.pointerEvents = "auto";
  }

  onMount(() => {
    if (browser) {
      containerElement.style.width = `${initialWidthPx}px`; // Set initial width
      containerElement.style.minWidth = `${minimumWidthPx}px`;
      containerElement.style.maxWidth = `${maximumWidthPx}px`;
    }
  });

  onDestroy(() => {
    stopResize(); // Clean up listeners if component is destroyed while resizing
  });
</script>

<div
  bind:this={containerElement}
  class="resizable-horizontal"
  class:resizer-left={position === "left"}
  class:resizer-right={position === "right"}
>
  <slot></slot>
  <div
    role="separator"
    aria-orientation="vertical"
    aria-label="Horizontal resizer"
    class="resizer-handle horizontal-handle"
    class:handle-left={position === "left"}
    class:handle-right={position === "right"}
    on:mousedown={startResize}
  ></div>
</div>

<style>
  .resizable-horizontal {
    position: relative;
    display: flex; /* Or block, depending on content. Flex helps with handle positioning. */
    overflow: hidden; /* Hide content that goes beyond current width */
    min-width: 50px; /* Enforce min width */
    box-sizing: border-box; /* Include padding/border in width calculation */
    height: 100%; /* Take full height of parent if in a grid row */
  }

  /* Positioning for the resizer handle */
  .resizer-handle {
    position: absolute;
    top: 0;
    width: 3px; /* Thickness of the handle */
    height: 100%;
    cursor: ew-resize; /* Horizontal resize cursor */
    z-index: 10; /* Ensure handle is above content */
    background-color: rgba(0, 0, 0, 0.1);
  }

  .handle-right {
    right: 0;
  }

  .handle-left {
    left: 0;
  }

  .resizable-horizontal:hover {
    /* background-color: rgba(0, 0, 0, 0.01); */
    opacity: 0.3;
  }
</style>
