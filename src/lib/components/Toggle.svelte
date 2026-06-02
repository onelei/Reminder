<script lang="ts">
  interface Props {
    checked: boolean;
    disabled?: boolean;
    onchange?: (checked: boolean) => void;
  }

  let { checked, disabled = false, onchange }: Props = $props();
</script>

<button
  type="button"
  class="toggle"
  class:on={checked}
  {disabled}
  aria-pressed={checked}
  aria-label="开关"
  onclick={() => onchange?.(!checked)}
>
  <span class="knob"></span>
</button>

<style>
  .toggle {
    width: 44px;
    height: 24px;
    border-radius: 12px;
    border: none;
    background: #d8dde3;
    position: relative;
    cursor: pointer;
    padding: 0;
    transition: background 0.2s;
    flex-shrink: 0;
  }

  .toggle.on {
    background: var(--primary);
  }

  .toggle:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .knob {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: white;
    transition: transform 0.2s;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
  }

  .toggle.on .knob {
    transform: translateX(20px);
  }
</style>
