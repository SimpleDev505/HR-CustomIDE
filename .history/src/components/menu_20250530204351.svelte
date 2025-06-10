<div class="menu-cnt">
  <div class="menu-tools">
    <span>HR</span>
    <ul class="menu-nav">
      <li>File</li>
      <li>Edit</li>
      <li>Settings</li>
      <li>Run</li>
    </ul>
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
    height: 3em;
    padding: 0;
    margin: 0;
    padding-left: 15px;
    border-bottom: 2px solid rgba(255, 255, 255, 0.1);
    font-size: var(--font-sm);
  }
  .menu-tools {
    display: flex;
    flex-direction: row;
    width: fit-content;
    height: 10px;
    align-items: center;
    justify-content: flex-start;
  }
  .menu-nav {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    column-gap: 15px;
    height: fit-content;
    list-style-type: none;
    text-decoration: none;
  }
</style>
