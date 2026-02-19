<script lang="ts">
  let {
    name,
    onUpdateName,
  }: {
    name: string;
    onUpdateName?: (newName: string) => void;
  } = $props();

  let isEditing = $state(false);
  let editedName = $state("");

  function handleInputClick(e: MouseEvent) {
    e.stopPropagation();
  }

  function handleInputFocus(e: FocusEvent) {
    e.stopPropagation();
    isEditing = true;
    editedName = name;
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
    isEditing = false;
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
    value={isEditing ? editedName : name}
    oninput={(e) => (editedName = e.currentTarget.value)}
    class="font-medium w-full truncate px-1 -mx-1 rounded selection:bg-accent selection:text-on-accent"
    onclick={handleInputClick}
    onfocus={handleInputFocus}
    onkeydown={handleInputKeydown}
    onblur={handleInputBlur}
  />
</div>
