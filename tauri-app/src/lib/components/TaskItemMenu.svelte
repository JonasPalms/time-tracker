<script lang="ts">
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";

  let {
    onDelete,
  }: {
    onDelete: () => void;
  } = $props();

  let dropdownOpen = $state(false);

  function handleDelete(e?: MouseEvent | Event) {
    if (e) {
      e.stopPropagation();
    }
    if (onDelete) {
      onDelete();
    }
    dropdownOpen = false;
  }
</script>

<DropdownMenu.Root bind:open={dropdownOpen}>
  <DropdownMenu.Trigger>
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Open menu"
      class="opacity-0 group-hover:opacity-100 group-focus:opacity-100 focus-within:opacity-100 {dropdownOpen
        ? 'opacity-100'
        : ''} transition-opacity"
    >
      <EllipsisIcon class="w-4 h-4" />
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content class="w-[180px]" align="end">
    <DropdownMenu.Item onclick={handleDelete} variant="destructive">
      <TrashIcon class="me-2 size-4" />
      Delete
    </DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
