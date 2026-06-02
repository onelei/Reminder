<script lang="ts">
  import Toggle from "./Toggle.svelte";
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    description: string;
    checked?: boolean;
    showToggle?: boolean;
    onchange?: (checked: boolean) => void;
    children?: Snippet;
  }

  let {
    title,
    description,
    checked = false,
    showToggle = true,
    onchange,
    children,
  }: Props = $props();
</script>

<div class="card">
  <div class="row">
    <div class="text">
      <div class="title">{title}</div>
      <div class="desc">{description}</div>
    </div>
    {#if showToggle}
      <Toggle {checked} {onchange} />
    {/if}
  </div>
  {#if children}
    <div class="extra">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .card {
    background: #f5f8f9;
    border-radius: 14px;
    padding: 14px 16px;
    margin-bottom: 10px;
  }

  .row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .title {
    font-size: 15px;
    font-weight: 700;
    color: var(--primary);
    margin-bottom: 4px;
  }

  .desc {
    font-size: 12px;
    color: #7a9490;
    line-height: 1.5;
  }

  .extra {
    margin-top: 12px;
  }
</style>
