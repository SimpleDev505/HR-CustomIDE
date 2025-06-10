<script lang="ts">
  import Applayout from "../components/applayout.svelte";
</script>
<style>
  @import url('https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300..800;1,300..800&family=Victor+Mono:ital,wght@0,100..700;1,100..700&display=swap');
  </style>
<main class="container">
  <Applayout></Applayout>
</main>

<style>
  :root {
    /* font-family: Inter, Avenir, Helvetica, Arial, sans-serif; */
    font-family: "Victor Mono";
    font-weight: 400;
    /* Theme */
    --black-bg: #302b27;
    --blue-bg: #20a4f3;
    --white-bg: #f5f3f5;
    --grey-bg: #252525;
    --red-bg: #e3170a;

    --font-sm: 13px;
    --font-mm: 16px;
    --font-lg: 18px;
    cursor: default;

    /* border-top: 1px solid var(--grey-bg); */
    /* border-top: 2px solid rgba(255, 255, 255, .1); */
  }

  .container {
    margin: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    color: #ffffff;
    background-color: var(--grey-bg);
  }
</style>
