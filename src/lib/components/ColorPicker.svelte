<script lang="ts">
  import { THEME_COLORS } from "$lib/types";

  interface Props {
    colors?: typeof THEME_COLORS;
    value: string;
    onchange?: (value: string) => void;
  }

  let { colors = THEME_COLORS, value, onchange }: Props = $props();
</script>

<div class="grid">
  {#each colors as color}
    <button
      type="button"
      class="swatch"
      class:selected={value === color.value}
      style={`background:${color.value}`}
      aria-label={color.id}
      onclick={() => onchange?.(color.value)}
    >
      {#if value === color.value}
        <span class="check">✓</span>
      {/if}
    </button>
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 10px;
    margin-top: 12px;
  }

  .swatch {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    transition: transform 0.15s;
  }

  .swatch:hover {
    transform: scale(1.08);
  }

  .swatch.selected {
    border-color: rgba(255, 255, 255, 0.8);
    box-shadow: 0 0 0 2px var(--primary);
  }

  .check {
    color: white;
    font-size: 14px;
    font-weight: 700;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
  }
</style>
