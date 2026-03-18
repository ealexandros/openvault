"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { FolderItemResult } from "@/types/filesystem";
import { FOLDER_ICON_OPTIONS } from "../../../data/folder-icons";
import { useUpdateFolderIconDialog } from "./useUpdateFolderIconDialog";

type UpdateFolderIconDialogProps = {
  isOpen: boolean;
  item: FolderItemResult | null;
  onUpdate?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const UpdateFolderIconDialog = ({
  isOpen,
  item,
  onUpdate,
  onOpenChange,
}: UpdateFolderIconDialogProps) => {
  const { isUpdating, handleIconSelect, handleOpenChange } = useUpdateFolderIconDialog({
    item,
    onUpdate,
    onOpenChange,
  });

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-base">Change folder icon</DialogTitle>
          <DialogDescription className="text-sm">
            Pick an icon for this folder.
          </DialogDescription>
        </DialogHeader>

        <div className="grid grid-cols-5 gap-2">
          {FOLDER_ICON_OPTIONS.map(({ name, Icon }) => (
            <Button
              key={name}
              type="button"
              variant="outline"
              onClick={() => handleIconSelect(name)}
              disabled={isUpdating}
              className="h-12 rounded-lg p-0"
              title={name}>
              <Icon className="size-5" />
              <span className="sr-only">{name}</span>
            </Button>
          ))}
        </div>
      </DialogContent>
    </Dialog>
  );
};
