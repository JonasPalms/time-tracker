<script lang="ts">
  import { type DateValue, getLocalTimeZone } from "@internationalized/date";
  import CalendarIcon from "@lucide/svelte/icons/calendar";
  import { cn } from "$lib/utils.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";

  interface Props {
    value?: DateValue;
    onValueChange?: (value: DateValue | undefined) => void;
    placeholder?: string;
    class?: string;
  }

  let {
    value = $bindable(),
    onValueChange,
    placeholder = "Pick a date",
    class: className,
  }: Props = $props();

  let open = $state(false);

  function formatDate(date: DateValue): string {
    const jsDate = date.toDate(getLocalTimeZone());
    const day = String(jsDate.getDate()).padStart(2, "0");
    const month = String(jsDate.getMonth() + 1).padStart(2, "0");
    const year = String(jsDate.getFullYear());
    return `${day}.${month}.${year}`;
  }

  function handleValueChange(newValue: DateValue | undefined) {
    value = newValue;
    onValueChange?.(newValue);
    if (newValue) {
      open = false;
    }
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger
    class={cn(
      "inline-flex cursor-pointer w-full items-center justify-start gap-4 rounded-md text-left outline-none transition-colors focus-visible:ring-2 focus-visible:ring-accent/50",
      !value && "text-muted-foreground",
      className
    )}
  >
    <CalendarIcon class="size-4 shrink-0" />
    <span class="truncate">{value ? formatDate(value) : placeholder}</span>
  </Popover.Trigger>
  <Popover.Content class="w-auto p-0 border-muted" align="start">
    <Calendar {value} onValueChange={handleValueChange} initialFocus />
  </Popover.Content>
</Popover.Root>
