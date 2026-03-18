import { tauriApi } from "@/libraries/tauri-api";
import { useForm } from "@tanstack/react-form";
import { toast } from "sonner";
import z from "zod";

type UseCreateFolderDialogParams = {
  parentId?: string;
  onSuccess?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const useCreateFolderDialog = ({
  parentId,
  onSuccess,
  onOpenChange,
}: UseCreateFolderDialogParams) => {
  const form = useForm({
    defaultValues: {
      name: "",
    },
    onSubmit: async ({ value }) => {
      const result = await tauriApi.createFolder({
        parentId,
        name: value.name.trim(),
      });

      if (!result.success) {
        toast.error("Failed to create folder");
        return;
      }

      onSuccess?.();
      onOpenChange(false);
      form.reset();
    },
    validators: {
      onSubmit: z.object({
        name: z.string().refine(name => name.trim() !== "", "Folder name cannot be empty"),
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
