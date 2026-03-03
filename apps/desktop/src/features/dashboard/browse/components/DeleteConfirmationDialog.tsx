import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { useState } from "react";

type DeleteConfirmationDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  itemName: string;
  itemType: "file" | "folder";
  onConfirm: () => Promise<void>;
};

export const DeleteConfirmationDialog = ({
  isOpen,
  onOpenChange,
  itemName,
  itemType,
  onConfirm,
}: DeleteConfirmationDialogProps) => {
  const [isDeleting, setIsDeleting] = useState(false);

  const handleOpenChange = (open: boolean) => {
    if (isDeleting) {
      return;
    }

    onOpenChange(open);
  };

  const handleConfirm = async () => {
    setIsDeleting(true);

    try {
      await onConfirm();
      onOpenChange(false);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-base">Delete {itemType}</DialogTitle>
          <DialogDescription className="text-sm">
            Are you sure you want to delete &ldquo;{itemName}&rdquo;? If you delete it,
            there is nothing to undo this operation.
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
            onClick={() => {
              void handleConfirm();
            }}
            disabled={isDeleting}
            className="p-4">
            {isDeleting ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
