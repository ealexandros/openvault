"use client";

import {
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/shadcn/sidebar";
import { cn } from "@/utils/cn";
import { PinIcon } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { usePinNavigation } from "../../../hooks/usePinNavigation";
import { NavigationItem } from "../Sidebar";

type NavigationMenuItemProps = {
  item: NavigationItem;
  pinnable?: boolean;
};

export const NavigationMenuItem = ({ item, pinnable = true }: NavigationMenuItemProps) => {
  const pathname = usePathname();
  const { pinnedRouteUrls, togglePin } = usePinNavigation();

  const isPinned = pinnedRouteUrls.includes(item.url);
  const isActive = pathname === item.url;

  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        tooltip={item.title}
        asChild
        className="px-3 py-5 text-base"
        isActive={isActive}>
        {item.blank === true ? (
          <a href={item.url} target="_blank" rel="noopener noreferrer">
            {item.icon}
            <span>{item.title}</span>
          </a>
        ) : (
          <Link href={item.url}>
            {item.icon}
            <span>{item.title}</span>
          </Link>
        )}
      </SidebarMenuButton>

      {pinnable && (
        <SidebarMenuAction
          showOnHover={!isPinned}
          onClick={e => {
            e.preventDefault();
            e.stopPropagation();
            togglePin(item.url);
          }}
          className="cursor-pointer px-3 py-1.5 hover:text-foreground">
          <PinIcon
            className={cn(
              "size-3 text-neutral-400 transition-all",
              isPinned && "fill-current text-neutral-600",
            )}
          />
        </SidebarMenuAction>
      )}
    </SidebarMenuItem>
  );
};
