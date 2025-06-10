<!-- TabWrapper.svelte -->
<script lang="ts">
  import { setContext } from "svelte";
  import { writable, type Writable } from "svelte/store"; // Import type Writable

  // --- Props ---
  // Use $props() for component props in Svelte 5 runes mode
  let { wrapperClass = "" }: { wrapperClass?: string } = $props();

  // --- Internal State ---
  // A writable store to hold the ID of the currently active tab.
  // This is shared with child Tab components via context.
  const activeTabIdStore: Writable<string | null> = writable(null);

  // An array to keep track of all registered tab IDs and their titles.
  // This is used by Tab components to register themselves.
  interface RegisteredTab {
    id: string;
    title: string;
  }
  const registeredTabs: Writable<RegisteredTab[]> = writable([]);

  // --- Context Setup for Children ---
  // Define the type for the context object
  interface TabsContext {
    registerTab: (id: string, title: string) => void;
    activeTabId: Writable<string | null>;
  }

  // Set context for child Tab components to register and interact.
  setContext<TabsContext>("tabs_context", {
    // Allows a Tab component to register its ID and title
    registerTab: (id: string, title: string) => {
      registeredTabs.update((tabs) => {
        if (!tabs.some((t) => t.id === id)) {
          return [...tabs, { id, title }];
        }
        return tabs;
      });
      // Automatically activate the first registered tab if none is active
      activeTabIdStore.update((currentId) => {
        if (currentId === null) {
          return id;
        }
        return currentId;
      });
    },
    // Provides the active tab ID store so children can subscribe
    activeTabId: activeTabIdStore,
  });

  // --- Note on Slot ---
  // The `<slot {activeTabIdStore}></slot>` syntax here is correct for
  // passing data down to direct children placed in the default slot.
  // The warning "Using <slot> to render parent content is deprecated"
  // typically refers to a child component *receiving* content via `<slot>`
  // which should now use `{@render children()}` or `{@render namedSlot()}`.
  // In this `TabWrapper`, we are *defining* a slot and *passing* a prop to it,
  // which is still a valid use case for this pattern.
</script>

<div class="tab-wrapper {wrapperClass}">
  <!-- Slot for tab headers/buttons -->
  <div class="tab-headers">
    {#each $registeredTabs as tab (tab.id)}
      <button
        class="tab-button"
        class:active-tab-button={$activeTabIdStore === tab.id}
        on:click={() => activeTabIdStore.set(tab.id)}
      >
        {tab.title}
      </button>
    {/each}
  </div>

  <!-- Slot for tab content. Content for active tab will be rendered here. -->
  <div class="tab-content">
    <slot {activeTabIdStore}></slot>
  </div>
</div>

<style>
  /* Default minimal styles for the wrapper */
  .tab-wrapper {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
    border-radius: 0.5rem; /* Equivalent to rounded-lg */
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06); /* Equivalent to shadow-lg */
    background-color: #ffffff; /* Equivalent to bg-white */
  }

  .tab-headers {
    display: flex;
    gap: 0.25rem; /* space-x-1 */
    padding: 0.25rem; /* p-1 */
    background-color: #f3f4f6; /* bg-gray-100 */
    border-radius: 0.5rem; /* rounded-lg */
    margin-bottom: 1rem; /* mb-4 */
  }

  .tab-button {
    padding: 0.5rem 1rem; /* px-4 py-2 */
    font-size: 0.875rem; /* text-sm */
    font-weight: 500; /* font-medium */
    border-radius: 0.375rem; /* rounded-md */
    transition-property: color, background-color, border-color,
      text-decoration-color, fill, stroke, opacity, box-shadow, transform,
      filter, backdrop-filter; /* transition-colors */
    transition-duration: 200ms; /* duration-200 */
    background-color: transparent; /* Default transparent background */
    border: none; /* No default button border */
    cursor: pointer;
    color: #4b5563; /* text-gray-600 */
  }

  .tab-button:hover {
    background-color: #e5e7eb; /* hover:bg-gray-200 */
  }

  .active-tab-button {
    background-color: #ffffff; /* bg-white */
    box-shadow:
      0 1px 3px 0 rgba(0, 0, 0, 0.1),
      0 1px 2px 0 rgba(0, 0, 0, 0.06); /* shadow */
    color: #2563eb; /* text-blue-700 */
  }

  .tab-content {
    border: 1px solid #e5e7eb; /* border border-gray-200 */
    padding: 1rem; /* p-4 */
    border-radius: 0.5rem; /* rounded-lg */
    background-color: #ffffff; /* bg-white */
  }
</style>
