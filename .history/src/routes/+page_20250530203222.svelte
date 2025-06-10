<script lang="ts">
  import Applayout from "../components/applayout.svelte";
</script>

<main class="container">
  <Applayout></Applayout>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    /* Theme */
    --black-bg: #302b27;
    --blue-bg: #20a4f3;
    --white-bg: #f5f3f5;
    --grey-bg: #252525;
    --red-bg: #e3170a;

    --font-sm: 12px;
    --font-mm: 16px;
    --font-lg: 18px;

    /* border-top: 1px solid var(--grey-bg); */
    /* border-top: 2px solid rgba(255, 255, 255, .1); */
  }

  .container {
    margin: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    width: 100%;
    height: 100%;
    color: #ffffff;
    background-color: var(--grey-bg);
  }
</style>
