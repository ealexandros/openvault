import { tauriApi } from "@/libraries/tauri-api";
import { FolderItemResult } from "@/types/filesystem";
import { useState } from "react";
import { toast } from "sonner";

type UseUpdateFolderIconDialogProps = {
  item: FolderItemResult | null;
  onUpdate?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const useUpdateFolderIconDialog = ({
  item,
  onUpdate,
  onOpenChange,
}: UseUpdateFolderIconDialogProps) => {
  const [isUpdating, setIsUpdating] = useState(false);

  const handleIconSelect = async (iconName: string) => {
    if (item == null) return;

    setIsUpdating(true);
    const result = await tauriApi.setFolderIcon({
      id: item.id,
      icon: iconName,
    });
    setIsUpdating(false);

    if (!result.success) {
      toast.error("Failed to change folder icon");
      return;
    }

    onUpdate?.();
    onOpenChange(false);
  };

  const handleOpenChange = (open: boolean) => {
    if (!isUpdating) onOpenChange(open);
  };

  return {
    isUpdating,
    handleIconSelect,
    handleOpenChange,
  };
};
