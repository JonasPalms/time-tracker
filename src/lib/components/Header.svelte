<script lang="ts">
  import Icon from "./Icon.svelte"
  import { formatDateForDisplay, getDateString, addDays } from "$lib/timeUtils"
  import { getTodayDate } from "$lib/tasks"

  let {
    selectedDate,
    onDateChange,
  }: {
    selectedDate: string
    onDateChange: (newDate: string) => void
  } = $props()

  const isToday = $derived(selectedDate === getTodayDate())
  const displayDate = $derived(formatDateForDisplay(selectedDate))

  function handlePreviousDay() {
    onDateChange(addDays(selectedDate, -1))
  }

  function handleNextDay() {
    onDateChange(addDays(selectedDate, 1))
  }
</script>

<div class="mb-6">
  <!-- Action buttons in top right -->
  <div class="flex items-center justify-end gap-2 mb-3">
    <a
      href="/history"
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      aria-label="View history"
    >
      <Icon name="clock" class="w-6 h-6" />
    </a>
    <a
      href="/settings"
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      aria-label="Open settings"
    >
      <Icon name="settings" class="w-6 h-6" />
    </a>
  </div>
  
  <!-- Date navigation in its own section -->
  <div class="flex items-center gap-3">
    <button
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      onclick={handlePreviousDay}
      aria-label="Previous day"
    >
      <Icon name="chevron-left" class="w-6 h-6" />
    </button>
    <div class="flex-1 min-w-0">
      <h1 class="text-2xl font-black text-accent">
        {displayDate}
        {#if isToday}
          <span class="text-sm ml-2 text-accent">(Today)</span>
        {/if}
      </h1>
    </div>
    <button
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      onclick={handleNextDay}
      aria-label="Next day"
    >
      <Icon name="chevron-right" class="w-6 h-6" />
    </button>
  </div>
</div>

