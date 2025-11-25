<script lang="ts">
  import { useTheme } from "$lib/theme.svelte"

  let { open = $bindable(false) } = $props<{ open: boolean }>()

  const theme = useTheme()

  let dialogEl: HTMLDialogElement

  // Sync the `open` prop with the dialog's native state
  $effect(() => {
    if (!dialogEl) return

    if (open && !dialogEl.open) {
      dialogEl.showModal()
    } else if (!open && dialogEl.open) {
      dialogEl.close()
    }
  })

  function handleClose() {
    open = false
  }

  function handleBackdropClick(e: MouseEvent) {
    // If click target is the dialog itself (not a child), it's the backdrop
    if (e.target === dialogEl) {
      handleClose()
    }
  }
</script>

<dialog
  bind:this={dialogEl}
  onclose={handleClose}
  onclick={handleBackdropClick}
  class="m-auto bg-surface-raised text-on-surface rounded-xl shadow-2xl w-full max-w-md overflow-hidden backdrop:bg-black/50 backdrop:backdrop-blur-sm focus:outline-none focus-visible:outline-none"
>
  <!-- Header -->
  <div class="flex items-center justify-between px-6 py-4 border-b border-on-surface/10">
    <h2 class="text-xl font-bold">Settings</h2>
    <button
      class="p-2 rounded-lg hover:bg-surface transition-colors text-on-surface-muted hover:text-on-surface"
      onclick={handleClose}
      aria-label="Close settings"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  </div>

  <!-- Content -->
  <div class="p-6 space-y-6">
    <!-- Theme Setting -->
    <div class="flex items-center justify-between">
      <div>
        <div class="font-medium">Theme</div>
        <div class="text-sm text-on-surface-muted">Choose light or dark mode</div>
      </div>
      <button
        class="relative w-16 h-9 rounded-full transition-colors {theme.isDark ? 'bg-accent' : 'bg-on-surface/20'}"
        onclick={theme.toggleTheme}
        aria-label="Toggle theme"
      >
        <span
          class="absolute top-1 left-1 w-7 h-7 rounded-full bg-surface-raised shadow-md transition-transform flex items-center justify-center {theme.isDark ? 'translate-x-7' : 'translate-x-0'}"
        >
          {theme.isDark ? "🌙" : "☀️"}
        </span>
      </button>
    </div>

    <!-- Divider for future settings -->
    <hr class="border-on-surface/10" />

    <!-- Placeholder for more settings -->
    <div class="text-sm text-on-surface-muted text-center py-4">
      More settings coming soon...
    </div>
  </div>
</dialog>
