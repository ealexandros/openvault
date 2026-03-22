import { SidebarMenuButton, SidebarMenuItem } from "@/components/ui/shadcn/sidebar";

export const UserListItemSkeleton = () => (
  <SidebarMenuItem>
    <SidebarMenuButton size="lg" className="pointer-events-none">
      <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-muted animate-pulse" />
      <div className="flex flex-col gap-1 w-full mr-2">
        <div className="h-4 w-[60%] animate-pulse rounded bg-muted" />
        <div className="h-3 w-[40%] animate-pulse rounded bg-muted" />
      </div>
    </SidebarMenuButton>
  </SidebarMenuItem>
);
