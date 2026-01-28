"use client";

import { ChevronRightIcon } from "lucide-react";
import React from "react";

type BreadcrumbsProps = {
  currentPath: string[];
  onReset: () => void;
  onClick: (index: number) => void;
};

export const Breadcrumbs = ({ currentPath, onReset, onClick }: BreadcrumbsProps) => (
  <div className="flex w-fit items-center gap-1 rounded-md bg-muted/30 px-2 py-1 text-xs font-medium text-muted-foreground">
    <button
      onClick={onReset}
      className={`rounded px-1 transition-colors hover:bg-background/50 hover:text-foreground ${currentPath.length === 0 ? "font-semibold text-foreground" : ""}`}>
      Home
    </button>
    {currentPath.map((segment, i) => (
      <React.Fragment key={i}>
        <ChevronRightIcon className="size-3 opacity-40" />
        <button
          onClick={() => onClick(i)}
          className={`rounded px-1 transition-colors hover:bg-background/50 hover:text-foreground ${i === currentPath.length - 1 ? "font-semibold text-foreground" : ""}`}>
          {segment}
        </button>
      </React.Fragment>
    ))}
  </div>
);
