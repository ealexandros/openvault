import { DeletionTarget } from "@/features/dashboard/browse/types";
import { tauriApi } from "@/libraries/tauri-api";
import { useState } from "react";
import { toast } from "sonner";

type UseDeleteDialogParams = {
  item: DeletionTarget | null;
  onDelete?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const useDeleteDialog = ({ item, onDelete, onOpenChange }: UseDeleteDialogParams) => {
  const [isDeleting, setIsDeleting] = useState(false);

  const handleDelete = async () => {
    if (item == null) return;

    setIsDeleting(true);
    const result = await tauriApi.deleteItem({
      id: item.id,
      itemType: item.type,
    });
    setIsDeleting(false);

    if (!result.success) {
      toast.error(`Failed to delete ${item.type} "${item.name}"`);
      return;
    }

    onDelete?.();
    onOpenChange(false);
  };

  const handleOpenChange = (open: boolean) => {
    if (!isDeleting) onOpenChange(open);
  };

  return {
    isDeleting,
    handleDelete,
    handleOpenChange,
  };
};
