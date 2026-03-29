import {
  clearAccentColor as clearSavedAccentColor,
  getAccentColor as getSavedAccentColor,
  getTheme,
  setAccentColor as saveAccentColor,
  setTheme,
  type Theme,
} from "$lib/services/settings";
import { deriveAccentPalette, getDefaultAccentColor, normalizeHexColor } from "$lib/utils";

// Module-level reactive state - shared globally
let theme = $state<Theme>("dark");
let accentColor = $state(getDefaultAccentColor("dark"));
let hasCustomAccent = $state(false);

function applyTheme() {
  if (typeof document !== "undefined") {
    document.documentElement.classList.toggle("dark", theme === "dark");
  }
}

function applyAccent() {
  if (typeof document === "undefined") return;

  const root = document.documentElement;

  if (!hasCustomAccent) {
    root.style.removeProperty("--accent");
    root.style.removeProperty("--accent-foreground");
    root.style.removeProperty("--ring");
    root.style.removeProperty("--accent-hover");
    accentColor = getDefaultAccentColor(theme);
    return;
  }

  const palette = deriveAccentPalette(accentColor, theme);

  root.style.setProperty("--accent", palette.accent);
  root.style.setProperty("--accent-foreground", palette.accentForeground);
  root.style.setProperty("--ring", palette.ring);
  root.style.setProperty("--accent-hover", palette.accentHover);
}

async function init() {
  theme = await getTheme();
  applyTheme();

  const savedAccentColor = normalizeHexColor(await getSavedAccentColor());
  hasCustomAccent = savedAccentColor !== null;
  accentColor = savedAccentColor ?? getDefaultAccentColor(theme);
  applyAccent();
}

async function toggleTheme() {
  theme = theme === "dark" ? "light" : "dark";
  await setTheme(theme);
  applyTheme();
  if (!hasCustomAccent) {
    accentColor = getDefaultAccentColor(theme);
  }
  applyAccent();
}

async function setAccentColor(color: string) {
  const normalized = normalizeHexColor(color);

  if (!normalized) return;

  accentColor = normalized;
  hasCustomAccent = true;
  applyAccent();
  await saveAccentColor(normalized);
}

async function resetAccentColor() {
  hasCustomAccent = false;
  accentColor = getDefaultAccentColor(theme);
  applyAccent();
  await clearSavedAccentColor();
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
    get accentColor() {
      return accentColor;
    },
    get hasCustomAccent() {
      return hasCustomAccent;
    },
    init,
    toggleTheme,
    setAccentColor,
    resetAccentColor,
  };
}
