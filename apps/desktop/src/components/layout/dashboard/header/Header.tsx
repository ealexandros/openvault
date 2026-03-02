"use client";

import { Separator } from "@/components/ui/shadcn/separator";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { PanelLeft } from "lucide-react";

type DashboardHeaderProps = {
  onToggleSidebar: () => void;
  title?: string;
};

export const DashboardHeader = ({ onToggleSidebar, title }: DashboardHeaderProps) => (
  <header className="flex h-20 items-center gap-4 border-b border-border/40 bg-background/50 px-6">
    <Tooltip>
      <TooltipTrigger asChild>
        <button className="cursor-pointer" onClick={onToggleSidebar}>
          <PanelLeft className="size-5 text-muted-foreground/60 transition-colors hover:text-foreground" />
        </button>
      </TooltipTrigger>
      <TooltipContent side="right">Toggle Sidebar</TooltipContent>
    </Tooltip>

    <div className="flex items-center gap-4">
      <Separator orientation="vertical" className="h-6" />
      {typeof title === "string" && title.length > 0 && (
        <span className="text-sm font-medium tracking-tight text-foreground/80">{title}</span>
      )}
    </div>
  </header>
);
