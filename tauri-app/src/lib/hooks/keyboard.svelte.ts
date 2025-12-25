import { getCurrentWindow } from "@tauri-apps/api/window";
import { useTracking } from "./tracking.svelte";

type KeyboardShortcutHandler = (e: KeyboardEvent) => boolean | void | Promise<boolean | void>;

interface ShortcutRegistration {
  id: string;
  handler: KeyboardShortcutHandler;
}

// Module-level state
let isInitialized = false;
const registeredShortcuts: ShortcutRegistration[] = [];

/**
 * Global keyboard event handler
 */
async function handleKeydown(e: KeyboardEvent) {
  const isMac = navigator.userAgent.toLowerCase().includes("mac");
  const modKey = isMac ? e.metaKey : e.ctrlKey;

  // Cmd/Ctrl+W or Cmd/Ctrl+Q to close window
  if (modKey && (e.key === "w" || e.key === "q")) {
    e.preventDefault();
    await closeWindow();
    return;
  }

  // Run any registered custom shortcuts
  for (const { handler } of registeredShortcuts) {
    const handled = await handler(e);
    if (handled) {
      e.preventDefault();
      return;
    }
  }
}

/**
 * Close the window, stopping any active tracking first
 */
async function closeWindow() {
  const tracking = useTracking();
  await tracking.stopTracking();
  const currentWindow = getCurrentWindow();
  await currentWindow.close();
}

/**
 * Initialize global keyboard shortcuts
 * Should be called once in the root layout
 */
function init() {
  if (isInitialized) return;
  window.addEventListener("keydown", handleKeydown);
  isInitialized = true;
}

/**
 * Cleanup global keyboard shortcuts
 * Should be called when the app is destroyed
 */
function cleanup() {
  if (!isInitialized) return;
  window.removeEventListener("keydown", handleKeydown);
  isInitialized = false;
}

/**
 * Register a custom keyboard shortcut handler
 * Handler should return true if it handled the event (prevents further processing)
 * Returns an unregister function
 */
function register(id: string, handler: KeyboardShortcutHandler): () => void {
  // Remove existing registration with same id
  const existingIndex = registeredShortcuts.findIndex((s) => s.id === id);
  if (existingIndex !== -1) {
    registeredShortcuts.splice(existingIndex, 1);
  }

  registeredShortcuts.push({ id, handler });

  return () => {
    const index = registeredShortcuts.findIndex((s) => s.id === id);
    if (index !== -1) {
      registeredShortcuts.splice(index, 1);
    }
  };
}

export function useKeyboard() {
  return {
    init,
    cleanup,
    register,
  };
}
