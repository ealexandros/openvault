"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { DeletionTarget } from "@/features/dashboard/browse/types";
import { useDeleteDialog } from "./useDeleteDialog";

type DeleteItemDialogProps = {
  isOpen: boolean;
  item: DeletionTarget | null;
  onDelete?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const DeleteItemDialog = ({
  isOpen,
  item,
  onDelete,
  onOpenChange,
}: DeleteItemDialogProps) => {
  const { isDeleting, handleDelete, handleOpenChange } = useDeleteDialog({
    item,
    onDelete,
    onOpenChange,
  });

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-base">Delete {item?.type}</DialogTitle>
          <DialogDescription className="text-sm">
            Are you sure you want to delete <b>&ldquo;{item?.name}&rdquo;</b>? If you delete
            it, there is nothing to undo this operation.
          </DialogDescription>
        </DialogHeader>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
            disabled={isDeleting}
            className="p-4">
            Cancel
          </Button>
          <Button
            type="button"
            variant="destructive"
            onClick={handleDelete}
            disabled={isDeleting}
            className="p-4">
            {isDeleting ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
