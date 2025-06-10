<script lang="ts">
  import { APP_STATE, projectStore, SelectedFile_Path } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";
  import Resizerhorizontal from "./resizerhorizontal.svelte";
  let cache_fileicon = $state("");
  let rename_file = $state(false);
  function getfile_Icon(name: String): String {
    switch (name) {
      case "text":
        return "/icons/lang/text.png";
      case "python":
        return "/icons/lang/python.png";
      default:
        return "/icons/lang/unknown.png";
    }
  }
  let selectedfile_name: String = $state("");
  let selectedfile_renamebox: HTMLInputElement | null = $state(null);
  let renaming = $state(false);
  async function RenameFile(path: String) {
    //Commit Rename
    let result = await invoke("rename_file_or_dir", {
      old_relative_path: path,
      new_name: selectedfile_name,
    });
  }
  function setSelectedFile(path: String) {
    if (!renaming) {
      $SelectedFile_Path = path;
      console.log($SelectedFile_Path);
    }
  }
  function SetRenameFile(rename: boolean) {
    if (rename == false) {
      setSelectedFile("");
      selectedfile_name = "";
      renaming = false;
    } else {
      if (selectedfile_renamebox) {
        let lastDotIndex = selectedfile_renamebox.textContent?.lastIndexOf(".");
        if (lastDotIndex) {
          selectedfile_renamebox.setSelectionRange(0, lastDotIndex);
        } else {
          selectedfile_renamebox.select();
        }
        renaming = true;
      }
    }
  }
  document.addEventListener("keydown", () => SetRenameFile(true));
  document.addEventListener("click", () => SetRenameFile(false));
</script>

<Resizerhorizontal
  initialWidthPx={256}
  maximumWidthPx={256}
  minimumWidthPx={192}
  position="right"
>
  <div id="filetree-cnt">
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
                    oninput={(e) => {
                      selectedfile_name = e;
                    }}
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
