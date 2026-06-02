<script lang="ts">

  import { onMount } from "svelte";

  import { page } from "$app/state";

  import { invoke } from "@tauri-apps/api/core";

  import { getCurrentWindow } from "@tauri-apps/api/window";

  import { DEFAULT_CONFIG, type AppConfig } from "$lib/types";

  import { applyThemeColor } from "$lib/utils";

  import { setLocale, t } from "$lib/i18n.svelte";



  let config = $state<AppConfig>({ ...DEFAULT_CONFIG });



  const useLongBreak = $derived(page.url.searchParams.get("long") === "true");

  const breakMinutes = $derived(

    useLongBreak ? config.longBreakMinutes : config.shortBreakMinutes,

  );



  onMount(async () => {

    config = await invoke<AppConfig>("get_config");

    setLocale(config.locale);

    applyThemeColor(config.customThemeColor ? config.themeColor : "#2aab9b");

  });



  async function skip() {

    await invoke("close_break_prompt_cmd");

    await getCurrentWindow().close();

  }



  async function rest() {

    await invoke("start_break_cmd", { long: useLongBreak });

    await getCurrentWindow().close();

  }

</script>



<div class="prompt">

  <div class="icon">☕</div>

  <h2>{t("breakPromptTitle")}</h2>

  <p>{t("breakPromptBody", { minutes: breakMinutes })}</p>

  <div class="actions">

    <button type="button" class="skip" onclick={skip}>{t("skip")}</button>

    <button type="button" class="rest" onclick={rest}>{t("rest")}</button>

  </div>

</div>



<style>

  :global(body) {

    margin: 0;

    background: transparent;

  }



  .prompt {

    width: 100vw;

    height: 100vh;

    box-sizing: border-box;

    background: white;

    border-radius: 16px;

    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);

    padding: 20px 24px;

    display: flex;

    flex-direction: column;

    align-items: center;

    text-align: center;

  }



  .icon {

    font-size: 20px;

    color: var(--primary);

    margin-bottom: 4px;

  }



  h2 {

    margin: 0 0 6px;

    font-size: 18px;

    color: var(--primary);

    font-weight: 700;

  }



  p {

    margin: 0 0 16px;

    font-size: 13px;

    color: #7a9490;

  }



  .actions {

    display: flex;

    gap: 12px;

    width: 100%;

  }



  .skip,

  .rest {

    flex: 1;

    padding: 10px;

    border-radius: 20px;

    font-size: 14px;

    font-weight: 600;

    cursor: pointer;

  }



  .skip {

    border: 1.5px solid var(--primary);

    background: white;

    color: var(--primary);

  }



  .rest {

    border: none;

    background: var(--primary);

    color: white;

  }

</style>

