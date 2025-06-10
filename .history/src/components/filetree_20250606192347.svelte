<script lang="ts">
  import {
    APP_STATE,
    Fetch_APPSTATE,
    FileTreeMenu_Enabled,
    OPEN_TABS,
    projectStore,
    SelectedFile_Path,
    type AppState,
    type MenuItem,
  } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import Resizerhorizontal from "./resizerhorizontal.svelte";
  import { onMount, tick } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Dropdownmenu from "./dropdownmenu.svelte";
  import { get } from "svelte/store";

  //States
  let selectedfile_name: String = $state("");
  let selectedfile_renamebox: HTMLInputElement | null = $state(null);
  let renaming = $state(false);

  //On Setup
  onMount(() => {
    const filewatcher_appstate = listen<AppState>("app_state_updated", () => {
      Fetch_APPSTATE();
      if (
        $SelectedFile_Path &&
        !$APP_STATE.file_tree.some((file) => file.path === $SelectedFile_Path)
      ) {
        $SelectedFile_Path = "";
      }
      SetRenameFile(false);
    });
    return () => {
      filewatcher_appstate.then((f) => f()); // Clean up listener
    };
  });

  //Functions
  function getfile_Icon(name: String): String {
    switch (name) {
      case "txt":
        return "/icons/lang/text.png";
      case "py":
        return "/icons/lang/python.png";
      default:
        return "/icons/lang/unknown.png";
    }
  }
  async function RenameFile(path: String) {
    //Commit Rename
    if (path == "") {
      return;
    }
    let result: AppState = await invoke("rename_file_or_dir", {
      old_relative_path: path,
      new_name: selectedfile_name,
    });
    if (result) {
      SetRenameFile(false);
    }
  }
  function setSelectedFile(path: String) {
    if (!renaming) {
      renaming = true;
      $SelectedFile_Path = path;
    }
  }
  async function SetRenameFile(rename: boolean) {
    if (renaming && rename) {
      return;
    }
    if (rename == false) {
      setSelectedFile("");
      selectedfile_name = "";
      renaming = false;
    } else {
      await tick();
      if (selectedfile_renamebox) {
        let lastDotIndex = selectedfile_renamebox.textContent?.lastIndexOf(".");
        if (lastDotIndex !== undefined && lastDotIndex !== -1) {
          selectedfile_renamebox.setSelectionRange(0, lastDotIndex);
        } else {
          selectedfile_renamebox.select();
        }
        selectedfile_renamebox.focus();
        renaming = true;
      }
    }
  }

  //Global Events
  document.addEventListener("keydown", () => SetRenameFile(true));
  document.addEventListener("click", () => SetRenameFile(false));

  //Filetree Menu
  let filetree_menu_pos_x: number = $state(0);
  let filetree_menu_pos_y: number = $state(0);
  let filetree_menu_items: MenuItem[] = [
    {
      label: "Edit File",
      action: () => console.log(""),
      split: false,
    },
  ];
  function HandleFileTreeMenu(e: MouseEvent) {
    e.preventDefault();
    filetree_menu_pos_x = e.clientX;
    filetree_menu_pos_y = e.clientY;
    $FileTreeMenu_Enabled = true;
  }
  function OnFocusRemoved() {
    $FileTreeMenu_Enabled = false;
  }
</script>

<!-- File Tree -->
<Resizerhorizontal
  initialWidthPx={256}
  maximumWidthPx={256}
  minimumWidthPx={192}
  position="right"
>
  <div
    id="filetree-cnt"
    onmousedown={OnFocusRemoved}
    oncontextmenu={HandleFileTreeMenu}
    onmouseleave={OnFocusRemoved}
    role="tabpanel"
    tabindex="999"
  >
    <div class="file-tools">
      <span>EXPLORER</span>
      <button><img src="/icons/new_file.png" alt="newfile" /></button>
      <button><img src="/icons/new_folder.png" alt="newfolder" /></button>
    </div>
    <div class="filetree-files">
      {#if $APP_STATE && $APP_STATE.file_tree.length > 0}
        {#each $APP_STATE.file_tree as file}
          <div
            role="button"
            tabindex="999"
            ondblclick={() => {
              if (!file.is_dir && !renaming) {
                setSelectedFile(file.path);
                selectedfile_name = file.name;
              }
            }}
            onmousedown={() => {
              if (!renaming && !file.is_dir) {
                OPEN_TABS.update((tabs) => {
                  const file_exists = tabs.some(
                    (tab) => tab.tab_name === file.name
                  );
                  if (!file_exists) {
                    return [
                      ...tabs,
                      {
                        tab_name: file.name,
                        tab_lang: file.language,
                        tab_path: file.path,
                      },
                    ];
                  }
                  return tabs;
                });
              }
            }}
            class="file-cnt {file.is_dir ? 'file-directory' : 'file'}"
          >
            <span
              style="display: flex; flex-direction: row;justify-content:center;align-items:center;column-gap:12px;"
            >
              {#if !file.is_dir}
                <img
                  src={getfile_Icon(file.language).toString()}
                  alt="file_icon"
                />
              {/if}
              <div
                role="button"
                tabindex="999"
                onkeydown={(e) => {
                  if (e.key === "F2") {
                    e.preventDefault();
                    if (file.path === $SelectedFile_Path) {
                      console.log(file.path);
                      SetRenameFile(true);
                    }
                  }
                }}
              >
                {#if file.path != $SelectedFile_Path}
                  {file.name}
                {:else}
                  <input
                    id="renaming-box"
                    bind:this={selectedfile_renamebox}
                    bind:value={selectedfile_name}
                    type="text"
                    onkeydown={(e) => {
                      if (e.key === "Escape") {
                        e.preventDefault();
                        if (file.path === $SelectedFile_Path) {
                          console.log(file.path);
                          SetRenameFile(false);
                        }
                      } else if (e.key === "Enter") {
                        e.preventDefault();
                        if (file.path === $SelectedFile_Path) {
                          RenameFile(file.path);
                        }
                      }
                    }}
                  />
                {/if}
              </div>
            </span>
            {#if !file.is_dir}
              <span class="file-lang">{file.language}</span>
            {/if}
          </div>
        {/each}
      {/if}
    </div>
    {#if $FileTreeMenu_Enabled}
      <Dropdownmenu
        rightpx={filetree_menu_pos_y}
        leftpx={filetree_menu_pos_x}
        items={filetree_menu_items}
      ></Dropdownmenu>
    {/if}
  </div>
</Resizerhorizontal>

<style>
  #filetree-cnt {
    display: flex;
    flex-direction: column;
    justify-content: baseline;
    align-items: flex-start;
    grid-column: 1;
    grid-row: 2;
    /* max-width: 16em;
    min-width: 10em; */
    width: 100%;
    height: 100%;
    padding-left: 25px;
    border-right: 2px solid rgba(255, 255, 255, 0.1);
  }
  .file-tools {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    column-gap: 5px;
    margin-top: 15px;
    font-size: 12px;
    opacity: 0.5;
    padding: 0;
  }
  .file-tools button {
    display: flex;
    justify-content: center;
    align-items: center;
    width: fit-content;
    height: fit-content;
    background-color: transparent;
    border: none;
    padding: 0;
    margin: 0;
  }
  .file-tools img {
    object-fit: cover;
    width: 20px;
    height: 20px;
    cursor: pointer;
    user-select: none;
  }
  .file-tools img:hover {
    scale: 1.05;
    background-color: rgba(255, 255, 255, 0.2);
  }
  /* FileTree Files */
  .filetree-files {
    display: flex;
    justify-content: baseline;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
    height: 100%;
    row-gap: 10px;
    font-size: var(--font-sm);
  }
  .file-directory {
    margin-top: 10px;
    text-transform: uppercase;
    font-weight: bold;
  }
  .file {
    margin-top: 10px;
  }
  .file-cnt {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: fit-content;
    justify-content: space-between;
    align-items: center;
    padding-left: 10px;
  }
  .file-cnt img {
    object-fit: cover;
    width: 19px;
    height: 19px;
    margin: 0;
    padding: 0;
  }
  .file-cnt:hover .file-lang {
    opacity: 1;
  }
  .file-cnt:hover {
    background-color: rgba(125, 125, 125, 0.15);
  }
  .file-lang {
    padding: 4px;
    padding-left: 8px;
    padding-right: 8px;
    background-color: rgba(255, 255, 255, 0.05);
    opacity: 0;
    transition: 0.1s ease-in-out;
    user-select: none;
  }
</style>
