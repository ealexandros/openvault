"use client";

import { Label } from "@/components/ui/shadcn/label";
import { open } from "@tauri-apps/plugin-dialog";
import { FolderIcon } from "lucide-react";

// @todo-soon refactor this..

type LocationSelectorProps = {
  path: string;
  error?: string;
  touched?: boolean;
  setFieldValue: (field: string, value: unknown) => void;
};

export function LocationSelector({
  path,
  error,
  touched,
  setFieldValue,
}: LocationSelectorProps) {
  const handleSelectFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select folder to encrypt",
      });
      if (selected != null && typeof selected === "string") {
        setFieldValue("path", selected);
      }
    } catch (error) {
      // eslint-disable-next-line no-console
      console.error("Failed to open folder picker:", error);
    }
  };

  return (
    <div className="space-y-2">
      <Label className="ml-1 text-[11px] font-bold tracking-widest text-muted-foreground uppercase">
        Location
      </Label>
      <div
        onClick={handleSelectFolder}
        className={`group flex cursor-pointer items-center gap-3 rounded-2xl border ${
          touched === true && error != null ? "border-red-500" : "border-border"
        } bg-muted/30 p-4 transition-all hover:border-primary/30`}>
        <div className="rounded-lg border border-border bg-background p-2 shadow-sm transition-transform group-hover:scale-105">
          <FolderIcon className="h-4 w-4 text-muted-foreground" />
        </div>
        <div className="min-w-0 flex-1">
          <p className="truncate text-sm font-medium">{path || "Select a folder..."}</p>
        </div>
      </div>
      {touched === true && error != null && (
        <p className="ml-1 text-[10px] font-medium text-red-500">{error}</p>
      )}
    </div>
  );
}
