"use client";

import { Alert, AlertDescription, AlertTitle } from "@/components/ui/shadcn/alert";
import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { Input } from "@/components/ui/shadcn/input";
import { logger } from "@/libraries/logger";
import { ItemType } from "@/types/filesystem";
import { open } from "@tauri-apps/plugin-dialog";
import { AlertTriangle, FolderOpenIcon } from "lucide-react";
import { useState } from "react";

type ExportItemDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onExport: (destinationPath: string) => Promise<void>;
  itemName: string;
  itemType: ItemType;
};

export const ExportItemDialog = ({
  isOpen,
  onOpenChange,
  onExport,
  itemName,
  itemType,
}: ExportItemDialogProps) => {
  const [destinationPath, setDestinationPath] = useState("");
  const [isExporting, setIsExporting] = useState(false);

  const handleOpenChange = (open: boolean) => {
    if (open) {
      setDestinationPath("");
    }
    onOpenChange(open);
  };

  const handleBrowse = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: `Select destination directory for ${itemType} export`,
    });

    if (selected != null && typeof selected === "string") {
      setDestinationPath(selected);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!destinationPath.trim()) {
      return;
    }

    setIsExporting(true);
    try {
      await onExport(destinationPath.trim());
      onOpenChange(false);
    } catch (error) {
      logger.error("Export failed", error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="text-base">Unencrypt {itemType}</DialogTitle>
          <DialogDescription className="text-sm">
            You are about to export &quot;{itemName}&quot; to your computer.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6 py-4">
          <Alert variant="warning">
            <AlertTriangle className="size-4" />
            <AlertTitle>Security Warning</AlertTitle>
            <AlertDescription>
              The exported data will be exposed on your computer in an unencrypted format.
              Ensure the destination is secure before proceeding.
            </AlertDescription>
          </Alert>

          <div className="space-y-2">
            <label className="text-xs font-semibold text-muted-foreground uppercase">
              Destination Directory
            </label>
            <div className="flex gap-2">
              <Input
                value={destinationPath}
                onChange={e => setDestinationPath(e.target.value)}
                placeholder="Choose where to save..."
                className="h-10 grow rounded-lg bg-muted/50 focus-visible:ring-0"
                readOnly
              />
              <Button
                type="button"
                variant="outline"
                size="icon"
                onClick={handleBrowse}
                className="h-10 w-10 shrink-0 rounded-lg">
                <FolderOpenIcon className="size-4" />
              </Button>
            </div>
          </div>
        </div>

        <DialogFooter className="gap-2 sm:justify-end">
          <Button
            type="button"
            variant="ghost"
            onClick={() => onOpenChange(false)}
            disabled={isExporting}
            className="px-4">
            Cancel
          </Button>
          <Button
            type="button"
            onClick={handleSubmit}
            disabled={!destinationPath.trim() || isExporting}
            className="border-none bg-yellow-600 px-4 py-4 text-white transition-all hover:bg-yellow-700">
            {isExporting ? "Exporting..." : "Unencrypt"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
