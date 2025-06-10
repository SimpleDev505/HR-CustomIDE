<script lang="ts">
  import { APP_STATE } from "$lib/store";
  import Resizerhorizontal from "./resizerhorizontal.svelte";
  let cache_fileicon = $state("");
  function getfile_Icon(name: String): string {
    switch (name) {
      case "text":
        return "/icons/lang/text.png";
      case "python":
        return "/icons/lang/python.png";
      default:
        return "/icons/lang/unknown.png";
    }
  }
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
      {#each $APP_STATE.file_tree as file}
        <div class="file-cnt {file.is_dir ? 'file-directory' : 'file'}">
          {getfile_Icon}
          <img src={cache_fileicon} alt="file_icon" />
          <span>{file.name}</span>
          {#if !file.is_dir}
            <span class="file-lang">{file.language}</span>
          {/if}
        </div>
      {/each}
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
  }
</style>
