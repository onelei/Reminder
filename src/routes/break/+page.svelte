<script lang="ts">

  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api/core";

  import { listen } from "@tauri-apps/api/event";

  import { getCurrentWindow } from "@tauri-apps/api/window";

  import { convertFileSrc } from "@tauri-apps/api/core";

  import { DEFAULT_CONFIG, DEFAULT_TIMER, type AppConfig, type TimerSnapshot } from "$lib/types";

  import { applyThemeColor, formatTime } from "$lib/utils";

  import { setLocale, t } from "$lib/i18n.svelte";



  let config = $state<AppConfig>({ ...DEFAULT_CONFIG });

  let timer = $state<TimerSnapshot>({ ...DEFAULT_TIMER });

  let holdProgress = $state(0);

  let holding = $state(false);

  let holdTimer: ReturnType<typeof setInterval> | null = null;



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

        await getCurrentWindow().close();

      });

    })();



    return () => {

      unlistenTick?.();

      unlistenComplete?.();

    };

  });



  function startHold() {

    if (timer.strictBreakMode && timer.remainingSecs > 0) return;

    holding = true;

    holdProgress = 0;

    const start = Date.now();

    holdTimer = setInterval(async () => {

      const elapsed = Date.now() - start;

      holdProgress = Math.min(elapsed / 6000, 1);

      if (holdProgress >= 1) {

        stopHold();

        await invoke("skip_break_cmd");

        await getCurrentWindow().close();

      }

    }, 50);

  }



  function stopHold() {

    holding = false;

    holdProgress = 0;

    if (holdTimer) {

      clearInterval(holdTimer);

      holdTimer = null;

    }

  }

</script>



<div class="break-screen">

  <div class="bg" style={bgStyle}></div>

  <div class="overlay"></div>



  <div class="content">

    <h1>{t("breakTitle")}</h1>

    <div class="timer">{formatTime(timer.remainingSecs)}</div>

    <p class="hint">{t("breakHint")}</p>



    {#if !timer.strictBreakMode || timer.remainingSecs <= 0}

      <!-- svelte-ignore a11y_no_static_element_interactions -->

      <div

        class="hold-btn"

        class:holding

        onmousedown={startHold}

        onmouseup={stopHold}

        onmouseleave={stopHold}

        ontouchstart={startHold}

        ontouchend={stopHold}

      >

        <span class="clock">⏱</span>

        {t("holdToEndBreak")}

        {#if holding}

          <div class="hold-bar" style={`width:${holdProgress * 100}%`}></div>

        {/if}

      </div>

    {/if}

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

    margin: 0 0 48px;

    text-align: center;

    max-width: 480px;

    line-height: 1.6;

  }



  .hold-btn {

    position: relative;

    overflow: hidden;

    display: inline-flex;

    align-items: center;

    gap: 8px;

    padding: 12px 24px;

    border-radius: 24px;

    background: rgba(255, 255, 255, 0.22);

    backdrop-filter: blur(8px);

    border: 1px solid rgba(255, 255, 255, 0.35);

    font-size: 14px;

    cursor: pointer;

    user-select: none;

  }



  .hold-bar {

    position: absolute;

    left: 0;

    top: 0;

    bottom: 0;

    background: rgba(255, 255, 255, 0.25);

    pointer-events: none;

    transition: width 0.05s linear;

  }



  .clock {

    font-size: 14px;

  }

</style>

