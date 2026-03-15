import type { Theme } from "$lib/services/settings";

const DEFAULT_LIGHT_ACCENT_COLOR = "#d97706";
const DEFAULT_DARK_ACCENT_COLOR = "#f59e0b";
const LIGHT_FOREGROUND = "#1c1917";
const DARK_FOREGROUND = "#ffffff";

type Rgb = {
  r: number;
  g: number;
  b: number;
};

export function getDefaultAccentColor(theme: Theme): string {
  return theme === "dark" ? DEFAULT_DARK_ACCENT_COLOR : DEFAULT_LIGHT_ACCENT_COLOR;
}

export function normalizeHexColor(value: string | null | undefined): string | null {
  if (!value) return null;

  const normalized = value.trim().toLowerCase();
  const shortHexMatch = normalized.match(/^#([0-9a-f]{3})$/i);

  if (shortHexMatch) {
    const [, shortHex] = shortHexMatch;
    return `#${shortHex
      .split("")
      .map((char) => `${char}${char}`)
      .join("")}`;
  }

  if (/^#[0-9a-f]{6}$/i.test(normalized)) {
    return normalized;
  }

  return null;
}

export function deriveAccentPalette(color: string, theme: Theme) {
  const normalized = normalizeHexColor(color) ?? getDefaultAccentColor(theme);
  const foreground = getAccessibleForeground(normalized);

  return {
    accent: normalized,
    accentForeground: foreground,
    ring: mixColors(normalized, theme === "dark" ? "#ffffff" : "#000000", 0.12),
    accentHover: mixColors(normalized, theme === "dark" ? "#ffffff" : "#000000", 0.14),
  };
}

function getAccessibleForeground(color: string): string {
  const whiteContrast = getContrastRatio(color, DARK_FOREGROUND);
  const darkContrast = getContrastRatio(color, LIGHT_FOREGROUND);

  return whiteContrast >= darkContrast ? DARK_FOREGROUND : LIGHT_FOREGROUND;
}

function getContrastRatio(background: string, foreground: string): number {
  const backgroundLuminance = getRelativeLuminance(hexToRgb(background));
  const foregroundLuminance = getRelativeLuminance(hexToRgb(foreground));
  const lighter = Math.max(backgroundLuminance, foregroundLuminance);
  const darker = Math.min(backgroundLuminance, foregroundLuminance);

  return (lighter + 0.05) / (darker + 0.05);
}

function getRelativeLuminance({ r, g, b }: Rgb): number {
  const [red, green, blue] = [r, g, b].map((channel) => {
    const normalized = channel / 255;
    return normalized <= 0.03928 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
  });

  return red * 0.2126 + green * 0.7152 + blue * 0.0722;
}

function mixColors(colorA: string, colorB: string, weight: number): string {
  const a = hexToRgb(colorA);
  const b = hexToRgb(colorB);
  const clampedWeight = clamp(weight, 0, 1);

  return rgbToHex({
    r: Math.round(a.r * (1 - clampedWeight) + b.r * clampedWeight),
    g: Math.round(a.g * (1 - clampedWeight) + b.g * clampedWeight),
    b: Math.round(a.b * (1 - clampedWeight) + b.b * clampedWeight),
  });
}

function hexToRgb(color: string): Rgb {
  const normalized = normalizeHexColor(color);

  if (!normalized) {
    throw new Error(`Invalid hex color: ${color}`);
  }

  return {
    r: Number.parseInt(normalized.slice(1, 3), 16),
    g: Number.parseInt(normalized.slice(3, 5), 16),
    b: Number.parseInt(normalized.slice(5, 7), 16),
  };
}

function rgbToHex({ r, g, b }: Rgb): string {
  return `#${[r, g, b]
    .map((channel) => clamp(channel, 0, 255).toString(16).padStart(2, "0"))
    .join("")}`;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}
