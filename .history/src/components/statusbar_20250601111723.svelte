<script>
  import { current_lang, line_cloumn, warn_error } from "../lib/store";
  import Resizervertical from "./resizervertical.svelte";
</script>

<Resizervertical
  initialHeightPx={208}
  maximumHeightPx={208}
  minimumHeightPx={32}
  position="top"
>
  <div class="status-cnt">
    <ul class="status-info">
      <li>{$warn_error}</li>
      <li id="status">
        <span>{$line_cloumn}</span>
        <span>UTF-8</span>
        <span>{$current_lang}</span>
      </li>
    </ul>
  </div>
</Resizervertical>

<style>
  .status-cnt {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-end;
    grid-column: 1/3;
    grid-row: 3;
    width: 100%;
    /* min-height: 2em;
    max-height: 13em; */
    padding-left: 15px;
    border-top: 2px solid rgba(255, 255, 255, 0.1);
    resize: vertical;
    overflow: hidden;
    direction: rtl;
  }
  .status-info {
    width: 100%;
    height: fit-content;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    column-gap: 15px;
    font-size: var(--font-sm);
    padding-left: 14px;
    padding-right: 14px;
    list-style-type: none;
    margin: 0;
    padding-bottom: 5px;
    direction: ltr;
  }
  #status {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    width: fit-content;
    height: fit-content;
    column-gap: 14px;
  }
</style>
