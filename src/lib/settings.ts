import { load } from "@tauri-apps/plugin-store"

let store: Awaited<ReturnType<typeof load>> | null = null

/**
 * Get or create the settings store.
 * The store persists to: ~/.config/time-tracker/settings.json
 */
async function getStore() {
  if (!store) {
    store = await load("settings.json", { autoSave: true, defaults: {} })
  }
  return store
}

// ============================================
// Theme Settings
// ============================================

export type Theme = "dark" | "light"

/**
 * Get the saved theme, or return 'dark' as default
 */
export async function getTheme(): Promise<Theme> {
  const s = await getStore()
  const theme = await s.get<Theme>("theme")
  return theme ?? "dark"
}

/**
 * Save the theme preference
 */
export async function setTheme(theme: Theme): Promise<void> {
  const s = await getStore()
  await s.set("theme", theme)
}

// ============================================
// Time Adjustment Settings
// ============================================

/**
 * Get the time adjustment interval in minutes, or return 5 as default
 */
export async function getTimeAdjustInterval(): Promise<number> {
  const s = await getStore()
  const interval = await s.get<number>("timeAdjustInterval")
  return interval ?? 5 // Default to 5 minutes
}

/**
 * Save the time adjustment interval (in minutes)
 */
export async function setTimeAdjustInterval(minutes: number): Promise<void> {
  const s = await getStore()
  await s.set("timeAdjustInterval", minutes)
}
