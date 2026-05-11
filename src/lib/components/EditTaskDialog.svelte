<script lang="ts">
  import { untrack } from "svelte";
  import * as Dialog from "$lib/components/ui/dialog";
  import EditTaskForm from "$lib/components/EditTaskForm.svelte";
  import { useModalState } from "$lib/hooks/modal-state.svelte";
  import { getTaskById, type Task } from "$lib/services/tasks";
  import XIcon from "@lucide/svelte/icons/x";

  const MODAL_ID = "edit-task-dialog";

  let {
    open = $bindable(false),
    taskId = null,
    onTaskChange,
  }: {
    open?: boolean;
    taskId?: number | null;
    onTaskChange?: () => void | Promise<void>;
  } = $props();

  const modalState = useModalState();

  let task = $state<Task | null>(null);

  // Only re-fetch when `open` or `taskId` actually change. We intentionally
  // read `task` via untrack so that `loadTask` setting `task = {...}` does
  // not cause this effect to re-run and re-fetch in a loop.
  $effect(() => {
    if (!open || taskId === null) return;

    untrack(() => {
      if (task?.id !== taskId) {
        task = null;
      }
    });

    void loadTask(taskId);
  });

  $effect(() => {
    modalState.setOpen(MODAL_ID, open);
  });

  $effect(() => {
    return () => {
      modalState.setOpen(MODAL_ID, false);
    };
  });

  async function loadTask(id: number) {
    const loadedTask = await getTaskById(id);
    // Bail if the user has since switched task or closed the dialog,
    // so we don't poke state while bits-ui is animating the close.
    if (taskId !== id || !open) return;
    task = loadedTask ? { ...loadedTask } : null;
  }

  function attemptClose() {
    open = false;
  }

  function handleSaveSuccess() {
    open = false;
  }
</script>

<!--
  IMPORTANT: keep <Dialog.Content> mounted unconditionally and only
  conditionally render its inner contents. bits-ui's Dialog has its own
  PresenceManager + body scroll-lock cleanup that runs after the close
  animation. Wrapping <Dialog.Content> in an outer {#if} can yank it out
  of the DOM mid-cleanup and leave `pointer-events: none` stuck on <body>,
  making the whole UI un-clickable after the modal closes.
-->
<Dialog.Root bind:open>
  <Dialog.Content
    class="sm:max-w-xl bg-surface border-surface-hover"
    showCloseButton={false}
    onInteractOutside={(e: PointerEvent) => {
      e.preventDefault();
      attemptClose();
    }}
    onEscapeKeydown={(e: KeyboardEvent) => {
      e.preventDefault();
      attemptClose();
    }}
  >
    <Dialog.Header>
      <div class="flex items-start justify-between gap-4">
        <div class="space-y-1">
          <Dialog.Title>Edit Task</Dialog.Title>
          <Dialog.Description
            >Update the task details without leaving this view.</Dialog.Description
          >
        </div>
        <button
          class="rounded-xs p-1 opacity-70 transition-opacity hover:opacity-100"
          onclick={attemptClose}
          aria-label="Close"
        >
          <XIcon class="size-4" />
        </button>
      </div>
    </Dialog.Header>

    <div class="py-4">
      {#if taskId !== null && task}
        <EditTaskForm
          {taskId}
          initialTask={task}
          onCancel={attemptClose}
          {onTaskChange}
          onSaveSuccess={handleSaveSuccess}
        />
      {:else}
        <div class="text-center py-8 text-on-surface-muted">Loading...</div>
      {/if}
    </div>
  </Dialog.Content>
</Dialog.Root>
