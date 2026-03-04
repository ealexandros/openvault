import { tauriApi } from "@/libraries/tauri-api";
import { ItemType, type FileItemResult } from "@/types/filesystem";
import { open } from "@tauri-apps/plugin-dialog";
import { useRef, useState } from "react";
import { toast } from "sonner";
import { type FileRenamingItem, type ViewingItem } from "../types";

type UseFileOptions = {
  currentFolderId: string;
  files: FileItemResult[];
  searchQuery: string;
  refresh: () => Promise<void>;
};

const sortFiles = (items: FileItemResult[]) =>
  [...items].sort((left, right) => {
    if (left.isFavourite !== right.isFavourite) {
      return left.isFavourite ? -1 : 1;
    }

    return left.name.localeCompare(right.name);
  });

export const useFile = ({ currentFolderId, files, searchQuery, refresh }: UseFileOptions) => {
  const [renamingItem, setRenamingItem] = useState<FileRenamingItem | null>(null);
  const [viewingItem, setViewingItem] = useState<ViewingItem | null>(null);

  const previewSequenceRef = useRef(0);

  const uploadPath = async (path: string) => {
    const isFile = await tauriApi.isFile({ path });

    if (isFile.success && !isFile.data) {
      return await tauriApi.uploadFolder({ parentId: currentFolderId, sourcePath: path });
    }

    return await tauriApi.uploadFile({ parentId: currentFolderId, sourcePath: path });
  };

  const uploadPaths = async (paths: string[]) => {
    if (paths.length === 0) {
      return;
    }

    const toastId = toast.loading("Uploading files");
    const results = await Promise.all(paths.map(path => uploadPath(path)));
    toast.dismiss(toastId);

    if (results.some(result => result.success)) {
      await refresh();
    }
  };

  const handleUploadFile = async () => {
    const selected = await open({
      multiple: true,
      directory: false,
    });

    if (selected) {
      await uploadPaths(selected);
    }
  };

  const handleUploadFolder = async () => {
    const selected = await open({
      multiple: false,
      directory: true,
    });

    if (selected != null) {
      await uploadPaths([selected]);
      await refresh();
    }
  };

  const handleDeleteFile = async (id: string) => {
    const result = await tauriApi.deleteItem({ id, itemType: "file" });

    if (result.success) {
      await refresh();
    }
  };

  const handleRenameFile = async (id: string, newName: string) => {
    const trimmedName = newName.trim();
    if (!trimmedName) {
      return;
    }

    const result = await tauriApi.renameItem({
      id,
      itemType: "file",
      newName: trimmedName,
    });

    if (result.success) {
      await refresh();
    }
  };

  const handleRequestFileRename = (file: FileItemResult) => {
    setRenamingItem({ id: file.id, name: file.name, type: "file" });
  };

  const clearRenamingItem = () => {
    setRenamingItem(null);
  };

  const renameRenamingItem = async (newName: string) => {
    if (!renamingItem) {
      return;
    }

    await handleRenameFile(renamingItem.id, newName);
  };

  const getFileContent = async (id: string) => {
    const result = await tauriApi.readFileBytes({ id });

    if (!result.success) {
      return null;
    }

    return result.data;
  };

  const handleFileClick = async (item: Pick<FileItemResult, "id" | "name" | "extension">) => {
    const previewSequence = ++previewSequenceRef.current;

    setViewingItem({
      id: item.id,
      name: item.name,
      extension: item.extension,
      content: null,
    });

    const content = await getFileContent(item.id);

    if (previewSequenceRef.current !== previewSequence) {
      return;
    }

    setViewingItem(previousItem =>
      previousItem?.id === item.id ? { ...previousItem, content } : previousItem,
    );
  };

  const handleFileViewerOpenChange = (open: boolean) => {
    if (open) {
      return;
    }

    previewSequenceRef.current += 1;
    setViewingItem(null);
  };

  const handleToggleFavourite = async (file: FileItemResult) => {
    const result = await tauriApi.setFavourtieItem({
      id: file.id,
      isFavourite: !file.isFavourite,
      itemType: ItemType.FILE,
    });

    if (result.success) {
      await refresh();
    }
  };

  const handleExportFile = async (id: string, destinationPath: string) => {
    const result = await tauriApi.exportFile({ id, destinationPath });

    if (result.success) {
      toast.success("File exported successfully");
    } else {
      toast.error("Failed to export file");
    }
  };

  const normalizedSearch = searchQuery.trim().toLowerCase();
  const filteredFiles = normalizedSearch
    ? files.filter(item =>
        `${item.name}.${item.extension}`.toLowerCase().includes(normalizedSearch),
      )
    : files;
  const sortedFiles = sortFiles(filteredFiles);

  return {
    files: sortedFiles,
    renamingItem,
    viewingItem,
    uploadPaths,
    handleUploadFile,
    handleUploadFolder,
    handleDeleteFile,
    handleRequestFileRename,
    clearRenamingItem,
    renameRenamingItem,
    handleFileClick,
    handleFileViewerOpenChange,
    handleToggleFavourite,
    handleExportFile,
  };
};
