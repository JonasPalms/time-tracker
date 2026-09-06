const STORAGE_KEY_HIDDEN = "sidebar-hidden";
const DEFAULT_WIDTH = 200;

let hidden = $state(false);

function init() {
  if (typeof localStorage !== "undefined") {
    hidden = localStorage.getItem(STORAGE_KEY_HIDDEN) === "true";
  }
}

function toggle() {
  hidden = !hidden;

  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY_HIDDEN, String(hidden));
  }
}

export function useSidebar() {
  return {
    get width() {
      return DEFAULT_WIDTH;
    },
    get hidden() {
      return hidden;
    },
    init,
    toggle,
  };
}

init();
