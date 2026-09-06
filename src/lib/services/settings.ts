export type Theme = "dark" | "light";

const THEME_KEY = "theme";
const ACCENT_COLOR_KEY = "accentColor";

function canUseLocalStorage() {
  return typeof localStorage !== "undefined";
}

export function getTheme(): Theme {
  if (!canUseLocalStorage()) return "dark";
  const stored = localStorage.getItem(THEME_KEY);
  return stored === "light" || stored === "dark" ? stored : "dark";
}

export function setTheme(theme: Theme): void {
  if (!canUseLocalStorage()) return;
  localStorage.setItem(THEME_KEY, theme);
}

export function getAccentColor(): string | null {
  if (!canUseLocalStorage()) return null;
  return localStorage.getItem(ACCENT_COLOR_KEY);
}

export function setAccentColor(accentColor: string): void {
  if (!canUseLocalStorage()) return;
  localStorage.setItem(ACCENT_COLOR_KEY, accentColor);
}

export function clearAccentColor(): void {
  if (!canUseLocalStorage()) return;
  localStorage.removeItem(ACCENT_COLOR_KEY);
}
