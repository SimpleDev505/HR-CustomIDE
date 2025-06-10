<div class="filetree-cnt">
  <div class="file-tools">
    <span>EXPLORER</span>
    <button><img src="/icons/new_file.png" alt="newfile" /></button>
    <button><img src="/icons/new_folder.png" alt="newfolder" /></button>
  </div>
</div>

<style>
  .filetree-cnt {
    display: flex;
    flex-direction: column;
    justify-content: baseline;
    align-items: flex-start;
    grid-column: 1;
    grid-row: 2;
    width: 16em;
    height: 100%;
    padding-left: 25px;
    border-right: 2px solid rgba(255, 255, 255, 0.1);
  }
  .file-tools {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    margin-top: 15px;
    font-size: 12px;
    opacity: 0.5;
    padding: 0;
    margin: 0;
  }
  .file-tools button {
    width: fit-content;
    height: fit-content;
    background-color: transparent;
    border: none;
  }
  .file-tools img {
    object-fit: cover;
    width: 20px;
    height: 20px;
  }
  .file-tools img:hover {
    scale: 1.05;
    background-color: rgba(255, 255, 255, 0.2);
  }
</style>
