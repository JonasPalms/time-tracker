<script lang="ts">
  import { useTheme } from "$lib/theme.svelte"
  import { useTimeAdjust } from "$lib/timeAdjust.svelte"
  import Icon from "$lib/components/Icon.svelte"

  const theme = useTheme()
  const timeAdjust = useTimeAdjust()

  let inputValue = $state(timeAdjust.intervalMinutes)

  // Sync input when context changes
  $effect(() => {
    inputValue = timeAdjust.intervalMinutes
  })

  function handleChange(e: Event) {
    const target = e.target as HTMLInputElement
    const value = parseInt(target.value, 10)
    if (!isNaN(value) && value > 0 && value <= 60) {
      inputValue = value
      timeAdjust.setIntervalMinutes(value)
    }
  }
</script>

<div class="px-6">
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

  <!-- Divider -->
  <hr class="border-on-surface/10" />

  <!-- Time Adjustment Interval -->
  <div>
    <div class="mb-3 flex items-center gap-2 justify-between">
      <div>
        <p class="font-medium">Time Adjustment Interval</p>
        <p class="text-sm text-on-surface-muted">
          How many minutes to add/subtract per click
        </p>
      </div>
      <div class="flex items-center gap-2">
        <input
          type="number"
          min="1"
          max="60"
          bind:value={inputValue}
          onchange={handleChange}
          class="w-24 px-3 py-2 bg-surface rounded-xl border-none text-on-surface font-medium"
        />
      </div>
    </div>
  </div>
</div>
</div>
