"use client";

import { SidebarGroup, SidebarGroupLabel, SidebarMenu } from "@/components/ui/shadcn/sidebar";
import { cn } from "@/utils/cn";
import { ChevronDownIcon } from "lucide-react";
import { useState } from "react";
import { NavigationItem } from "../Sidebar";
import { NavigationMenuItem } from "./NavigationMenuItem";

type NavigationGroupProps = React.ComponentPropsWithoutRef<typeof SidebarGroup> & {
  title?: string;
  items: NavigationItem[];
  pinnable?: boolean;
};

export const NavigationGroup = ({
  title,
  items,
  pinnable = false,
  ...props
}: NavigationGroupProps) => {
  const [isCollapsed, setIsCollapsed] = useState(false);

  return (
    <SidebarGroup {...props}>
      {title != null && (
        <SidebarGroupLabel
          asChild
          className="cursor-pointer justify-between text-sm transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground">
          <button onClick={() => setIsCollapsed(!isCollapsed)}>
            {title}
            <ChevronDownIcon
              className={cn(
                "size-4 transition-transform duration-200",
                isCollapsed && "-rotate-90",
              )}
            />
          </button>
        </SidebarGroupLabel>
      )}

      <div
        className={cn(
          "grid transition-all duration-200 ease-in-out",
          isCollapsed ? "grid-rows-[0fr] opacity-0" : "grid-rows-[1fr] opacity-100",
        )}>
        <div className="overflow-hidden">
          <SidebarMenu className="mt-1.5 ml-0 gap-1">
            {items.map(item => (
              <NavigationMenuItem key={item.title} item={item} pinnable={pinnable} />
            ))}
          </SidebarMenu>
        </div>
      </div>
    </SidebarGroup>
  );
};
