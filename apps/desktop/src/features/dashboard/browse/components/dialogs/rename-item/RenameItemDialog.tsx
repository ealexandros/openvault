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
import { RenameTarget } from "@/features/dashboard/browse/types";
import { cn } from "@/utils/cn";
import { useRenameDialog } from "./useRenameDialog";

type RenameItemDialogProps = {
  isOpen: boolean;
  item: RenameTarget | null;
  onRename?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const RenameItemDialog = ({
  isOpen,
  item,
  onRename,
  onOpenChange,
}: RenameItemDialogProps) => {
  const { form, handleSubmit, handleOpenChange } = useRenameDialog({
    item,
    onRename,
    onOpenChange,
  });

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-base">Rename {item?.type}</DialogTitle>
          <DialogDescription className="text-sm">
            Enter a new name for this {item?.type}.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <form.Field name="name">
            {field => (
              <>
                <Input
                  value={field.state.value}
                  onChange={e => field.handleChange(e.target.value)}
                  placeholder="New name..."
                  autoFocus
                  className={cn(
                    "h-10 rounded-lg transition-all",
                    field.state.meta.errors.length > 0 &&
                      "border-destructive ring-destructive/10 focus-visible:ring-destructive/40",
                  )}
                />
                {field.state.meta.errors.length > 0 && (
                  <p className="text-[10px] font-medium text-destructive">
                    {(field.state.meta.errors as unknown as string[])[0]}
                  </p>
                )}
              </>
            )}
          </form.Field>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleOpenChange(false)}
              disabled={form.state.isSubmitting}
              className="p-4">
              Cancel
            </Button>
            <Button type="submit" disabled={form.state.isSubmitting} className="p-4">
              {form.state.isSubmitting ? "Renaming..." : "Rename"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
