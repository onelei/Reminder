export type TimerPhase = "idle" | "working" | "break";
export type AppLocale = "zh" | "en";

export interface AppConfig {
  workMinutes: number;
  breakMinutes: number;
  customBreakBackground: boolean;
  breakBackgroundPath: string | null;
  customThemeColor: boolean;
  themeColor: string;
  autostart: boolean;
  locale: AppLocale;
}

export interface TimerSnapshot {
  phase: TimerPhase;
  remainingSecs: number;
  totalSecs: number;
  isPaused: boolean;
}

export const THEME_COLORS = [
  { id: "purple", value: "#8b6fd4" },
  { id: "teal", value: "#2aab9b" },
  { id: "rose", value: "#d48b9b" },
  { id: "red", value: "#c96a6a" },
  { id: "peach", value: "#e8a87c" },
  { id: "olive", value: "#a8b86d" },
  { id: "sage", value: "#7fb69e" },
  { id: "steel", value: "#6a9ec9" },
  { id: "slate", value: "#7a8a9a" },
  { id: "charcoal", value: "#5a5a5a" },
];

export const DEFAULT_CONFIG: AppConfig = {
  workMinutes: 30,
  breakMinutes: 5,
  customBreakBackground: false,
  breakBackgroundPath: null,
  customThemeColor: true,
  themeColor: "#2aab9b",
  autostart: false,
  locale: "zh",
};

export const DEFAULT_TIMER: TimerSnapshot = {
  phase: "idle",
  remainingSecs: 0,
  totalSecs: 0,
  isPaused: false,
};
