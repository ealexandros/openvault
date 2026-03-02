"use client";

import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { cn } from "@/utils/cn";
import { truncateLeft } from "@/utils/format";
import { FileIcon, XIcon } from "lucide-react";
import { RecentVault } from "../hooks/useVaultAccess";

type RecentVaultItemProps = {
  vault: RecentVault;
  onConnect: (path: string) => void;
  onRemove: (id: string) => void;
};

export const RecentVaultItem = ({ vault, onConnect, onRemove }: RecentVaultItemProps) => {
  const nameWithoutExtension = vault.name.replace(/\.[^/.]+$/, "");

  return (
    <div
      onClick={() => onConnect(vault.path)}
      className="group flex cursor-pointer items-center justify-between rounded-xl border border-slate-200/60 bg-slate-50/50 p-3.5 transition-all duration-300 hover:border-primary/20 hover:bg-white hover:shadow-[0_6px_8px_rgb(0,0,0,0.02)]">
      <div className="flex min-w-0 items-center gap-4">
        <div className="flex size-12 shrink-0 items-center justify-center rounded-lg border border-slate-200 bg-white transition-all duration-300 group-hover:border-primary/10 group-hover:bg-primary/5">
          <FileIcon className="size-5 text-slate-400 transition-colors duration-300 group-hover:text-primary" />
        </div>
        <div className="min-w-0">
          <h4 className="truncate text-base font-semibold tracking-tight text-slate-900">
            {nameWithoutExtension}
          </h4>
          <Tooltip>
            <TooltipTrigger asChild>
              <p className="mt-0.5 max-w-72 text-sm font-semibold whitespace-nowrap text-slate-400 group-hover:text-slate-500">
                {truncateLeft(vault.path, 35)}
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
            onRemove(vault.id);
          }}
          className={cn(
            "flex size-7 cursor-pointer items-center justify-center rounded-lg transition-all duration-200",
            "text-slate-300 opacity-0 group-hover:opacity-100 hover:bg-red-50 hover:text-red-500",
          )}>
          <XIcon className="size-4" />
        </button>
      </div>
    </div>
  );
};
