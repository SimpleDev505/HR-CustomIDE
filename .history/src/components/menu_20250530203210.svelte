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
    height: 1.8em;
    padding-left: 15px;
    border-bottom: 2px solid rgba(255, 255, 255, 0.1);
  }
  .menu-tools {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
    align-items: center;
    justify-content: flex-start;
  }
  .menu-tools:first-child {
    margin-right: 10px;
  }
  .menu-nav {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    width: 100%;
    height: 100%;
    list-style-type: none;
    text-decoration: none;
  }
</style>
