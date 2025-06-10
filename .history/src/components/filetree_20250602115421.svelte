<script>
  import Resizerhorizontal from "./resizerhorizontal.svelte";
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
    <div class="filetree-files"></div>
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
  }
</style>
