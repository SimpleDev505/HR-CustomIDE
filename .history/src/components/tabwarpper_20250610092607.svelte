<!-- TabWrapper.svelte -->
<script>
  import { setContext, getContext } from "svelte";
  import { writable } from "svelte/store";

  /** @type {string} */
  export let wrapperClass = "";

  const activeTabIdStore = writable(null);

  const registeredTabs = writable([]);
  setContext("tabs_context", {
    registerTab: (id, title) => {
      registeredTabs.update((tabs) => {
        if (!tabs.some((t) => t.id === id)) {
          // Prevent duplicate registration
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
</script>

<div class="tab-wrapper {wrapperClass}">
  <div class="tab-headers">
    {#each $registeredTabs as tab (tab.id)}
      <button onclick={() => activeTabIdStore.set(tab.id)}>
        {tab.title}
      </button>
    {/each}
  </div>

  <div class="tab-content border border-gray-200 p-4 rounded-lg bg-white">
    <slot {activeTabIdStore}></slot>
  </div>
</div>

<style>
  .tab-wrapper {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
  }
</style>
