export type Locale = "zh" | "en";

export const localeStore = $state({ current: "zh" as Locale });

const messages = {
  zh: {
    appTitle: "Reminder",
    settings: "设置",
    back: "返回",
    menu: "菜单",
    minimizeToTray: "最小化到托盘",
    preference: "偏好",
    workDuration: "设置工作时长",
    breakDuration: "设置休息时长",
    customBreakBg: "自定义休息背景",
    customBreakBgDesc: "开启后在休息模式展示图片",
    pickBg: "选择背景图",
    changeBg: "更换背景图",
    customTheme: "自定义主题色",
    customThemeDesc: "开启后可以选择主题色",
    autostart: "开机自启",
    autostartDesc: "开启后在电脑开机时自动启动",
    language: "界面语言",
    languageDesc: "切换应用显示语言",
    langZh: "中文",
    langEn: "English",
    minutes: "分钟",
    custom: "自定义",
    phaseIdle: "准备开始",
    phaseWorking: "专注中",
    phaseBreak: "休息中",
    start: "开始",
    stop: "停止",
    breakTitle: "休息时间",
    breakHint: "请离开座位，伸个懒腰，放松一下眼睛...",
    closeBreak: "关闭",
    trayShow: "显示主窗口",
    trayQuit: "退出",
  },
  en: {
    appTitle: "Reminder",
    settings: "Settings",
    back: "Back",
    menu: "Menu",
    minimizeToTray: "Minimize to tray",
    preference: "Preferences",
    workDuration: "Work duration",
    breakDuration: "Break duration",
    customBreakBg: "Custom break background",
    customBreakBgDesc: "Show an image during break",
    pickBg: "Choose image",
    changeBg: "Change image",
    customTheme: "Custom theme color",
    customThemeDesc: "Pick your accent color",
    autostart: "Launch at startup",
    autostartDesc: "Start Reminder when Windows starts",
    language: "Language",
    languageDesc: "Switch display language",
    langZh: "中文",
    langEn: "English",
    minutes: "min",
    custom: "Custom",
    phaseIdle: "Ready",
    phaseWorking: "Focusing",
    phaseBreak: "On break",
    start: "Start",
    stop: "Stop",
    breakTitle: "Break time",
    breakHint: "Step away, stretch, and rest your eyes...",
    closeBreak: "Close",
    trayShow: "Show main window",
    trayQuit: "Quit",
  },
} as const;

export type MessageKey = keyof typeof messages.zh;

export function t(key: MessageKey, params?: Record<string, string | number>): string {
  let msg: string = messages[localeStore.current][key];
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      msg = msg.replaceAll(`{${k}}`, String(v));
    }
  }
  return msg;
}

export function setLocale(next: Locale) {
  localeStore.current = next;
}

export function phaseLabel(phase: string): string {
  switch (phase) {
    case "working":
      return t("phaseWorking");
    case "break":
      return t("phaseBreak");
    default:
      return t("phaseIdle");
  }
}
