<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { useTheme } from "$lib/hooks/theme.svelte";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { formatTimeHuman } from "$lib/utils/time";
  import Icon from "./Icon.svelte";

  let {
    open = $bindable(false),
  }: {
    open?: boolean;
  } = $props();

  const theme = useTheme();
  const favouritesContext = useFavourites();

  let newFavouriteName = $state("");
  let newFavouriteMinutes = $state<number | null>(null);

  async function handleCreateFavourite(e: Event) {
    e.preventDefault();
    if (!newFavouriteName.trim() || newFavouriteMinutes === null || newFavouriteMinutes <= 0)
      return;

    await favouritesContext.add(newFavouriteName.trim(), newFavouriteMinutes * 60);
    newFavouriteName = "";
    newFavouriteMinutes = null;
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

    <div class="space-y-6 py-4">
      <!-- Theme Setting -->
      <div class="space-y-2">
        <h3 class="text-sm font-medium">Appearance</h3>
        <div class="flex items-center justify-between rounded-lg border border-surface-hover p-4">
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

      <!-- Favourites Section -->
      <div class="space-y-2">
        <h3 class="text-sm font-medium">Favourites</h3>
        <div class="rounded-lg border border-surface-hover p-4 space-y-4">
          <!-- Add new favourite form -->
          <form onsubmit={handleCreateFavourite} class="flex gap-2">
            <Input
              type="text"
              placeholder="Name"
              bind:value={newFavouriteName}
              class="flex-1 bg-surface-hover border-surface-hover"
            />
            <Input
              type="number"
              min="1"
              placeholder="Minutes"
              bind:value={newFavouriteMinutes}
              class="w-24 bg-surface-hover border-surface-hover"
            />
            <Button type="submit" variant="outline" class="shrink-0">Add</Button>
          </form>

          {#if favouritesContext.favourites.length === 0}
            <p class="text-sm text-on-surface-muted">
              No favourites yet. Favourites will appear in the task input dropdown for quick access.
            </p>
          {:else}
            <ul class="space-y-2">
              {#each favouritesContext.favourites as favourite (favourite.id)}
                <li
                  class="flex items-center justify-between py-2 px-3 rounded-md bg-surface-hover/50"
                >
                  <div>
                    <span class="font-medium">{favourite.name}</span>
                    <span class="ml-2 text-sm text-on-surface-muted"
                      >{formatTimeHuman(favourite.duration_seconds)}</span
                    >
                  </div>
                  <button
                    onclick={() => handleDeleteFavourite(favourite.id)}
                    class="p-1.5 rounded-md hover:bg-surface-hover text-on-surface-muted hover:text-red-500 transition-colors"
                    aria-label="Delete favourite"
                  >
                    <Icon name="trash" class="w-4 h-4" />
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      </div>
    </div>

    <Dialog.Footer>
      <Dialog.Close>
        <Button variant="outline">Close</Button>
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
