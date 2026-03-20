"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/shadcn/dialog";
import { Input } from "@/components/ui/shadcn/input";
import { PropsWithChildren, useState } from "react";
import { useCreateFolderDialog } from "./useCreateFolderDialog";

type CreateFolderDialogProps = PropsWithChildren & {
  parentId?: string;
  onSuccess?: () => void;
};

export const CreateFolderDialog = ({
  children,
  parentId,
  onSuccess,
}: CreateFolderDialogProps) => {
  const [isOpen, setIsOpen] = useState(false);

  const { form, handleSubmit, handleOpenChange } = useCreateFolderDialog({
    parentId,
    onSuccess,
    onOpenChange: setIsOpen,
  });

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-base font-semibold">New Folder</DialogTitle>
          <DialogDescription className="text-sm">
            Enter a name for your new folder inside the vault.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <form.Field name="name">
            {field => (
              <div className="space-y-1">
                <Input
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onBlur={field.handleBlur}
                  onChange={e => field.handleChange(e.target.value)}
                  placeholder="Folder name..."
                  autoFocus
                  className="h-10 rounded-lg"
                />
                {field.state.meta.errors.length > 0 && (
                  <p className="text-xs font-medium text-destructive">
                    {field.state.meta.errors[0] as unknown as string}
                  </p>
                )}
              </div>
            )}
          </form.Field>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleOpenChange(false)}
              disabled={form.state.isSubmitting}>
              Cancel
            </Button>
            <Button type="submit" disabled={form.state.isSubmitting}>
              {form.state.isSubmitting ? "Creating..." : "Create Folder"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
