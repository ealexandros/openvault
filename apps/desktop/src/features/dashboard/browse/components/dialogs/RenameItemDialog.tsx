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
import { cn } from "@/utils/cn";
import { useState } from "react";

type RenameItemDialogProps = {
  isOpen: boolean;
  initialName: string;
  itemType: ItemType;
  onOpenChange: (open: boolean) => void;
  onRename: (newName: string) => Promise<boolean>;
};

export const RenameItemDialog = ({
  isOpen,
  initialName,
  itemType,
  onOpenChange,
  onRename,
}: RenameItemDialogProps) => {
  const [name, setName] = useState(initialName);
  const [isRenaming, setIsRenaming] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleOpenChange = (open: boolean) => {
    onOpenChange(open);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmedName = name.trim();

    if (!trimmedName || trimmedName === initialName) {
      onOpenChange(false);
      return;
    }

    setIsRenaming(true);
    setError(null);

    const success = await onRename(trimmedName);

    setIsRenaming(false);

    if (success) {
      onOpenChange(false);
    } else {
      setError("Name already exists or invalid.");
    }
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
          <div className="space-y-2">
            <Input
              value={name}
              onChange={e => {
                setName(e.target.value);
                if (error != null) setError(null);
              }}
              placeholder="New name..."
              autoFocus
              className={cn(
                "h-10 rounded-lg transition-all",
                error !== null &&
                  "border-destructive ring-destructive/10 focus-visible:ring-destructive/40",
              )}
            />
            {error != null && (
              <p className="text-[10px] font-medium text-destructive">{error}</p>
            )}
          </div>
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
