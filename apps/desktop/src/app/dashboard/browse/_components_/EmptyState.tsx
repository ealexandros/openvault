"use client";

import { FolderIcon } from "lucide-react";

export const EmptyState = () => (
  <div className="col-span-full flex animate-in flex-col items-center justify-center space-y-4 py-20 text-center duration-300 fade-in zoom-in">
    <div className="flex h-20 w-20 items-center justify-center rounded-full border border-muted/30 bg-muted/20">
      <FolderIcon className="size-10 text-muted-foreground/30" />
    </div>
    <div className="space-y-1">
      <p className="text-sm font-medium text-foreground">No files found</p>
      <p className="mx-auto max-w-xs text-xs text-muted-foreground">
        This folder is empty. Upload a file to get started.
      </p>
    </div>
  </div>
);
