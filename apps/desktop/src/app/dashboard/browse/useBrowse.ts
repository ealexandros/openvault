import { FileItem, FolderItem, tauriApi } from "@/libraries/tauri-api";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useState } from "react";

type PathSegment = {
  id: string;
  name: string;
};

const ROOT_FOLDER_ID = "00000000-0000-0000-0000-000000000000";

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
    const { data, error } = await tauriApi.safeInvoke("browse_vault", {
      parentId: folderId,
    });

    if (data != null && error == null) {
      setFolders(data.folders);
      setFiles(data.files);
    }
    setIsLoading(false);
  }, []);

  useEffect(() => {
    void fetchFiles(currentFolder.id);
  }, [currentFolder.id, fetchFiles]);

  const performUpload = async (path: string) => {
    const { error } = await tauriApi.safeInvoke("upload_file", {
      parentId: currentFolder.id,
      sourcePath: path,
    });

    if (error == null) {
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
    const { data, error } = await tauriApi.safeInvoke("get_file_content", { id });
    if (error != null) {
      return null;
    }
    return data;
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
