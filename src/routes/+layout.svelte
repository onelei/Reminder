<script lang="ts">
  import { onMount } from "svelte";
  import { applyThemeColor } from "$lib/utils";

  let { children } = $props();

  onMount(() => {
    applyThemeColor("#2aab9b");
  });
</script>

{@render children()}

<style>
  :global(:root) {
    --primary: #2aab9b;
    --primary-light: #2aab9b22;
    --primary-soft: #2aab9b18;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
    background: white;
    color: #333;
    overflow: hidden;
  }
</style>
