// Re-export utils for backward compatibility with shadcn-svelte UI components
// which import from "$lib/utils.js"
export {
  cn,
  type WithoutChild,
  type WithoutChildren,
  type WithoutChildrenOrChild,
  type WithElementRef,
} from "./utils/cn";
