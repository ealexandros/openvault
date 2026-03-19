"use client";

import { Kbd } from "@/components/ui/shadcn/kbd";
import { Separator } from "@/components/ui/shadcn/separator";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { PanelLeft } from "lucide-react";

type DashboardHeaderProps = {
  title?: string;
  onToggleSidebar: () => void;
};

export const DashboardHeader = ({ title, onToggleSidebar }: DashboardHeaderProps) => (
  <header className="flex h-20 items-center gap-4 border-b border-border/40 bg-background/50 px-6">
    <Tooltip>
      <TooltipTrigger asChild>
        <button className="cursor-pointer" onClick={onToggleSidebar}>
          <PanelLeft className="size-5 text-muted-foreground/60 transition-colors hover:text-foreground" />
        </button>
      </TooltipTrigger>
      <TooltipContent side="right">
        <Kbd>m</Kbd>
        Toggle Sidebar
      </TooltipContent>
    </Tooltip>

    <div className="flex items-center gap-4">
      <Separator orientation="vertical" className="h-6" />
      {title != null && (
        <span className="text-base font-medium tracking-tight text-foreground/80">
          {title}
        </span>
      )}
    </div>
  </header>
);
