export type TimerPhase = "idle" | "working" | "short_break" | "long_break";
export type AppLocale = "zh" | "en";

export interface AppConfig {
  workMinutes: number;
  shortBreakMinutes: number;
  longBreakMinutes: number;
  longBreakInterval: number;
  longBreakMode: boolean;
  strictBreakMode: boolean;
  autoBreakMode: boolean;
  dailyFocusGoal: boolean;
  dailyGoalCount: number;
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
  sessionsCompletedToday: number;
  sessionsInCycle: number;
  dailyGoalCount: number;
  dailyFocusGoal: boolean;
  strictBreakMode: boolean;
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

export const WORK_PRESETS = [10, 20, 25, 30, 45, 60];
export const BREAK_PRESETS = [1, 2, 5, 10, 15, 20];

export const DEFAULT_CONFIG: AppConfig = {
  workMinutes: 25,
  shortBreakMinutes: 5,
  longBreakMinutes: 15,
  longBreakInterval: 4,
  longBreakMode: false,
  strictBreakMode: false,
  autoBreakMode: false,
  dailyFocusGoal: false,
  dailyGoalCount: 8,
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
  sessionsCompletedToday: 0,
  sessionsInCycle: 0,
  dailyGoalCount: 8,
  dailyFocusGoal: false,
  strictBreakMode: false,
};
