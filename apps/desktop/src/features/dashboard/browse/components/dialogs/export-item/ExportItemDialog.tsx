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
import { ExportTarget } from "@/features/dashboard/browse/types";
import { AlertTriangle, FolderOpenIcon } from "lucide-react";
import { useExportDialog } from "./useExportDialog";

type ExportItemDialogProps = {
  isOpen: boolean;
  item: ExportTarget | null;
  onExport?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const ExportItemDialog = ({
  isOpen,
  item,
  onExport,
  onOpenChange,
}: ExportItemDialogProps) => {
  const { destinationPath, isExporting, handleBrowse, handleExport, handleOpenChange } =
    useExportDialog({ item, onExport, onOpenChange });

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="text-base">Unencrypt {item?.type}</DialogTitle>
          <DialogDescription className="text-sm">
            You are about to export &quot;{item?.name}&quot; to your computer.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleExport} className="space-y-6 py-4">
          <Alert variant="warning" className="rounded-xl border-yellow-500/50 bg-yellow-500/5">
            <AlertTriangle className="size-4 text-yellow-600" />
            <AlertTitle className="text-yellow-600">Security Warning</AlertTitle>
            <AlertDescription className="text-yellow-700/80">
              The exported data will be exposed on your computer in an unencrypted format.
              Ensure the destination is secure before proceeding.
            </AlertDescription>
          </Alert>

          <div className="space-y-2">
            <label className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
              Destination Directory
            </label>
            <div className="flex gap-2">
              <Input
                value={destinationPath}
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

          <DialogFooter className="gap-2 sm:justify-end">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isExporting}
              className="h-10 rounded-lg p-4 px-6">
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={!destinationPath.trim() || isExporting}
              className="h-10 rounded-lg border-none bg-yellow-600 px-6 text-white transition-all hover:bg-yellow-700 disabled:bg-muted disabled:text-muted-foreground">
              {isExporting ? "Exporting..." : "Unencrypt"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
