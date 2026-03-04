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
import { Input } from "@/components/ui/shadcn/input";
import { ItemType } from "@/types/filesystem";
import { useState } from "react";

type RenameItemDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onRename: (newName: string) => Promise<void>;
  initialName: string;
  itemType: ItemType;
};

// @todo-now make the error state different..

export const RenameItemDialog = ({
  isOpen,
  onOpenChange,
  onRename,
  initialName,
  itemType,
}: RenameItemDialogProps) => {
  const [name, setName] = useState(initialName);
  const [isRenaming, setIsRenaming] = useState(false);

  const handleOpenChange = (open: boolean) => {
    if (open) {
      setName(initialName);
    }
    onOpenChange(open);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || name.trim() === initialName) {
      onOpenChange(false);
      return;
    }

    setIsRenaming(true);
    await onRename(name.trim());
    setIsRenaming(false);
    onOpenChange(false);
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-base">Rename {itemType}</DialogTitle>
          <DialogDescription className="text-sm">
            Enter a new name for this {itemType}.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <Input
            value={name}
            onChange={e => setName(e.target.value)}
            placeholder="New name..."
            autoFocus
            className="h-10 rounded-lg"
          />
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isRenaming}
              className="p-4">
              Cancel
            </Button>
            <Button type="submit" disabled={!name.trim() || isRenaming} className="p-4">
              {isRenaming ? "Renaming..." : "Rename"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
