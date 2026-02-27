"use client";

import { Skeleton } from "@/components/ui/shadcn/skeleton";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { cn } from "@/utils/cn";
import { CheckIcon, ChevronRightIcon, FileIcon, XIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { RecentVault } from "../hooks/useVaultAccess";

type RecentVaultItemProps = {
  vault: RecentVault;
  onConnect: (path: string) => void;
  onRemove: (id: string) => void;
};

export const RecentVaultItem = ({ vault, onConnect, onRemove }: RecentVaultItemProps) => {
  const [isConfirming, setIsConfirming] = useState(false);
  const nameWithoutExtension = vault.name.replace(/\.[^/.]+$/, "");

  const truncateMiddle = (str: string, maxLen: number = 40) => {
    if (str.length <= maxLen) return str;
    const start = str.slice(0, Math.floor(maxLen / 2) - 1);
    const end = str.slice(-Math.floor(maxLen / 2) + 1);
    return `${start}...${end}`;
  };

  useEffect(() => {
    if (isConfirming) {
      const timer = setTimeout(() => setIsConfirming(false), 3000);
      return () => clearTimeout(timer);
    }
  }, [isConfirming]);

  return (
    <div
      onClick={() => (isConfirming ? setIsConfirming(false) : onConnect(vault.path))}
      className="group flex cursor-pointer items-center justify-between rounded-xl border border-slate-200/60 bg-slate-50/50 p-3.5 transition-all duration-300 hover:border-primary/20 hover:bg-white hover:shadow-[0_6px_8px_rgb(0,0,0,0.03)]">
      <div className="flex min-w-0 items-center gap-4">
        <div className="flex size-11 shrink-0 items-center justify-center rounded-lg border border-slate-200 bg-white transition-all duration-300 group-hover:border-primary/10 group-hover:bg-primary/5">
          <FileIcon className="size-5 text-slate-400 transition-colors duration-300 group-hover:text-primary" />
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
      <div className="flex items-center gap-2">
        <button
          onClick={e => {
            e.stopPropagation();
            if (isConfirming) {
              onRemove(vault.id);
            } else {
              setIsConfirming(true);
            }
          }}
          className={cn(
            "flex size-7 items-center justify-center rounded-lg transition-all duration-200",
            isConfirming
              ? "scale-110 bg-red-500 text-white opacity-100 shadow-lg shadow-red-200"
              : "text-slate-300 opacity-0 group-hover:opacity-100 hover:bg-red-50 hover:text-red-500",
          )}>
          {isConfirming ? <CheckIcon className="size-3.5" /> : <XIcon className="size-3.5" />}
        </button>
        <ChevronRightIcon className="size-4 text-slate-300 transition-all duration-300 group-hover:translate-x-0.5 group-hover:text-primary/50" />
      </div>
    </div>
  );
};

export const RecentVaultSkeleton = () => (
  <div className="flex items-center justify-between rounded-xl border border-slate-200/60 bg-slate-50/50 p-3.5">
    <div className="flex min-w-0 items-center gap-4">
      <Skeleton className="size-11 shrink-0 rounded-lg" />
      <div className="space-y-2">
        <Skeleton className="h-4 w-24" />
        <Skeleton className="h-3 w-32" />
      </div>
    </div>
    <div className="flex items-center gap-4">
      <Skeleton className="size-4 rounded-full" />
    </div>
  </div>
);
