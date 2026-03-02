"use client";

import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { PanelLeft } from "lucide-react";

export const DashboardHeader = () => (
  <header className="flex h-20 items-center justify-between border-b border-border/40 bg-background/50 px-6">
    <Tooltip>
      <TooltipTrigger asChild>
        <button className="cursor-pointer">
          <PanelLeft className="size-5" />
        </button>
      </TooltipTrigger>
      <TooltipContent side="right">Open menu</TooltipContent>
    </Tooltip>
  </header>
);
