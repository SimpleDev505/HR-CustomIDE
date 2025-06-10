<div class="status-cnt">
  <ul class="status-info">
    <li>Wn 0 Er 0</li>
    <li id="status">
      <span>Ln 0,Col 0</span>
      <span>UTF-8</span>
      <span>lang</span>
    </li>
  </ul>
</div>

<style>
  .status-cnt {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    grid-column: 1/3;
    grid-row: 3;
    height: 2em;
    padding-left: 15px;
    border-top: 2px solid rgba(255, 255, 255, 0.1);
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
  }
</style>
