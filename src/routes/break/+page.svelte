<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { DEFAULT_CONFIG, DEFAULT_TIMER, type AppConfig, type TimerSnapshot } from "$lib/types";
  import { applyThemeColor, formatTime } from "$lib/utils";
  import { setLocale, t } from "$lib/i18n.svelte";

  let config = $state<AppConfig>({ ...DEFAULT_CONFIG });
  let timer = $state<TimerSnapshot>({ ...DEFAULT_TIMER });

  const bgStyle = $derived.by(() => {
    if (config.customBreakBackground && config.breakBackgroundPath) {
      const src = convertFileSrc(config.breakBackgroundPath);
      return `background-image:url('${src}')`;
    }
    return "background:linear-gradient(160deg,#f6d365 0%,#fda085 40%,#6a85b6 100%)";
  });

  onMount(() => {
    let unlistenTick: (() => void) | undefined;
    let unlistenComplete: (() => void) | undefined;

    void (async () => {
      config = await invoke<AppConfig>("get_config");
      timer = await invoke<TimerSnapshot>("get_timer_state");
      setLocale(config.locale);
      applyThemeColor(config.customThemeColor ? config.themeColor : "#2aab9b");

      unlistenTick = await listen<TimerSnapshot>("timer-tick", (event) => {
        timer = event.payload;
      });

      unlistenComplete = await listen("timer-break-complete", async () => {
        await getCurrentWebviewWindow().close();
      });
    })();

    return () => {
      unlistenTick?.();
      unlistenComplete?.();
    };
  });

  async function endBreak() {
    await invoke("skip_break_cmd");
    await getCurrentWebviewWindow().close();
  }
</script>

<div class="break-screen">
  <button type="button" class="close-btn" aria-label={t("closeBreak")} onclick={endBreak}>
    ×
  </button>

  <div class="bg" style={bgStyle}></div>
  <div class="overlay"></div>

  <div class="content">
    <h1>{t("breakTitle")}</h1>
    <div class="timer">{formatTime(timer.remainingSecs)}</div>
    <p class="hint">{t("breakHint")}</p>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    overflow: hidden;
  }

  .break-screen {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  .close-btn {
    position: absolute;
    top: 20px;
    right: 20px;
    z-index: 2;
    width: 40px;
    height: 40px;
    border: 1px solid rgba(255, 255, 255, 0.4);
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.25);
    backdrop-filter: blur(8px);
    color: white;
    font-size: 28px;
    font-weight: 300;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 0 2px;
  }

  .close-btn:hover {
    background: rgba(0, 0, 0, 0.4);
  }

  .bg {
    position: absolute;
    inset: -20px;
    background-size: cover;
    background-position: center;
    filter: blur(18px);
    transform: scale(1.05);
  }

  .overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.25);
  }

  .content {
    position: relative;
    z-index: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: white;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.35);
    padding: 24px;
  }

  h1 {
    font-size: 28px;
    font-weight: 700;
    margin: 0 0 16px;
  }

  .timer {
    font-size: 96px;
    font-weight: 800;
    letter-spacing: 2px;
    margin-bottom: 20px;
  }

  .hint {
    font-size: 16px;
    opacity: 0.92;
    margin: 0;
    text-align: center;
    max-width: 480px;
    line-height: 1.6;
  }
</style>
