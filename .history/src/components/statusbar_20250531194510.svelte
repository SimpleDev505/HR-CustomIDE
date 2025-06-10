<div class="status-cnt">
  <ul class="status-info">
    <li>Wn 0 Er 0</li>
    <li>Ln 0,Col 0</li>
    <li>UTF-8</li>
    <li>lang</li>
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
    height: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    column-gap: 15px;
    font-size: var(--font-sm);
    padding-left: 14px;
    padding-right: 14px;
    list-style-type: none;
  }
  .status-info :not(:first-child) {
    text-align: right;
  }
</style>
