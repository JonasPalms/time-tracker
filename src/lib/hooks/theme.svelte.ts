import { getTheme, setTheme, type Theme } from "$lib/services/settings";

// Module-level reactive state - shared globally
let theme = $state<Theme>("dark");

function applyTheme() {
  if (typeof document !== "undefined") {
    document.documentElement.classList.toggle("dark", theme === "dark");
  }
}

async function init() {
  theme = await getTheme();
  applyTheme();
}

async function toggleTheme() {
  theme = theme === "dark" ? "light" : "dark";
  await setTheme(theme);
  applyTheme();
}

// Export a function that returns getters to maintain reactivity
export function useTheme() {
  return {
    get theme() {
      return theme;
    },
    get isDark() {
      return theme === "dark";
    },
    init,
    toggleTheme,
  };
}
