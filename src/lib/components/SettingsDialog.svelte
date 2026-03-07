<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { useTheme } from "$lib/hooks/theme.svelte";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { formatTimeHuman } from "$lib/utils/time";
  import { Plus, Trash2, Check, X } from "@lucide/svelte";

  let {
    open = $bindable(false),
  }: {
    open?: boolean;
  } = $props();

  const theme = useTheme();
  const favouritesContext = useFavourites();

  let state = $state({
    showAddFavourite: false,
    newFavourite: {
      name: "",
      minutes: null as number | null,
    },
  });

  // Reset state when dialog closes
  $effect(() => {
    if (!open) {
      cancelAddFavourite();
    }
  });

  function cancelAddFavourite() {
    state.showAddFavourite = false;
    state.newFavourite.name = "";
    state.newFavourite.minutes = null;
  }

  async function handleCreateFavourite(e: Event) {
    e.preventDefault();
    if (
      !state.newFavourite.name.trim() ||
      state.newFavourite.minutes === null ||
      state.newFavourite.minutes <= 0
    )
      return;

    await favouritesContext.add(state.newFavourite.name.trim(), state.newFavourite.minutes * 60);
    cancelAddFavourite();
  }

  async function handleDeleteFavourite(id: number) {
    await favouritesContext.remove(id);
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-xl bg-surface border-surface-hover">
    <Dialog.Header>
      <Dialog.Title>Settings</Dialog.Title>
      <Dialog.Description>Customize your time tracker preferences.</Dialog.Description>
    </Dialog.Header>

    <div class="py-4">
      <!-- Theme Setting -->
      <div class="space-y-3">
        <h3 class="text-lg font-medium">Appearance</h3>
        <div class="rounded-lg divide-y divide-surface-hover">
          <div class="flex items-center justify-between">
            <div>
              <div class="font-medium text-on-surface-muted">Theme</div>
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

      <!-- Favourites Section -->
      <div class="space-y-3 mt-6">
        <h3 class="text-lg font-medium border-b border-muted pb-2">Favourites</h3>
        {#if favouritesContext.favourites.length === 0}
          <div class="text-sm text-on-surface-muted">
            No favourites yet. Favourites will appear in the task input dropdown for quick access.
          </div>
        {:else}
          <div class="space-y-2">
            {#each favouritesContext.favourites as favourite, index (favourite.id)}
              <div class="flex gap-2 items-stretch">
                <div
                  class="flex-1 flex items-center justify-between py-2 pl-4 pr-3 rounded-lg bg-surface-raised transition-colors"
                >
                  <div>
                    <span class="font-medium">{favourite.name}</span>
                    <span class="ml-2 text-sm text-on-surface-muted"
                      >{formatTimeHuman(favourite.duration_seconds)}</span
                    >
                  </div>
                  <Button
                    onclick={() => handleDeleteFavourite(favourite.id)}
                    variant="ghost"
                    size="icon"
                    class="text-on-surface-muted hover:text-red-500"
                    aria-label="Delete favourite"
                  >
                    <Trash2 class="w-4 h-4" />
                  </Button>
                </div>
                {#if index === favouritesContext.favourites.length - 1 && !state.showAddFavourite}
                  <div>
                    <Button
                      onclick={() => (state.showAddFavourite = true)}
                      class="h-full rounded-lg bg-surface-raised hover:bg-surface-hover transition-colors text-on-surface-muted hover:text-on-surface"
                      aria-label="Add favourite"
                      variant="ghost"
                    >
                      <Plus class="w-4 h-4" />
                    </Button>
                  </div>
                {/if}
              </div>
            {/each}

            {#if state.showAddFavourite}
              <div class="p-4 rounded-lg bg-surface-raised">
                <form onsubmit={handleCreateFavourite} class="flex gap-2">
                  <Input
                    type="text"
                    placeholder="Name"
                    bind:value={state.newFavourite.name}
                    class="flex-1 bg-surface-raised border-surface-hover h-9"
                  />
                  <Input
                    type="number"
                    min="1"
                    placeholder="Min"
                    bind:value={state.newFavourite.minutes}
                    class="w-20 bg-surface-raised border-surface-hover h-9"
                  />
                  <button
                    type="submit"
                    class="p-2 h-9 rounded-md hover:bg-surface-hover text-accent hover:text-accent transition-colors"
                    aria-label="Add favourite"
                  >
                    <Check class="w-4 h-4" />
                  </button>
                  <button
                    type="button"
                    onclick={cancelAddFavourite}
                    class="p-2 rounded-md hover:bg-surface-hover text-on-surface-muted hover:text-red-500 transition-colors"
                    aria-label="Cancel"
                  >
                    <X class="w-4 h-4" />
                  </button>
                </form>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <Dialog.Footer>
      <Dialog.Close>
        <Button variant="outline">Close</Button>
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
