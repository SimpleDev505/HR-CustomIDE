<script lang="ts">
  import { projectStore, type ProjectMetadata } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import Dropdownmenu from "./dropdownmenu.svelte";
  import type { MenuItem } from "$lib/store";
  let dropdown_file = $state(false);
  let dropdown_edit = $state(false);
  let dropdown_settings = $state(false);
  let dropdown_run = $state(false);
  let current_dropdownitems = $state([]);
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
      label: "New Project".to_string(),
      action: new_project,
      split: true,
    },
    {
      label: "Open File".to_string(),
      action: open_file,
      split: false,
    },
    {
      label: "Open Project".to_string(),
      action: open_project,
      split: false,
    },
  ];
  let edit_menu_items = [
    {
      label: "Find".to_string(),
      action: new_file,
      split: false,
    },
    {
      label: "Find In Project".to_string(),
      action: new_file,
      split: false,
    },
  ];
  let settings_menu_items = [
    {
      label: "Project Settings".to_string(),
      action: new_file,
      split: false,
    },
    {
      label: "Editor Settings".to_string(),
      action: new_file,
      split: false,
    },
  ];
  let help_menu_items = [
    {
      label: "Check For Updates".to_string(),
      action: new_file,
      split: false,
    },
    {
      label: "About".to_string(),
      action: new_file,
      split: false,
    },
  ];
  function clickOption(name: string) {
    switch (name) {
      case "file":
        dropdown_file = true;
        current_dropdownitems =
          //
          dropdown_edit = false;
        dropdown_settings = false;
        dropdown_run = false;

        break;
      case "edit":
        dropdown_edit = true;
        //
        dropdown_file = false;
        dropdown_settings = false;
        dropdown_run = false;
        break;
      case "settings":
        dropdown_settings = true;
        //
        dropdown_file = false;
        dropdown_edit = false;
        dropdown_run = false;
        break;
      case "run":
        dropdown_run = true;
        //
        dropdown_file = false;
        dropdown_settings = false;
        dropdown_edit = false;
        break;
    }
  }
  //File
  async function createProject() {
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
            clickOption("run");
          }}>Run</button
        >
      </li>
    </ul>
    <Dropdownmenu></Dropdownmenu>
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
