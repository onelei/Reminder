export function formatTime(totalSecs: number): string {
  const mins = Math.floor(totalSecs / 60);
  const secs = totalSecs % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}

export function applyThemeColor(color: string) {
  document.documentElement.style.setProperty("--primary", color);
  document.documentElement.style.setProperty(
    "--primary-light",
    `${color}22`,
  );
  document.documentElement.style.setProperty(
    "--primary-soft",
    `${color}18`,
  );
}

export function isBreakPhase(phase: string): boolean {
  return phase === "short_break" || phase === "long_break";
}
