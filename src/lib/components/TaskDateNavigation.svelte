<script lang="ts">
  import Icon from "$lib/components/Icon.svelte";

  let {
    displayDate,
    isToday,
    onPreviousDay,
    onNextDay,
    onGoToToday,
  }: {
    displayDate: string;
    isToday: boolean;
    onPreviousDay: () => void;
    onNextDay: () => void;
    onGoToToday: () => void;
  } = $props();
</script>

<div class="flex items-center gap-3 mb-4">
  <button
    class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
    onclick={onPreviousDay}
    aria-label="Previous day"
  >
    <Icon name="chevron-left" class="w-6 h-6" />
  </button>
  <button
    class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
    onclick={onNextDay}
    aria-label="Next day"
  >
    <Icon name="chevron-right" class="w-6 h-6" />
  </button>
  <h1 class="text-2xl ml-2 font-black text-accent">
    {displayDate}
  </h1>
  {#if !isToday}
    <button
      class="ml-auto px-3 py-1 text-sm rounded-md border border-accent text-accent hover:bg-accent/10 transition-colors"
      onclick={onGoToToday}
    >
      Go to today
    </button>
  {/if}
</div>
