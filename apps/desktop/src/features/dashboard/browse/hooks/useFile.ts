import { tauriApi } from "@/libraries/tauri-api";
import { type FileItem } from "@/types/filesystem";
import { open } from "@tauri-apps/plugin-dialog";
import { useRef, useState } from "react";
import { type FileRenamingItem, type ViewingItem } from "../types";

type UseFileOptions = {
  currentFolderId: string;
  files: FileItem[];
  searchQuery: string;
  refresh: () => Promise<void>;
};

export const useFile = ({ currentFolderId, files, searchQuery, refresh }: UseFileOptions) => {
  const [renamingItem, setRenamingItem] = useState<FileRenamingItem | null>(null);
  const [viewingItem, setViewingItem] = useState<ViewingItem | null>(null);

  const previewSequenceRef = useRef(0);

  const uploadPaths = async (paths: string[]) => {
    if (paths.length === 0) {
      return;
    }

    const results = await Promise.all(
      paths.map(path =>
        tauriApi.uploadFile({
          parentId: currentFolderId,
          sourcePath: path,
        }),
      ),
    );

    if (results.some(result => result.success)) {
      await refresh();
    }
  };

  const handleUploadFile = async () => {
    const selected = await open({
      multiple: false,
      directory: false,
    });

    if (typeof selected === "string") {
      await uploadPaths([selected]);
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

  const handleRequestFileRename = (item: FileItem) => {
    setRenamingItem({ id: item.id, name: item.name, type: "file" });
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
    const result = await tauriApi.getFileContent({ id });

    if (!result.success) {
      return null;
    }

    return result.data;
  };

  const handleFileClick = async (item: Pick<FileItem, "id" | "name" | "extension">) => {
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

  const normalizedSearch = searchQuery.trim().toLowerCase();
  const filteredFiles = normalizedSearch
    ? files.filter(item =>
        `${item.name}.${item.extension}`.toLowerCase().includes(normalizedSearch),
      )
    : files;

  return {
    files: filteredFiles,
    renamingItem,
    viewingItem,
    uploadPaths,
    handleUploadFile,
    handleDeleteFile,
    handleRequestFileRename,
    clearRenamingItem,
    renameRenamingItem,
    handleFileClick,
    handleFileViewerOpenChange,
  };
};
