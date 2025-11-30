<script lang="ts">
  import Icon from "./Icon.svelte"
  import { getCurrentDate } from "$lib/timeUtils"
</script>


<div class="flex items-center justify-between mb-6">
  <div>
    <h1 class="text-2xl font-black text-accent">{getCurrentDate()}</h1>
  </div>
  <div class="flex items-center gap-2">
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
</div>

