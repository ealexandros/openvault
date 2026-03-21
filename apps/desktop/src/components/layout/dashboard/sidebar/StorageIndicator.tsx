"use client";

import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/shadcn/sidebar";
import { lodash } from "@/libraries/lodash";
import { HardDrive } from "lucide-react";

type StorageIndicatorProps = {
  sizeInBytes?: number;
};

const BYTES_PER_MB = 1024 * 1024;
const VAULT_CAPACITY_MB = 2000;
const MIN_BAR_PERCENTAGE = 2;

export const StorageIndicator = ({ sizeInBytes = 0 }: StorageIndicatorProps) => {
  const usedStorageMb = sizeInBytes / BYTES_PER_MB;
  const displayedUsedMb = lodash.round(usedStorageMb, 2);
  const usagePercentage = lodash.percent(usedStorageMb, VAULT_CAPACITY_MB);

  return (
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          <SidebarMenuItem className="px-1 py-2 text-sm">
            <div className="mb-2 flex items-center justify-between text-muted-foreground">
              <span className="flex items-center gap-1.5 font-medium">
                <HardDrive className="size-4" />
                Storage
              </span>
              <span className="text-xs">
                {displayedUsedMb}MB / {VAULT_CAPACITY_MB / 1000}GB
              </span>
            </div>
            <div className="h-1.5 w-full overflow-hidden rounded-full bg-secondary">
              <div
                className="h-full bg-primary transition-all delay-150 duration-500 ease-out"
                style={{ width: `${Math.max(usagePercentage, MIN_BAR_PERCENTAGE)}%` }}
              />
            </div>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
};
