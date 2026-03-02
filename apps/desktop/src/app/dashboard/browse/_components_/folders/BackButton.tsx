"use client";

import { ArrowLeftIcon } from "lucide-react";

type BackButtonProps = {
  handleBreadcrumbClick: (index: number) => void;
  currentPath: string[];
};

export const BackButton = ({ handleBreadcrumbClick, currentPath }: BackButtonProps) => (
  <button
    type="button"
    onClick={() => handleBreadcrumbClick(currentPath.length - 2)}
    className="group relative flex cursor-pointer items-center gap-4 overflow-hidden rounded-2xl border border-dashed border-primary/40 bg-primary/5 p-3.5 text-left transition-all duration-300 hover:bg-primary/10">
    <div className="flex size-11 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
      <ArrowLeftIcon className="size-5" strokeWidth={2.25} />
    </div>

    <div className="min-w-0 flex-1">
      <p className="truncate text-sm font-semibold tracking-tight text-foreground/90 transition-colors group-hover:text-foreground">
        Back folder
      </p>
      <div className="flex items-center gap-1.5 opacity-70">
        <span className="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
          Go to parent
        </span>
      </div>
    </div>
  </button>
);
