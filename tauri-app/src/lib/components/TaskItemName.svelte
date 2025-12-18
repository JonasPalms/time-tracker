<script lang="ts">
  let {
    name,
    onUpdateName,
  }: {
    name: string;
    onUpdateName?: (newName: string) => void;
  } = $props();

  let editedName = $state(name);

  $effect(() => {
    editedName = name;
  });

  function handleInputClick(e: MouseEvent) {
    e.stopPropagation();
  }

  function handleInputFocus(e: FocusEvent) {
    e.stopPropagation();
  }

  function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      saveName();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === "Escape") {
      e.preventDefault();
      editedName = name;
      (e.target as HTMLInputElement).blur();
    }
  }

  function handleInputBlur() {
    saveName();
  }

  function saveName() {
    if (editedName.trim() !== name.trim() && onUpdateName) {
      onUpdateName(editedName.trim());
    }
  }
</script>

<div class="flex-1">
  <input
    type="text"
    bind:value={editedName}
    class="font-medium truncate px-1 -mx-1 rounded selection:bg-accent selection:text-on-accent"
    onclick={handleInputClick}
    onfocus={handleInputFocus}
    onkeydown={handleInputKeydown}
    onblur={handleInputBlur}
  />
</div>
