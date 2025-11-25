<script lang="ts">
  import SettingsModal from "$lib/components/SettingsModal.svelte"

  let settingsOpen = $state(false)
</script>

<main class="container mx-auto p-8">
  <!-- Header with settings button -->
  <div class="flex items-center justify-between mb-8">
    <h1 class="text-3xl font-black text-accent">
      Time Tracker
    </h1>
    <button
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      onclick={() => (settingsOpen = true)}
      aria-label="Open settings"
    >
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
        />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      </svg>
    </button>
  </div>

  <!-- Main content area (timer will go here) -->
  <div class="bg-surface-raised rounded-2xl p-8 text-center">
    <p class="text-on-surface-muted">Timer coming soon...</p>
  </div>
</main>

<!-- Settings Modal -->
<SettingsModal bind:open={settingsOpen} />
