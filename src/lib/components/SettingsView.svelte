<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SettingCard from "$lib/components/SettingCard.svelte";
  import DurationGrid from "$lib/components/DurationGrid.svelte";
  import ColorPicker from "$lib/components/ColorPicker.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import { BREAK_PRESETS, type AppConfig, type AppLocale } from "$lib/types";
  import { applyThemeColor } from "$lib/utils";
  import { localeStore, setLocale, t } from "$lib/i18n.svelte";

  interface Props {
    config: AppConfig;
    onback: () => void;
    onchange: (config: AppConfig) => void;
  }

  let { config, onback, onchange }: Props = $props();

  let breakCustom = $state("");

  async function update(partial: Partial<AppConfig>) {
    const next = { ...config, ...partial };
    if (next.customThemeColor) {
      applyThemeColor(next.themeColor);
    } else {
      applyThemeColor("#2aab9b");
    }
    if (partial.locale) {
      setLocale(partial.locale);
    }
    await invoke("save_config", { config: next });
    onchange(next);
  }

  async function pickBackground() {
    const path = await invoke<string | null>("pick_break_background");
    if (path) {
      await update({ breakBackgroundPath: path, customBreakBackground: true });
    }
  }

  async function setLanguage(next: AppLocale) {
    await update({ locale: next });
  }
</script>

<div class="settings">
  <TitleBar title={t("settings")} showBack {onback} />

  <div class="content">
    <section>
      <h3>{t("breakDuration")}</h3>
      <DurationGrid
        presets={BREAK_PRESETS}
        selected={config.breakMinutes}
        unit={t("minutes")}
        customPlaceholder={t("custom")}
        onselect={(m) => update({ breakMinutes: m })}
        showCustom
        customValue={breakCustom}
        oncustomChange={(v) => (breakCustom = v)}
        oncustomConfirm={() => {
          const n = parseInt(breakCustom, 10);
          if (n > 0) update({ breakMinutes: n });
        }}
      />
    </section>

    <SettingCard
      title={t("language")}
      description={t("languageDesc")}
      showToggle={false}
    >
      <div class="lang-row">
        <button
          type="button"
          class="lang-btn"
          class:active={localeStore.current === "zh"}
          onclick={() => setLanguage("zh")}
        >
          {t("langZh")}
        </button>
        <button
          type="button"
          class="lang-btn"
          class:active={localeStore.current === "en"}
          onclick={() => setLanguage("en")}
        >
          {t("langEn")}
        </button>
      </div>
    </SettingCard>

    <SettingCard
      title={t("customBreakBg")}
      description={t("customBreakBgDesc")}
      checked={config.customBreakBackground}
      onchange={(v) => update({ customBreakBackground: v })}
    >
      {#if config.customBreakBackground}
        <button type="button" class="pick-btn" onclick={pickBackground}>
          {config.breakBackgroundPath ? t("changeBg") : t("pickBg")}
        </button>
      {/if}
    </SettingCard>

    <SettingCard
      title={t("customTheme")}
      description={t("customThemeDesc")}
      checked={config.customThemeColor}
      onchange={(v) => update({ customThemeColor: v })}
    >
      {#if config.customThemeColor}
        <ColorPicker value={config.themeColor} onchange={(c) => update({ themeColor: c })} />
      {/if}
    </SettingCard>

    <SettingCard
      title={t("autostart")}
      description={t("autostartDesc")}
      checked={config.autostart}
      onchange={(v) => update({ autostart: v })}
    />
  </div>
</div>

<style>
  .settings {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 0 18px 20px;
  }

  section {
    margin-bottom: 18px;
  }

  h3 {
    font-size: 15px;
    font-weight: 700;
    color: var(--primary);
    margin: 0 0 10px;
  }

  .pick-btn {
    width: 100%;
    border: 1px dashed var(--primary);
    background: white;
    color: var(--primary);
    border-radius: 10px;
    padding: 10px;
    cursor: pointer;
    font-size: 13px;
  }

  .lang-row {
    display: flex;
    gap: 8px;
  }

  .lang-btn {
    flex: 1;
    border: 1.5px solid var(--primary-soft);
    background: white;
    color: var(--primary);
    border-radius: 10px;
    padding: 10px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .lang-btn.active {
    border-color: var(--primary);
    background: var(--primary);
    color: white;
  }
</style>
