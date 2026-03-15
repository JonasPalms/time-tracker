import { load } from "@tauri-apps/plugin-store";

let store: Awaited<ReturnType<typeof load>> | null = null;

/**
 * Get or create the settings store.
 * The store persists to: ~/.config/time-tracker/settings.json
 */
async function getStore() {
  if (!store) {
    store = await load("settings.json", { autoSave: true, defaults: {} });
  }
  return store;
}

// ============================================
// Theme Settings
// ============================================

export type Theme = "dark" | "light";

const THEME_KEY = "theme";
const ACCENT_COLOR_KEY = "accentColor";

/**
 * Get the saved theme, or return 'dark' as default
 */
export async function getTheme(): Promise<Theme> {
  const s = await getStore();
  const theme = await s.get<Theme>(THEME_KEY);
  return theme ?? "dark";
}

/**
 * Save the theme preference
 */
export async function setTheme(theme: Theme): Promise<void> {
  const s = await getStore();
  await s.set(THEME_KEY, theme);
}

/**
 * Get the saved accent color, or null when using the default theme accent.
 */
export async function getAccentColor(): Promise<string | null> {
  const s = await getStore();
  const accentColor = await s.get<string>(ACCENT_COLOR_KEY);
  return typeof accentColor === "string" ? accentColor : null;
}

/**
 * Save the accent color preference.
 */
export async function setAccentColor(accentColor: string): Promise<void> {
  const s = await getStore();
  await s.set(ACCENT_COLOR_KEY, accentColor);
}

/**
 * Clear the accent color preference and fall back to the theme default.
 */
export async function clearAccentColor(): Promise<void> {
  const s = await getStore();
  await s.delete(ACCENT_COLOR_KEY);
}
