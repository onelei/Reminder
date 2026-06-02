<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SettingCard from "$lib/components/SettingCard.svelte";
  import ColorPicker from "$lib/components/ColorPicker.svelte";
  import DurationInput from "$lib/components/DurationInput.svelte";
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

</script>

<div class="settings">
  <TitleBar title={t("settings")} showBack {onback} />

  <div class="content">
    <section>
      <h3>{t("workDuration")}</h3>
      <div class="duration-input">
        <DurationInput
          value={savedConfig.workMinutes}
          onchange={(n) => update({ workMinutes: n })}
        />
        <span class="unit">{t("minutes")}</span>
      </div>
    </section>

    <section>
      <h3>{t("breakDuration")}</h3>
      <div class="duration-input">
        <DurationInput
          value={savedConfig.breakMinutes}
          onchange={(n) => update({ breakMinutes: n })}
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
