"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { useVault } from "@/context/VaultContext";
import { LockIcon, MoreVerticalIcon, SearchIcon, ShieldAlertIcon } from "lucide-react";
import { usePathname } from "next/navigation";
import { useEffect, useRef } from "react";

export const DashboardHeader = () => {
  const { lockVault } = useVault();
  const searchInputRef = useRef<HTMLInputElement>(null);
  const pathname = usePathname();

  const activeTitle = pathname.split("/").pop()?.replace("-", " ") || "Dashboard";

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "/" && document.activeElement?.tagName !== "INPUT") {
        e.preventDefault();
        searchInputRef.current?.focus();
      }

      if (e.shiftKey && e.key === "Escape") {
        e.preventDefault();
        lockVault();
      }

      if (e.shiftKey && e.key === "D") {
        e.preventDefault();
        alert("Switching to decoy...");
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [lockVault]);

  return (
    <header className="flex h-16 items-center justify-between border-b border-border/50 bg-card/10 px-8 backdrop-blur-md">
      <h2 className="text-sm font-semibold tracking-widest text-foreground uppercase">
        {activeTitle}
      </h2>
      <div className="flex items-center gap-4">
        <div className="mr-4 flex items-center gap-2 border-r border-border/50 pr-4">
          <Button
            variant="destructive"
            size="sm"
            onClick={lockVault}
            className="group/panic relative h-8 rounded-lg px-3 text-[10px] font-bold tracking-tighter uppercase">
            <LockIcon className="mr-1.5 size-3" />
            Panic Lock
            <span className="ml-2 hidden rounded border border-white/20 bg-white/10 px-1 font-mono text-[9px] italic opacity-60 group-hover/panic:opacity-100 lg:inline-block">
              ⇧ ESC
            </span>
          </Button>
          <Button
            variant="secondary"
            size="sm"
            className="group/decoy h-8 rounded-lg border border-orange-500/20 bg-orange-500/10 px-3 text-[10px] font-bold tracking-tighter text-orange-500 uppercase hover:bg-orange-500/20">
            <ShieldAlertIcon className="mr-1.5 size-3" />
            Decoy
            <span className="ml-2 hidden rounded border border-orange-500/20 bg-orange-500/5 px-1 font-mono text-[9px] italic opacity-60 group-hover/decoy:opacity-100 lg:inline-block">
              ⇧ D
            </span>
          </Button>
        </div>

        <div className="group relative">
          <SearchIcon className="absolute top-1/2 left-3 size-3.5 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary" />
          <Input
            ref={searchInputRef}
            placeholder="Search files..."
            className="h-8 w-64 rounded-lg border-border/50 bg-muted/30 pr-8 pl-9 text-xs focus:ring-primary/20"
          />
          <div className="pointer-events-none absolute top-1/2 right-2 flex h-4 -translate-y-1/2 items-center justify-center rounded border border-border/50 bg-muted/50 px-1.5 font-mono text-[10px] text-muted-foreground">
            /
          </div>
        </div>
        <Button
          variant="ghost"
          size="icon"
          className="size-8 text-muted-foreground hover:text-foreground">
          <MoreVerticalIcon className="size-4" />
        </Button>
      </div>
    </header>
  );
};
