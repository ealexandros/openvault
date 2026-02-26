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
import { useEffect, useState } from "react";

type RenameItemDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onRename: (newName: string) => Promise<void>;
  initialName: string;
  itemType: "file" | "folder";
};

export const RenameItemDialog = ({
  isOpen,
  onOpenChange,
  onRename,
  initialName,
  itemType,
}: RenameItemDialogProps) => {
  const [name, setName] = useState(initialName);
  const [isRenaming, setIsRenaming] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setName(initialName);
    }
  }, [isOpen, initialName]);

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
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename {itemType}</DialogTitle>
          <DialogDescription>Enter a new name for this {itemType}.</DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <Input
            value={name}
            onChange={e => setName(e.target.value)}
            placeholder="New name..."
            autoFocus
            className="h-9 rounded-lg"
          />
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isRenaming}>
              Cancel
            </Button>
            <Button type="submit" disabled={!name.trim() || isRenaming}>
              {isRenaming ? "Renaming..." : "Rename"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
