<script lang="ts">
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

  $effect(() => {
    if (!open || taskId === null) return;

    if (task?.id !== taskId) {
      task = null;
    }

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
    if (taskId !== id) return;
    task = loadedTask ? { ...loadedTask } : null;
  }

  function attemptClose() {
    open = false;
  }

  function handleSaveSuccess() {
    open = false;
  }
</script>

<Dialog.Root bind:open>
  {#if taskId !== null && task}
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
        <EditTaskForm
          {taskId}
          initialTask={task}
          onCancel={attemptClose}
          {onTaskChange}
          onSaveSuccess={handleSaveSuccess}
        />
      </div>
    </Dialog.Content>
  {/if}
</Dialog.Root>
