import { tauriApi } from "@/libraries/tauri-api";
import { type FileItem, type FolderItem } from "@/types/filesystem";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useState } from "react";

type PathSegment = {
  id: string;
  name: string;
};

const ROOT_FOLDER_ID = "00000000-0000-0000-0000-000000000000";

// @todo-now refactor everything from here..
// @todo-now performUpload executes twice

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([
    { id: ROOT_FOLDER_ID, name: "/" },
  ]);
  const [folders, setFolders] = useState<FolderItem[]>([]);
  const [files, setFiles] = useState<FileItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isDragging, setIsDragging] = useState(false);

  const currentFolder = currentPath[currentPath.length - 1] ?? {
    id: ROOT_FOLDER_ID,
    name: "/",
  };

  const fetchFiles = useCallback(async (folderId: string) => {
    setIsLoading(true);
    const result = await tauriApi.browseVault({ parentId: folderId });

    if (result.success) {
      setFolders(result.data.folders);
      setFiles(result.data.files);
    }
    setIsLoading(false);
  }, []);

  useEffect(() => {
    void fetchFiles(currentFolder.id);
  }, [currentFolder.id, fetchFiles]);

  const performUpload = async (path: string) => {
    const result = await tauriApi.uploadFile({ parentId: currentFolder.id, sourcePath: path });

    if (result.success) {
      await fetchFiles(currentFolder.id);
    }
  };

  useEffect(() => {
    const unlisten: (() => void)[] = [];

    const setupListeners = async () => {
      const u1 = await getCurrentWindow().listen("tauri://drag-enter", () => {
        setIsDragging(true);
      });
      const u2 = await getCurrentWindow().listen("tauri://drag-leave", () => {
        setIsDragging(false);
      });
      const u3 = await getCurrentWindow().listen<{ paths: string[] }>(
        "tauri://drag-drop",
        event => {
          setIsDragging(false);
          void (async () => {
            for (const path of event.payload.paths) {
              await performUpload(path);
            }
          })();
        },
      );
      unlisten.push(u1, u2, u3);
    };

    void setupListeners();

    return () => {
      unlisten.forEach(u => u());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleFolderClick = (item: FolderItem) => {
    setCurrentPath(prev => [...prev, { id: item.id, name: item.name }]);
  };

  const handleBreadcrumbClick = (index: number) => {
    setCurrentPath(prev => prev.slice(0, index + 1));
  };

  const handleResetPath = () => {
    setCurrentPath([{ id: ROOT_FOLDER_ID, name: "/" }]);
  };

  const handleCreateFolder = async (name: string) => {
    const result = await tauriApi.createFolder({ parentId: currentFolder.id, name });

    if (result.success) {
      await fetchFiles(currentFolder.id);
    }
  };

  const handleDeleteItem = async (id: string, itemType: "file" | "folder") => {
    const result = await tauriApi.deleteItem({ id, itemType });

    if (result.success) {
      await fetchFiles(currentFolder.id);
    }
  };

  const handleRenameItem = async (
    id: string,
    itemType: "file" | "folder",
    newName: string,
  ) => {
    const result = await tauriApi.renameItem({ id, itemType, newName });

    if (result.success) {
      await fetchFiles(currentFolder.id);
    }
  };

  const handleUploadFile = async () => {
    const selected = await open({
      multiple: false,
      directory: false,
    });

    if (typeof selected === "string") {
      await performUpload(selected);
    }
  };

  const getFileContent = async (id: string) => {
    const result = await tauriApi.getFileContent({ id });
    if (!result.success) {
      return null;
    }
    return result.data;
  };

  return {
    currentPath: currentPath.map(p => p.name),
    folders,
    files,
    isLoading,
    isDragging,
    handleFolderClick,
    handleBreadcrumbClick,
    handleResetPath,
    handleCreateFolder,
    handleDeleteItem,
    handleRenameItem,
    handleUploadFile,
    getFileContent,
    refresh: () => fetchFiles(currentFolder.id),
  };
};
