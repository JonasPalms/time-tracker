import { getContext, setContext } from "svelte"
import { getTheme, setTheme, type Theme } from "$lib/settings"

const THEME_KEY = Symbol("theme")

export type ThemeContext = {
  get theme(): Theme
  get isDark(): boolean
  toggleTheme: () => Promise<void>
}

/**
 * Create and provide the theme context
 */
export function createThemeContext() {
  let theme = $state<Theme>("dark")
  let isDark = $derived(theme === "dark")

  // Apply theme class to <html>
  $effect(() => {
    if (typeof document !== "undefined") {
      document.documentElement.classList.toggle("dark", isDark)
    }
  })

  async function init() {
    theme = await getTheme()
  }

  async function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark"
    await setTheme(theme)
  }

  const ctx: ThemeContext = {
    get theme() {
      return theme
    },
    get isDark() {
      return isDark
    },
    toggleTheme,
  }

  setContext(THEME_KEY, ctx)

  return { init }
}

export function useTheme(): ThemeContext {
  return getContext<ThemeContext>(THEME_KEY)
}
