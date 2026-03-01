"use client";

import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";
import { FolderIcon } from "lucide-react";

type LocationSelectorProps = {
  path: string;
  error?: string;
  touched?: boolean;
  chooseFolder: () => void;
};

export const LocationSelector = ({
  path,
  error,
  touched,
  chooseFolder,
}: LocationSelectorProps) => (
  <div className="space-y-2">
    <Label className="ml-1 text-xs font-bold tracking-wider text-muted-foreground uppercase">
      Location
    </Label>
    <div
      onClick={chooseFolder}
      className={cn(
        "group flex cursor-pointer items-center gap-3 rounded-lg border bg-muted/30 p-4 transition-all hover:border-foreground/10",
        touched === true && error != null ? "border-destructive" : "border-border",
      )}>
      <div className="rounded-sm border border-border bg-background p-2 transition-transform group-hover:scale-105">
        <FolderIcon className="size-4 text-muted-foreground" />
      </div>
      <div className="min-w-0 flex-1">
        <p
          className={cn(
            "truncate text-sm text-muted-foreground/70",
            path && "text-foreground",
          )}>
          {path || "Select a folder..."}
        </p>
      </div>
    </div>
    {touched === true && error != null && (
      <p className="ml-1 text-xs text-destructive">{error}</p>
    )}
  </div>
);
