<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SettingCard from "$lib/components/SettingCard.svelte";
  import ColorPicker from "$lib/components/ColorPicker.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import { DEFAULT_CONFIG, type AppConfig, type AppLocale } from "$lib/types";
  import { applyThemeColor } from "$lib/utils";
  import { localeStore, setLocale, t } from "$lib/i18n.svelte";

  interface Props {
    config: AppConfig;
    onback: () => void;
    onchange: (config: AppConfig) => void;
  }

  let { config, onback, onchange }: Props = $props();

  let savedConfig = $state<AppConfig>({ ...DEFAULT_CONFIG });

  $effect(() => {
    savedConfig = { ...config };
  });

  let breakInput = $state(String(DEFAULT_CONFIG.breakMinutes));
  let workInput = $state(String(DEFAULT_CONFIG.workMinutes));

  $effect(() => {
    breakInput = String(savedConfig.breakMinutes);
    workInput = String(savedConfig.workMinutes);
  });

  async function update(partial: Partial<AppConfig>) {
    const prev = savedConfig;
    savedConfig = { ...savedConfig, ...partial };
    const next = savedConfig;

    if (next.customThemeColor) {
      applyThemeColor(next.themeColor);
    } else {
      applyThemeColor("#2aab9b");
    }
    if (partial.locale) {
      setLocale(partial.locale);
    }

    try {
      await invoke("save_config", { config: next });
      onchange(next);
    } catch (err) {
      savedConfig = prev;
      console.error("Failed to save config:", err);
    }
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

  function onWorkMinutesInput(value: string) {
    workInput = value;
    const n = parseInt(value, 10);
    if (n > 0 && n <= 180) void update({ workMinutes: n });
  }

  function onWorkMinutesBlur() {
    const n = parseInt(workInput, 10);
    if (Number.isNaN(n) || n <= 0) {
      workInput = String(savedConfig.workMinutes);
      return;
    }
    const clamped = Math.min(180, Math.max(1, n));
    workInput = String(clamped);
    if (clamped !== savedConfig.workMinutes) void update({ workMinutes: clamped });
  }

  function onBreakMinutesInput(value: string) {
    breakInput = value;
    const n = parseInt(value, 10);
    if (n > 0 && n <= 180) void update({ breakMinutes: n });
  }

  function onBreakMinutesBlur() {
    const n = parseInt(breakInput, 10);
    if (Number.isNaN(n) || n <= 0) {
      breakInput = String(savedConfig.breakMinutes);
      return;
    }
    const clamped = Math.min(180, Math.max(1, n));
    breakInput = String(clamped);
    if (clamped !== savedConfig.breakMinutes) void update({ breakMinutes: clamped });
  }
</script>

<div class="settings">
  <TitleBar title={t("settings")} showBack {onback} />

  <div class="content">
    <section>
      <h3>{t("workDuration")}</h3>
      <div class="duration-input">
        <input
          type="number"
          min="1"
          max="180"
          value={workInput}
          oninput={(e) => onWorkMinutesInput((e.currentTarget as HTMLInputElement).value)}
          onblur={onWorkMinutesBlur}
        />
        <span class="unit">{t("minutes")}</span>
      </div>
    </section>

    <section>
      <h3>{t("breakDuration")}</h3>
      <div class="duration-input">
        <input
          type="number"
          min="1"
          max="180"
          value={breakInput}
          oninput={(e) => onBreakMinutesInput((e.currentTarget as HTMLInputElement).value)}
          onblur={onBreakMinutesBlur}
        />
        <span class="unit">{t("minutes")}</span>
      </div>
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
      checked={savedConfig.customBreakBackground}
      onchange={(v) => update({ customBreakBackground: v })}
    >
      {#if savedConfig.customBreakBackground}
        <button type="button" class="pick-btn" onclick={pickBackground}>
          {savedConfig.breakBackgroundPath ? t("changeBg") : t("pickBg")}
        </button>
      {/if}
    </SettingCard>

    <SettingCard
      title={t("customTheme")}
      description={t("customThemeDesc")}
      checked={savedConfig.customThemeColor}
      onchange={(v) => update({ customThemeColor: v })}
    >
      {#if savedConfig.customThemeColor}
        <ColorPicker value={savedConfig.themeColor} onchange={(c) => update({ themeColor: c })} />
      {/if}
    </SettingCard>

    <SettingCard
      title={t("autostart")}
      description={t("autostartDesc")}
      checked={savedConfig.autostart}
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

  .duration-input {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .duration-input input {
    flex: 1;
    border: none;
    background: #eef2f4;
    border-radius: 10px;
    padding: 10px 12px;
    font-size: 14px;
    color: #333;
    outline: none;
  }

  .duration-input input:focus {
    box-shadow: 0 0 0 2px var(--primary-soft);
  }

  .duration-input .unit {
    font-size: 14px;
    color: #888;
    flex-shrink: 0;
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
