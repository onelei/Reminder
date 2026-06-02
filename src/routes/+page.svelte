<script lang="ts">

  import { onMount } from "svelte";

  import { listen } from "@tauri-apps/api/event";

  import { invoke } from "@tauri-apps/api/core";

  import ProgressRing from "$lib/components/ProgressRing.svelte";

  import TitleBar from "$lib/components/TitleBar.svelte";

  import SettingsView from "$lib/components/SettingsView.svelte";

  import {
    DEFAULT_CONFIG,
    DEFAULT_TIMER,
    WORK_MINUTES,
    type AppConfig,
    type TimerSnapshot,
  } from "$lib/types";

  import { applyThemeColor, formatTime } from "$lib/utils";

  import { phaseLabel, setLocale, t } from "$lib/i18n.svelte";



  let view = $state<"main" | "settings">("main");

  let config = $state<AppConfig>({ ...DEFAULT_CONFIG });

  let timer = $state<TimerSnapshot>({ ...DEFAULT_TIMER });



  const progress = $derived(

    timer.totalSecs > 0 ? (timer.totalSecs - timer.remainingSecs) / timer.totalSecs : 0,

  );



  const displayTime = $derived(

    timer.phase === "idle"

      ? formatTime(WORK_MINUTES * 60)

      : formatTime(timer.remainingSecs),

  );



  const isActive = $derived(timer.phase !== "idle");



  onMount(() => {

    let unlisten: (() => void) | undefined;



    void (async () => {

      config = await invoke<AppConfig>("get_config");

      timer = await invoke<TimerSnapshot>("get_timer_state");

      setLocale(config.locale);

      applyThemeColor(config.customThemeColor ? config.themeColor : "#2aab9b");



      unlisten = await listen<TimerSnapshot>("timer-tick", (event) => {

        timer = event.payload;

      });

    })();



    return () => {

      unlisten?.();

    };

  });



  async function startWork() {

    timer = await invoke<TimerSnapshot>("start_work_cmd");

  }



  async function stop() {

    timer = await invoke<TimerSnapshot>("end_work_cmd");

  }



  function openSettings() {

    view = "settings";

  }



  function closeSettings() {

    view = "main";

  }



  function onConfigChange(next: AppConfig) {

    config = next;

    setLocale(next.locale);

  }

</script>



<div class="app">

  {#if view === "settings"}

    <SettingsView {config} onback={closeSettings} onchange={onConfigChange} />

  {:else}

    <TitleBar showMenu showClose onmenu={openSettings} />



    <div class="main">

      <div class="phase">{phaseLabel(timer.phase)}</div>



      <ProgressRing progress={isActive ? progress : 0} timeText={displayTime} />



      <div class="actions">

        {#if isActive}

          <button type="button" class="action-btn outline" onclick={stop}>

            {t("stop")}

          </button>

        {:else}

          <button type="button" class="action-btn primary" onclick={startWork}>

            {t("start")}

          </button>

        {/if}

      </div>

    </div>

  {/if}

</div>



<style>

  .app {

    height: 100vh;

    background: white;

    overflow: hidden;

  }



  .main {

    display: flex;

    flex-direction: column;

    align-items: center;

    padding: 8px 24px 32px;

  }



  .phase {

    font-size: 14px;

    color: #8aa8a3;

    margin-bottom: 12px;

  }



  .actions {

    margin-top: 36px;

    width: 100%;

    display: flex;

    justify-content: center;

  }



  .action-btn {

    display: inline-flex;

    align-items: center;

    gap: 8px;

    padding: 12px 28px;

    border-radius: 24px;

    font-size: 15px;

    font-weight: 600;

    cursor: pointer;

    transition: all 0.15s;

  }



  .action-btn.outline {

    border: 1.5px solid var(--primary);

    background: white;

    color: var(--primary);

  }



  .action-btn.primary {

    border: none;

    background: var(--primary);

    color: white;

  }



</style>

