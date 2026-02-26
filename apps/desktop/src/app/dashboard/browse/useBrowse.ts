import { FilesystemItem, tauriApi } from "@/libraries/tauri-api";
import { useEffect, useState } from "react";

type PathSegment = {
  id: string | null;
  name: string;
};

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([{ id: null, name: "Root" }]);
  const [currentFiles, setCurrentFiles] = useState<FilesystemItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const currentFolder = currentPath[currentPath.length - 1] ?? {
    id: "00000000-00000000-00000000-00000000",
    name: "/",
  };

  const fetchFiles = async (folderId: string | null) => {
    setIsLoading(true);
    const { data, error } = await tauriApi.safeInvoke("browse_vault", {
      parentId: folderId,
    });

    if (data != null && error == null) {
      setCurrentFiles(data);
    }
    setIsLoading(false);
  };

  useEffect(() => {
    void fetchFiles(currentFolder.id);
  }, [currentFolder.id]);

  const handleFolderClick = (item: FilesystemItem) => {
    if (item.type === "folder") {
      setCurrentPath(prev => [...prev, { id: item.id, name: item.name }]);
    }
  };

  const handleBreadcrumbClick = (index: number) => {
    setCurrentPath(prev => prev.slice(0, index + 1));
  };

  const handleResetPath = () => {
    setCurrentPath([{ id: null, name: "Root" }]);
  };

  const handleCreateFolder = async (name: string) => {
    const { error } = await tauriApi.safeInvoke("create_folder", {
      parentId: currentFolder.id,
      name,
    });

    if (error == null) {
      await fetchFiles(currentFolder.id);
    }
  };

  const handleDeleteItem = async (id: string, itemType: "file" | "folder") => {
    const { error } = await tauriApi.safeInvoke("delete_item", {
      id,
      itemType,
    });

    if (error == null) {
      await fetchFiles(currentFolder.id);
    }
  };

  const handleRenameItem = async (
    id: string,
    itemType: "file" | "folder",
    newName: string,
  ) => {
    const { error } = await tauriApi.safeInvoke("rename_item", {
      id,
      itemType,
      newName,
    });

    if (error == null) {
      await fetchFiles(currentFolder.id);
    }
  };

  return {
    currentPath: currentPath.map(p => p.name),
    currentFiles,
    isLoading,
    handleFolderClick,
    handleBreadcrumbClick,
    handleResetPath,
    handleCreateFolder,
    handleDeleteItem,
    handleRenameItem,
    refresh: () => fetchFiles(currentFolder.id),
  };
};
