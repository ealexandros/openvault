"use client";

import { Avatar, AvatarFallback } from "@/components/ui/shadcn/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/shadcn/sidebar";
import { env } from "@/config/env";
import { hrefs } from "@/config/hrefs";
import { BoxIcon, EllipsisVerticalIcon, LogOutIcon, Settings2Icon } from "lucide-react";
import Link from "next/link";

type DashboardFooterProps = {
  vaultName?: string;
  onLogout: () => void;
};

export function DashboardFooter({ vaultName, onLogout }: DashboardFooterProps) {
  const { isMobile } = useSidebar();

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton size="lg" className="cursor-pointer">
              <Avatar className="size-10 grayscale">
                <AvatarFallback>
                  <BoxIcon className="size-5!" />
                </AvatarFallback>
              </Avatar>
              <div className="grid flex-1 text-left leading-tight">
                <span className="truncate text-base font-medium">{vaultName}</span>
                <span className="truncate text-sm font-medium text-muted-foreground">
                  version {env.VERSION}
                </span>
              </div>
              <EllipsisVerticalIcon className="ml-auto size-4" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>

          <DropdownMenuContent
            className="min-w-56 rounded-lg shadow-none ring-gray-200/80"
            side={isMobile ? "bottom" : "right"}
            align="end"
            sideOffset={12}>
            <DropdownMenuGroup>
              <DropdownMenuItem className="text-sm" asChild>
                <Link href={hrefs.dashboard.settings.get()}>
                  <Settings2Icon className="size-4" />
                  Settings
                </Link>
              </DropdownMenuItem>
            </DropdownMenuGroup>

            <DropdownMenuSeparator />

            <DropdownMenuItem
              onClick={onLogout}
              className="group text-sm text-destructive hover:bg-destructive/5! hover:text-destructive!">
              <LogOutIcon className="size-4" color="var(--color-destructive)" />
              Lock Vault
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
