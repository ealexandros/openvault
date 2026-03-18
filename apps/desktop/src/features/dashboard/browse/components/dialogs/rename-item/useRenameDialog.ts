import { RenameTarget } from "@/features/dashboard/browse/types";
import { tauriApi } from "@/libraries/tauri-api";
import { useForm } from "@tanstack/react-form";
import { toast } from "sonner";
import z from "zod";

type UseRenameDialogParams = {
  item: RenameTarget | null;
  onRename?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const useRenameDialog = ({ item, onRename, onOpenChange }: UseRenameDialogParams) => {
  const form = useForm({
    defaultValues: {
      name: item?.name ?? "",
    },
    onSubmit: async ({ value }) => {
      if (item == null) return;

      const result = await tauriApi.renameItem({
        id: item.id,
        newName: value.name,
        itemType: item.type,
      });

      if (!result.success) {
        toast.error(`Failed to rename item names "${item.name}"`);
        return;
      }

      onRename?.();
      onOpenChange(false);
    },
    validators: {
      onSubmit: z.object({
        name: z
          .string()
          .refine(name => name.trim() !== "", "Name cannot be empty")
          .refine(name => name.trim() !== item?.name, "Name must be different"),
      }),
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    e.stopPropagation();
    void form.handleSubmit();
  };

  const handleOpenChange = (open: boolean) => {
    if (!form.state.isSubmitting) onOpenChange(open);
  };

  return {
    form,
    handleSubmit,
    handleOpenChange,
  };
};
