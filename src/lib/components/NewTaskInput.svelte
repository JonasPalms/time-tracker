<script lang="ts">
  let {
    onAddTask,
  }: {
    onAddTask: (taskName: string) => void;
  } = $props();

  let newTaskName = $state("");

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!newTaskName.trim()) return;

    onAddTask(newTaskName.trim());
    newTaskName = "";
  }
</script>

<form onsubmit={handleSubmit} class="mb-4">
  <input
    type="text"
    bind:value={newTaskName}
    placeholder="What are you working on?"
    class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted"
  />
</form>
