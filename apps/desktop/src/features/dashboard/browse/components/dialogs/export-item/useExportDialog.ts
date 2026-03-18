import { ExportTarget } from "@/features/dashboard/browse/types";
import { tauriApi } from "@/libraries/tauri-api";
import { ItemType } from "@/types/filesystem";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { toast } from "sonner";

type UseExportDialogParams = {
  item: ExportTarget | null;
  onExport?: () => void;
  onOpenChange: (open: boolean) => void;
};

export const useExportDialog = ({ item, onExport, onOpenChange }: UseExportDialogParams) => {
  const [destinationPath, setDestinationPath] = useState("");
  const [isExporting, setIsExporting] = useState(false);

  const handleBrowse = async () => {
    if (!item) return;

    const selected = await open({
      directory: true,
      multiple: false,
      title: `Select destination directory for ${item.type} export`,
    });

    if (selected != null && typeof selected === "string") {
      setDestinationPath(selected);
    }
  };

  const handleExport = async (e: React.FormEvent) => {
    e.preventDefault();

    const targetPath = destinationPath.trim();
    if (item == null || targetPath === "") return;

    setIsExporting(true);

    const result =
      item.type === ItemType.FILE
        ? await tauriApi.exportFile({ id: item.id, destinationPath: targetPath })
        : await tauriApi.exportFolder({ id: item.id, destinationPath: targetPath });

    setIsExporting(false);

    if (!result.success) {
      toast.error(`Failed to export ${item.type} "${item.name}"`);
      return;
    }
    toast.success(`${item.type} exported successfully`);

    onOpenChange(false);
    onExport?.();
    setDestinationPath("");
  };

  const handleOpenChange = (open: boolean) => {
    if (!isExporting) onOpenChange(open);
  };

  return {
    destinationPath,
    isExporting,
    setDestinationPath,
    handleBrowse,
    handleExport,
    handleOpenChange,
  };
};
