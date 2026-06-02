<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { t } from "$lib/i18n.svelte";

  interface Props {
    title?: string;
    showBack?: boolean;
    showMenu?: boolean;
    showClose?: boolean;
    onback?: () => void;
    onmenu?: () => void;
  }

  let {
    title = t("appTitle"),
    showBack = false,
    showMenu = false,
    showClose = false,
    onback,
    onmenu,
  }: Props = $props();

  async function startDrag(e: MouseEvent) {
    if ((e.target as HTMLElement).closest("button")) return;
    await getCurrentWindow().startDragging();
  }

  async function hideToTray() {
    await getCurrentWindow().hide();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="titlebar" onmousedown={startDrag}>
  <div class="left">
    {#if showBack}
      <button type="button" class="icon-btn" aria-label={t("back")} onclick={() => onback?.()}>
        ‹
      </button>
    {/if}
    {#if title}
      <span class="title">{title}</span>
    {/if}
  </div>
  <div class="right">
    {#if showMenu}
      <button type="button" class="icon-btn menu" aria-label={t("menu")} onclick={() => onmenu?.()}>
        ▾
      </button>
    {/if}
    {#if showClose}
      <button type="button" class="icon-btn close" aria-label={t("minimizeToTray")} onclick={hideToTray}>
        ×
      </button>
    {/if}
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 18px 8px;
    user-select: none;
    cursor: default;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .right {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .title {
    font-size: 22px;
    font-weight: 800;
    color: var(--primary);
    letter-spacing: 1px;
  }

  .icon-btn {
    border: none;
    background: transparent;
    color: var(--primary);
    font-size: 24px;
    line-height: 1;
    cursor: pointer;
    padding: 0 4px;
  }

  .menu {
    font-size: 18px;
    padding-top: 2px;
  }

  .close {
    font-size: 26px;
    font-weight: 300;
    line-height: 0.9;
  }

  .close:hover {
    opacity: 0.7;
  }
</style>
