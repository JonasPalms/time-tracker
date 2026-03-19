// Module-level reactive state - shared globally
let openModalIds = $state<string[]>([]);

function setOpen(id: string, isOpen: boolean) {
  const hasId = openModalIds.includes(id);

  if (isOpen && !hasId) {
    openModalIds = [...openModalIds, id];
    return;
  }

  if (!isOpen && hasId) {
    openModalIds = openModalIds.filter((modalId) => modalId !== id);
  }
}

function hasOtherOpen(id: string) {
  return openModalIds.some((modalId) => modalId !== id);
}

export function useModalState() {
  return {
    get openIds() {
      return openModalIds;
    },
    get hasOpen() {
      return openModalIds.length > 0;
    },
    setOpen,
    hasOtherOpen,
  };
}
