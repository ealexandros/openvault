"use client";

import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { ChevronRightIcon, FileIcon } from "lucide-react";
import { RecentVault } from "../useVaultSelection";

type RecentVaultItemProps = {
  vault: RecentVault;
  onConnect: (path: string) => void;
};

export const RecentVaultItem = ({ vault, onConnect }: RecentVaultItemProps) => {
  const formatTime = (time: string) => {
    if (time === "Just now") return time;
    try {
      const date = new Date(time);
      return new Intl.DateTimeFormat("en-GB", {
        hour: "2-digit",
        minute: "2-digit",
      }).format(date);
    } catch {
      return time;
    }
  };

  const nameWithoutExtension = vault.name.replace(/\.[^/.]+$/, "");

  const truncateMiddle = (str: string, maxLen: number = 40) => {
    if (str.length <= maxLen) return str;
    const start = str.slice(0, Math.floor(maxLen / 2) - 1);
    const end = str.slice(-Math.floor(maxLen / 2) + 1);
    return `${start}...${end}`;
  };

  return (
    <div
      onClick={() => onConnect(vault.path)}
      className="group flex cursor-pointer items-center justify-between rounded-xl border border-slate-200/60 bg-slate-50/50 p-3.5 transition-all duration-300 hover:border-primary/20 hover:bg-white hover:shadow-[0_6px_8px_rgb(0,0,0,0.03)]">
      <div className="flex min-w-0 items-center gap-4">
        <div className="flex size-11 shrink-0 items-center justify-center rounded-lg border border-slate-200 bg-white transition-all duration-300 group-hover:border-primary/10 group-hover:bg-primary/5">
          <FileIcon className="h-5 w-5 text-slate-400 transition-colors duration-300 group-hover:text-primary" />
        </div>
        <div className="min-w-0">
          <h4 className="truncate text-sm font-semibold tracking-tight text-slate-900">
            {nameWithoutExtension}
          </h4>
          <Tooltip>
            <TooltipTrigger asChild>
              <p className="mt-0.5 truncate text-[11px] font-semibold text-slate-400 group-hover:text-slate-500">
                {truncateMiddle(vault.path)}
              </p>
            </TooltipTrigger>
            <TooltipContent side="bottom" className="max-w-[400px] break-all">
              {vault.path}
            </TooltipContent>
          </Tooltip>
        </div>
      </div>
      <div className="flex shrink-0 items-center gap-4">
        <span className="text-[10px] font-bold tracking-widest text-slate-300 uppercase transition-colors group-hover:text-slate-400">
          {formatTime(vault.lastAccessed)}
        </span>
        <ChevronRightIcon className="size-4 text-slate-300 transition-all duration-300 group-hover:translate-x-0.5 group-hover:text-primary/50" />
      </div>
    </div>
  );
};
