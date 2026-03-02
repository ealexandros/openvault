"use client";

import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { PanelLeft } from "lucide-react";

type DashboardHeaderProps = {
  onToggleSidebar: () => void;
};

export const DashboardHeader = ({ onToggleSidebar }: DashboardHeaderProps) => (
  <header className="flex h-20 items-center justify-between border-b border-border/40 bg-background/50 px-6">
    <Tooltip>
      <TooltipTrigger asChild>
        <button className="cursor-pointer" onClick={onToggleSidebar}>
          <PanelLeft className="size-5 text-muted-foreground/60 transition-colors hover:text-foreground" />
        </button>
      </TooltipTrigger>
      <TooltipContent side="right">Toggle Sidebar</TooltipContent>
    </Tooltip>
  </header>
);
