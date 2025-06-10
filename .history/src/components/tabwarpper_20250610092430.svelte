<!-- TabWrapper.svelte -->
<script>
    import { setContext, getContext } from 'svelte';
    import { writable } from 'svelte/store'; // For shared activeTabId state

    // --- Props ---
    // Optional custom class for the main wrapper div
    /** @type {string} */
    export let wrapperClass = '';

    // --- Internal State ---
    // A writable store to hold the ID of the currently active tab.
    // This is shared with child Tab components via context.
    const activeTabIdStore = writable(null);

    // An array to keep track of all registered tab IDs.
    // This is used by Tab components to register themselves.
    const registeredTabs = writable([]);

    // --- Context Setup for Children ---
    // Set context for child Tab components to register and interact.
    setContext('tabs_context', {
        // Allows a Tab component to register its ID and title
        registerTab: (id, title) => {
            registeredTabs.update(tabs => {
                if (!tabs.some(t => t.id === id)) { // Prevent duplicate registration
                    return [...tabs, { id, title }];
                }
                return tabs;
            });
            // Automatically activate the first registered tab if none is active
            activeTabIdStore.update(currentId => {
                if (currentId === null) {
                    return id;
                }
                return currentId;
            });
        },
        // Provides the active tab ID store so children can subscribe
        activeTabId: activeTabIdStore
    });

    // --- Derived State for Active Tab Content ---
    // This derived store helps in rendering the correct content based on activeTabIdStore.
    // The actual content rendering is done via named slots.
    // We only need to control which slot is active.
</script>

<div class="tab-wrapper {wrapperClass}">
    <!-- Slot for tab headers/buttons -->
    <div class="tab-headers flex space-x-1 p-1 bg-gray-100 rounded-lg mb-4">
        {#each $registeredTabs as tab (tab.id)}
            <button
                class="px-4 py-2 text-sm font-medium rounded-md transition-colors duration-200
                       {#if $activeTabIdStore === tab.id}
                           bg-white shadow text-blue-700
                       {:else}
                           text-gray-600 hover:bg-gray-200
                       {/if}"
                on:click={() => activeTabIdStore.set(tab.id)}
            >
                {tab.title}
            </button>
        {/each}
    </div>

    <!-- Slot for tab content. Content for active tab will be rendered here. -->
    <div class="tab-content border border-gray-200 p-4 rounded-lg bg-white">
        <slot {activeTabIdStore}></slot> {#comment} This slot passes the activeTabIdStore down to children so they can conditionally render their content. Alternatively, a simpler approach is for the wrapper to just provide the ID, and children manage their own rendering. Here, we pass the store. {/comment}
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
