<script lang="ts">
  interface Props {
    options: { value: string; label: string }[];
    value: string;
    onchange?: (value: string) => void;
  }

  let { options, value, onchange }: Props = $props();
</script>

<div class="tabs">
  {#each options as option}
    <button
      type="button"
      class="tab"
      class:active={value === option.value}
      onclick={() => onchange?.(option.value)}
    >
      {option.label}
    </button>
  {/each}
</div>

<style>
  .tabs {
    display: flex;
    background: #eef2f4;
    border-radius: 20px;
    padding: 3px;
    gap: 2px;
  }

  .tab {
    border: none;
    background: transparent;
    color: var(--primary);
    font-size: 13px;
    font-weight: 600;
    padding: 6px 14px;
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab.active {
    background: var(--primary);
    color: white;
  }
</style>
