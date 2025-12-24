<script lang="ts">
  import { useTheme } from "$lib/theme.svelte";

  const theme = useTheme();
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <section class="shrink-0 px-app">
    <h1 class="text-2xl font-black text-accent mb-4">Settings</h1>
  </section>

  <!-- Content -->
  <section class="flex-1 overflow-y-auto py-4">
    <div class="px-app">
      <div class="bg-surface-raised rounded-xl p-6 space-y-6">
        <!-- Theme Setting -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium">Theme</div>
            <div class="text-sm text-on-surface-muted">Choose light or dark mode</div>
          </div>
          <button
            class="relative w-16 h-9 rounded-full transition-colors {theme.isDark
              ? 'bg-accent'
              : 'bg-on-surface/20'}"
            onclick={theme.toggleTheme}
            aria-label="Toggle theme"
          >
            <span
              class="absolute top-1 left-1 w-7 h-7 rounded-full bg-surface-raised shadow-md transition-transform flex items-center justify-center {theme.isDark
                ? 'translate-x-7'
                : 'translate-x-0'}"
            >
              {theme.isDark ? "🌙" : "☀️"}
            </span>
          </button>
        </div>
      </div>
    </div>
  </section>
</div>
