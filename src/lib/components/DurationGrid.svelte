<script lang="ts">
  interface Props {
    presets: number[];
    selected: number;
    unit?: string;
    customPlaceholder?: string;
    onselect?: (minutes: number) => void;
    customValue?: string;
    oncustomChange?: (value: string) => void;
    oncustomConfirm?: () => void;
    showCustom?: boolean;
  }

  let {
    presets,
    selected,
    unit = "min",
    customPlaceholder = "Custom",
    onselect,
    customValue = "",
    oncustomChange,
    oncustomConfirm,
    showCustom = false,
  }: Props = $props();
</script>

<div class="grid">
  {#each presets as minutes}
    <button
      type="button"
      class="preset"
      class:selected={selected === minutes}
      onclick={() => onselect?.(minutes)}
    >
      {minutes}{unit}
    </button>
  {/each}
</div>

{#if showCustom}
  <div class="custom">
    <input
      type="number"
      min="1"
      max="180"
      placeholder={customPlaceholder}
      value={customValue}
      oninput={(e) => oncustomChange?.((e.currentTarget as HTMLInputElement).value)}
    />
    <button type="button" class="confirm" onclick={() => oncustomConfirm?.()}>✓</button>
  </div>
{/if}

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .preset {
    border: none;
    background: var(--primary-soft);
    color: var(--primary);
    border-radius: 10px;
    padding: 10px 6px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
  }

  .preset.selected {
    background: var(--primary);
    color: white;
  }

  .custom {
    display: flex;
    gap: 8px;
    margin-top: 10px;
  }

  .custom input {
    flex: 1;
    border: none;
    background: #eef2f4;
    border-radius: 10px;
    padding: 10px 12px;
    font-size: 14px;
    color: #555;
    outline: none;
  }

  .confirm {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    border: none;
    background: var(--primary);
    color: white;
    font-size: 16px;
    cursor: pointer;
  }
</style>
