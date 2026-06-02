export type Locale = "zh" | "en";

export const localeStore = $state({ current: "zh" as Locale });

const messages = {
  zh: {
    appTitle: "Reminder",
    settings: "设置",
    back: "返回",
    menu: "菜单",
    minimizeToTray: "最小化到托盘",
    duration: "时长",
    preference: "偏好",
    workDuration: "设置工作时长",
    shortBreakDuration: "设置短时休息",
    longBreakDuration: "设置长时休息",
    longBreakMode: "长时休息模式",
    longBreakModeDesc: "开启后每完成一定数量专注时段将触发长休",
    strictBreakMode: "严格休息模式",
    strictBreakModeDesc: "开启后休息期间不可跳过",
    autoBreakMode: "自动休息模式",
    autoBreakModeDesc: "开启后专注提醒结束自动进入休息",
    dailyFocusGoal: "每日专注目标",
    dailyFocusGoalDesc: "开启后在主界面显示今日完成进度",
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
    dailyProgress: "今日 {done} / {goal} 个专注时段",
    phaseIdle: "准备开始",
    phaseWorking: "专注中",
    phaseShortBreak: "短休中",
    phaseLongBreak: "长休中",
    startFocus: "开始专注",
    endWork: "结束工作",
    stop: "停止",
    breakTitle: "休息时间",
    breakHint: "请离开座位，伸个懒腰，放松一下眼睛...",
    holdToEndBreak: "长按 6s 结束休息",
    breakPromptTitle: "该休息啦~",
    breakPromptBody: "辛苦了！休息 {minutes} 分钟吧？",
    skip: "跳过",
    rest: "休息",
    trayShow: "显示主窗口",
    trayQuit: "退出",
  },
  en: {
    appTitle: "Reminder",
    settings: "Settings",
    back: "Back",
    menu: "Menu",
    minimizeToTray: "Minimize to tray",
    duration: "Duration",
    preference: "Preferences",
    workDuration: "Work duration",
    shortBreakDuration: "Short break",
    longBreakDuration: "Long break",
    longBreakMode: "Long break mode",
    longBreakModeDesc: "Trigger a long break after a set number of focus sessions",
    strictBreakMode: "Strict break mode",
    strictBreakModeDesc: "Break cannot be skipped while active",
    autoBreakMode: "Auto break",
    autoBreakModeDesc: "Start break automatically when a focus session ends",
    dailyFocusGoal: "Daily focus goal",
    dailyFocusGoalDesc: "Show today's progress on the main screen",
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
    dailyProgress: "Today {done} / {goal} focus sessions",
    phaseIdle: "Ready",
    phaseWorking: "Focusing",
    phaseShortBreak: "Short break",
    phaseLongBreak: "Long break",
    startFocus: "Start focus",
    endWork: "End session",
    stop: "Stop",
    breakTitle: "Break time",
    breakHint: "Step away, stretch, and rest your eyes...",
    holdToEndBreak: "Hold 6s to end break",
    breakPromptTitle: "Time for a break",
    breakPromptBody: "Nice work! Take a {minutes}-minute break?",
    skip: "Skip",
    rest: "Break",
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
    case "short_break":
      return t("phaseShortBreak");
    case "long_break":
      return t("phaseLongBreak");
    default:
      return t("phaseIdle");
  }
}
