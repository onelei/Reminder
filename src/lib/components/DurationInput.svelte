<script lang="ts">
  interface Props {
    value: number;
    min?: number;
    max?: number;
    onchange: (value: number) => void;
  }

  let { value, min = 1, max = 180, onchange }: Props = $props();

  let text = $state("");
  let composing = $state(false);
  let focused = $state(false);

  $effect(() => {
    if (!focused && !composing) {
      text = String(value);
    }
  });

  function digitsOnly(raw: string): string {
    return raw.replace(/\D/g, "");
  }

  function handleInput(e: Event) {
    const el = e.currentTarget as HTMLInputElement;
    if (composing) {
      text = el.value;
      return;
    }
    const next = digitsOnly(el.value);
    text = next;
    el.value = next;
  }

  function handleCompositionStart() {
    composing = true;
  }

  function handleCompositionEnd(e: CompositionEvent) {
    composing = false;
    const next = digitsOnly((e.currentTarget as HTMLInputElement).value);
    text = next;
    (e.currentTarget as HTMLInputElement).value = next;
  }

  function handleFocus() {
    focused = true;
  }

  function handleBlur() {
    focused = false;
    const n = parseInt(text, 10);
    if (Number.isNaN(n) || n <= 0) {
      text = String(value);
      return;
    }
    const clamped = Math.min(max, Math.max(min, n));
    text = String(clamped);
    if (clamped !== value) onchange(clamped);
  }
</script>

<input
  type="text"
  inputmode="numeric"
  autocomplete="off"
  spellcheck="false"
  value={text}
  onfocus={handleFocus}
  onblur={handleBlur}
  oninput={handleInput}
  oncompositionstart={handleCompositionStart}
  oncompositionend={handleCompositionEnd}
/>

<style>
  input {
    flex: 1;
    border: none;
    background: #eef2f4;
    border-radius: 10px;
    padding: 10px 12px;
    font-size: 14px;
    color: #333;
    outline: none;
    ime-mode: disabled;
  }

  input:focus {
    box-shadow: 0 0 0 2px var(--primary-soft);
  }
</style>
