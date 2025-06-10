<script lang="ts">
  import {
    dropdown_enabled,
    projectStore,
    type ProjectMetadata,
  } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import Dropdownmenu from "./dropdownmenu.svelte";
  import type { MenuItem } from "$lib/store";
  import { onMount } from "svelte";
  // let dropdown_enable = $state(false);
  let current_dropdownitems: MenuItem[] = $state([]);
  let last_clickedmenuitem: string = $state("");
  let current_dropdownoffset: number = $state(10);
  function default_action() {
    console.log("No Action Specified For Menu Item!");
  }
  let file_menu_items: MenuItem[] = [
    {
      label: "New File",
      action: default_action,
      split: false,
    },
    {
      label: "New Project",
      action: createProject,
      split: true,
    },
    {
      label: "Open File",
      action: default_action,
      split: false,
    },
    {
      label: "Open Project",
      action: default_action,
      split: false,
    },
  ];
  let edit_menu_items = [
    {
      label: "Find",
      action: default_action,
      split: false,
    },
    {
      label: "Find In Project",
      action: default_action,
      split: false,
    },
  ];
  let settings_menu_items = [
    {
      label: "Project Settings",
      action: default_action,
      split: false,
    },
    {
      label: "Editor Settings",
      action: default_action,
      split: false,
    },
  ];
  let help_menu_items = [
    {
      label: "Check For Updates",
      action: default_action,
      split: false,
    },
    {
      label: "About",
      action: default_action,
      split: false,
    },
  ];
  function clickOption(name: string) {
    if (last_clickedmenuitem == name) {
      $dropdown_enabled = false;
      last_clickedmenuitem = "";
    } else {
      $dropdown_enabled = true;
      last_clickedmenuitem = name;
    }
    console.log($dropdown_enabled);
    switch (name) {
      case "file":
        current_dropdownitems = file_menu_items;
        current_dropdownoffset = 60;
        break;
      case "edit":
        current_dropdownitems = edit_menu_items;
        current_dropdownoffset = 120;
        break;
      case "settings":
        current_dropdownitems = settings_menu_items;
        current_dropdownoffset = 180;
        break;
      case "help":
        current_dropdownitems = help_menu_items;
        current_dropdownoffset = 260;
        break;
    }
  }
  //Reset DropDown
  function resetMenu() {
    $dropdown_enabled = false;
  }
  //File
  async function createProject() {
    resetMenu();
    const result = await invoke<ProjectMetadata>("create_project", {});
    projectStore.set(result);
  }
</script>

<div class="menu-cnt">
  <div class="menu-tools">
    <span>HR</span>
    <ul class="menu-nav">
      <li>
        <button
          onclick={() => {
            clickOption("file");
          }}>File</button
        >
      </li>
      <li>
        <button
          onclick={() => {
            clickOption("edit");
          }}>Edit</button
        >
      </li>
      <li>
        <button
          onclick={() => {
            clickOption("settings");
          }}>Settings</button
        >
      </li>
      <li>
        <button
          onclick={() => {
            clickOption("help");
          }}>Run</button
        >
      </li>
    </ul>

    {#if $dropdown_enabled}
      <Dropdownmenu
        leftpx={current_dropdownoffset}
        items={current_dropdownitems}
      ></Dropdownmenu>
    {/if}
  </div>
</div>

<style>
  .menu-cnt {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    grid-column: 1/3;
    grid-row: 1;
    width: 100%;
    height: 2.5em;
    padding: 0;
    margin: 0;
    padding-left: 15px;
    border-bottom: 2px solid rgba(255, 255, 255, 0.1);
    font-size: var(--font-sm);
    cursor: default;
  }
  .menu-tools {
    display: flex;
    flex-direction: row;
    width: fit-content;
    height: 10px;
    align-items: center;
    justify-content: flex-start;
  }
  .menu-tools span {
    margin-right: 15px;
    font-weight: bold;
    padding: 4px;
    background-color: var(--blue-bg);
    border-radius: 5px;
  }
  .menu-nav {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    column-gap: 15px;
    height: 10px;
    padding: 0;
    margin: 0;
    list-style-type: none;
    text-decoration: none;
  }
  .menu-nav li {
    padding: 4px;
    border-radius: 5px;
    transition: 0.25s;
  }
  .menu-nav li:hover {
    background-color: rgba(255, 255, 255, 0.2);
    /* outline: 1px solid white; */
  }
</style>
