<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { browser } from "$app/environment";

  // Props for configuring the grid
  export let numColumns: number = 2;
  export let numRows: number = 1; // Start with 1 row, you can add more via slots

  // Initial column widths and row heights. Use CSS 'fr' units for flexibility.
  // You can also use 'px' or 'auto' for initial values.
  export let initialColumnWidths: string[] = Array(numColumns).fill("1fr");
  export let initialRowHeights: string[] = Array(numRows).fill("1fr");

  let gridElement: HTMLElement; // Reference to the main grid div
  let columnWidths: string[] = [...initialColumnWidths];
  let rowHeights: string[] = [...initialRowHeights];

  let resizingColumnIndex: number | null = null;
  let resizingRowIndex: number | null = null;
  let startX: number = 0;
  let startY: number = 0;
  let initialSizes: number[] = []; // Stores pixel sizes at start of drag

  let activeResizerType: "column" | "row" | null = null;

  // Function to set CSS variables for grid tracks
  function setGridStyles() {
    if (gridElement) {
      gridElement.style.gridTemplateColumns = columnWidths.join(" ");
      gridElement.style.gridTemplateRows = rowHeights.join(" ");

      columnWidths.forEach((width, i) => {
        gridElement.style.setProperty(`--col-width-${i}`, width);
      });
      rowHeights.forEach((height, i) => {
        gridElement.style.setProperty(`--row-height-${i}`, height);
      });
    }
  }

  // --- Mouse Down Handlers ---
  function startColumnResize(event: MouseEvent, index: number) {
    if (!browser) return;
    activeResizerType = "column";
    resizingColumnIndex = index;
    startX = event.clientX;

    // Get current pixel widths of the columns involved
    initialSizes = Array.from(gridElement.children)
      .map((child, i) => {
        // Find the element in the target column
        const colStart = parseInt(
          getComputedStyle(child as Element).gridColumnStart
        );
        const colEnd = parseInt(
          getComputedStyle(child as Element).gridColumnEnd
        );
        if (colStart <= index + 1 && colEnd > index + 1) {
          // Check if element spans across the resizer
          return (child as HTMLElement).offsetWidth;
        }
        return 0; // Not ideal, but we need pixel sizes. More robust: get actual column pixel sizes.
      })
      .filter((size) => size > 0); // Filter out 0s from non-matching children
    // A more robust way to get column pixel sizes would be to inspect grid layout directly,
    // but this often requires more complex calculations or a temporary element to measure.
    // For a basic example, we'll try to calculate from grid element rects.

    const columns =
      getComputedStyle(gridElement).gridTemplateColumns.split(" ");
    initialSizes = [];
    let currentPixelWidth = 0;
    for (let i = 0; i < columns.length; i++) {
      const tempDiv = document.createElement("div");
      tempDiv.style.gridColumn = `${i + 1} / ${i + 2}`;
      tempDiv.style.visibility = "hidden";
      gridElement.appendChild(tempDiv);
      initialSizes.push(tempDiv.offsetWidth);
      gridElement.removeChild(tempDiv);
    }
    // Correctly get sizes for the two columns involved
    const col1ComputedWidth =
      gridElement.children[index]?.getBoundingClientRect().width || 0;
    const col2ComputedWidth =
      gridElement.children[index + 1]?.getBoundingClientRect().width || 0;
    initialSizes = [col1ComputedWidth, col2ComputedWidth]; // Simplify for adjacent columns
    console.log("Start Column Resize:", initialSizes);

    window.addEventListener("mousemove", handleColumnResize);
    window.addEventListener("mouseup", stopResize);
    gridElement.style.cursor = "ew-resize"; // Indicate horizontal resizing
    document.body.style.userSelect = "none"; // Prevent text selection during drag
  }

  function startRowResize(event: MouseEvent, index: number) {
    if (!browser) return;
    activeResizerType = "row";
    resizingRowIndex = index;
    startY = event.clientY;

    // Get current pixel heights of the rows involved
    const rows = getComputedStyle(gridElement).gridTemplateRows.split(" ");
    initialSizes = [];
    for (let i = 0; i < rows.length; i++) {
      const tempDiv = document.createElement("div");
      tempDiv.style.gridRow = `${i + 1} / ${i + 2}`;
      tempDiv.style.visibility = "hidden";
      gridElement.appendChild(tempDiv);
      initialSizes.push(tempDiv.offsetHeight);
      gridElement.removeChild(tempDiv);
    }
    // Simplify for adjacent rows
    const row1ComputedHeight =
      gridElement.children[index * numColumns]?.getBoundingClientRect()
        .height || 0;
    const row2ComputedHeight =
      gridElement.children[(index + 1) * numColumns]?.getBoundingClientRect()
        .height || 0;
    initialSizes = [row1ComputedHeight, row2ComputedHeight];
    console.log("Start Row Resize:", initialSizes);

    window.addEventListener("mousemove", handleRowResize);
    window.addEventListener("mouseup", stopResize);
    gridElement.style.cursor = "ns-resize"; // Indicate vertical resizing
    document.body.style.userSelect = "none";
  }

  // --- Mouse Move Handlers ---
  function handleColumnResize(event: MouseEvent) {
    if (resizingColumnIndex === null || activeResizerType !== "column") return;

    const dx = event.clientX - startX; // Change in mouse X position

    // Adjust the widths of the two columns involved
    // This is a simplified calculation. For 'fr' units, you might need to convert
    // to pixels, apply change, then convert back or use 'calc()'.
    // For fixed pixel sizes, it's more direct.
    // Let's use pixels for now, assuming initial sizes are pixels or converting to them.

    let newCol1Width = initialSizes[0] + dx;
    let newCol2Width = initialSizes[1] - dx;

    // Prevent columns from shrinking too much
    const minWidth = 50; // pixels
    if (newCol1Width < minWidth) {
      newCol2Width -= minWidth - newCol1Width; // Adjust 2nd col if 1st hits min
      newCol1Width = minWidth;
    }
    if (newCol2Width < minWidth) {
      newCol1Width -= minWidth - newCol2Width; // Adjust 1st col if 2nd hits min
      newCol2Width = minWidth;
    }

    columnWidths[resizingColumnIndex] = `${Math.max(0, newCol1Width)}px`;
    columnWidths[resizingColumnIndex + 1] = `${Math.max(0, newCol2Width)}px`;

    setGridStyles(); // Apply the new styles
  }

  function handleRowResize(event: MouseEvent) {
    if (resizingRowIndex === null || activeResizerType !== "row") return;

    const dy = event.clientY - startY; // Change in mouse Y position

    let newRow1Height = initialSizes[0] + dy;
    let newRow2Height = initialSizes[1] - dy;

    const minHeight = 50; // pixels
    if (newRow1Height < minHeight) {
      newRow2Height -= minHeight - newRow1Height;
      newRow1Height = minHeight;
    }
    if (newRow2Height < minHeight) {
      newRow1Height -= minHeight - newRow2Height;
      newRow2Height = minHeight;
    }

    rowHeights[resizingRowIndex] = `${Math.max(0, newRow1Height)}px`;
    rowHeights[resizingRowIndex + 1] = `${Math.max(0, newRow2Height)}px`;

    setGridStyles(); // Apply the new styles
  }

  // --- Mouse Up Handler ---
  function stopResize() {
    if (!browser) return;
    resizingColumnIndex = null;
    resizingRowIndex = null;
    activeResizerType = null;
    window.removeEventListener("mousemove", handleColumnResize);
    window.removeEventListener("mousemove", handleRowResize);
    window.removeEventListener("mouseup", stopResize);
    gridElement.style.cursor = "default"; // Reset cursor
    document.body.style.userSelect = "auto"; // Re-enable text selection
    console.log("Stop Resize - Final Column Widths:", columnWidths);
    console.log("Stop Resize - Final Row Heights:", rowHeights);
  }

  onMount(() => {
    if (browser) {
      setGridStyles(); // Set initial styles on mount
    }
  });

  onDestroy(() => {
    // Ensure listeners are cleaned up if component is destroyed while dragging
    stopResize();
  });

  // Reactive statement to re-apply styles if columnWidths/rowHeights change
  // Note: We are already calling setGridStyles in handlers, but this ensures
  // reactivity if props change from parent.
  // $: {
  //   setGridStyles();
  // }
</script>

<div
  bind:this={gridElement}
  class="resizable-grid"
  style="
         grid-template-columns: {columnWidths.join(' ')};
         grid-template-rows: {rowHeights.join(' ')};
         min-height: 100px; /* Ensure grid has some minimum height */
         min-width: 100px;  /* Ensure grid has some minimum width */
       "
>
  <slot></slot>

  {#each Array(numColumns - 1) as _, i}
    <div
      role="seperator"
      class="resizer resizer-col"
      style="grid-column: {i + 1} / {i + 2}; grid-row: 1 / -1;"
      on:mousedown={(e) => startColumnResize(e, i)}
    ></div>
  {/each}

  {#each Array(numRows - 1) as _, i}
    <div
      class="resizer resizer-row"
      style="grid-row: {i + 1} / {i + 2}; grid-column: 1 / -1;"
      on:mousedown={(e) => startRowResize(e, i)}
    ></div>
  {/each}
</div>

<style>
  .resizable-grid {
    display: grid;
    /* initial column/row templates are set via JS using `style` attribute */
    /* gap: 10px; Optional gap between grid items */
    position: relative; /* Needed for resizer positioning if you want them on the lines */
    box-sizing: border-box; /* Include padding and border in sizing */
  }

  .resizer {
    background-color: rgba(0, 0, 0, 0.1); /* Subtle color for visibility */
    position: absolute; /* Position resizers over grid lines */
    z-index: 10; /* Ensure resizers are above content */
  }

  .resizer-col {
    width: 8px; /* Width of the column resizer */
    height: 100%;
    cursor: ew-resize; /* Horizontal resize cursor */
    /* Position exactly on the grid line */
    left: var(--resizer-left, 0); /* Dynamic positioning */
    transform: translateX(-50%); /* Center on the grid line */
  }

  .resizer-row {
    height: 8px; /* Height of the row resizer */
    width: 100%;
    cursor: ns-resize; /* Vertical resize cursor */
    /* Position exactly on the grid line */
    top: var(--resizer-top, 0); /* Dynamic positioning */
    transform: translateY(-50%); /* Center on the grid line */
  }

  /*
      Because resizers are positioned absolutely, their grid-column/grid-row
      properties control their position relative to the grid itself, but then
      `left`/`top` with `transform` fine-tune it to sit on the line.
      The `grid-column: {i + 1} / {i + 2}` etc. ensures they span the correct track.
      For more precise positioning, you might need to adjust their `left`/`top`
      based on the computed pixel widths of the grid tracks.
  
      However, for simpler grids, using `grid-column: X / Y` and `grid-row: X / Y`
      on the resizer itself (without absolute positioning and `left`/`top`)
      and making it a full-fledged grid item might be easier.
      This current setup places the resizer *between* the grid cells.
    */

  /* To place resizers directly on grid lines when display: grid is used */
  .resizer.resizer-col {
    grid-row: 1 / -1; /* Spans all rows */
    /* The column position is set dynamically by grid-column property in JS */
  }
  .resizer.resizer-row {
    grid-column: 1 / -1; /* Spans all columns */
    /* The row position is set dynamically by grid-row property in JS */
  }

  /* Hover effect for resizers */
  .resizer:hover {
    background-color: rgba(0, 0, 0, 0.3);
  }
</style>
