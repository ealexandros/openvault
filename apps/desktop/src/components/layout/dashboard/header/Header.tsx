"use client";

import { Input } from "@/components/ui/shadcn/input";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { KeyCode } from "@/config/keycodes";
import { useKeyListeners } from "@/hooks/useKeyListeners";
import { cn } from "@/utils/cn";
import { PanelLeft, SearchIcon } from "lucide-react";
import { useRef } from "react";

export const DashboardHeader = () => {
  const searchInputRef = useRef<HTMLInputElement>(null);

  useKeyListeners({
    [KeyCode.Slash]: event => {
      event.preventDefault();
      searchInputRef.current?.focus();
    },
  });

  return (
    <header className="flex h-20 items-center justify-between border-b border-border/40 bg-background/50 px-6">
      <Tooltip>
        <TooltipTrigger asChild>
          <button className="cursor-pointer">
            <PanelLeft className="size-5" />
          </button>
        </TooltipTrigger>
        <TooltipContent side="right">Open menu</TooltipContent>
      </Tooltip>

      <div className="flex items-center gap-6">
        <div className="group relative">
          <SearchIcon className="absolute top-1/2 left-3.5 size-4 -translate-y-1/2 text-muted-foreground/40 transition-colors group-focus-within:text-primary" />
          <Input
            ref={searchInputRef}
            placeholder="Search vault..."
            className={cn(
              "h-10 w-72 rounded-lg border-border/40 bg-muted/20 pr-10 pl-10 text-sm transition-all duration-300",
              "focus:bg-muted/40 focus:ring-2 focus:ring-primary/10 focus:ring-offset-0",
            )}
          />
          <div className="pointer-events-none absolute top-1/2 right-3 flex h-5 w-5 -translate-y-1/2 items-center justify-center rounded-md border border-border/40 bg-muted/60 font-mono text-[10px] font-bold text-muted-foreground/60">
            /
          </div>
        </div>
      </div>
    </header>
  );
};
