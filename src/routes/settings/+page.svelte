<script lang="ts">
  import { useTheme } from "$lib/theme.svelte"
  import Icon from "$lib/components/Icon.svelte"

  const theme = useTheme()
</script>

<main class="container mx-auto p-6 max-w-2xl">
  <!-- Header -->
  <div class="flex items-center gap-4 mb-6">
    <a
      href="/"
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      aria-label="Back to tracker"
    >
      <Icon name="chevron-left" class="w-6 h-6" />
    </a>
    <h1 class="text-2xl font-black text-accent">Settings</h1>
  </div>

  <!-- Content -->
  <div class="bg-surface-raised rounded-xl p-6 space-y-6">
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
</main>

