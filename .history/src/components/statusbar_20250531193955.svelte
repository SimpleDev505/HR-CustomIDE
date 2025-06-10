<div class="status-cnt">
  <div class="status-info">
    <span>Wn 0 Er 0</span>
    <span>Ln 0,Col 0</span>
    <span>UTF-8</span>
    <span>lang</span>
  </div>
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
    height: 100%;
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: center;
    column-gap: 15px;
    font-size: var(--font-sm);
    padding-left: 14px;
    padding-right: 14px;
  }
  .status-info span:first-child {
    float: left;
  }
</style>
