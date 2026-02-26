import { FilesystemItem, tauriApi } from "@/libraries/tauri-api";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useState } from "react";

type PathSegment = {
  id: string | null;
  name: string;
};

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([{ id: null, name: "Root" }]);
  const [currentFiles, setCurrentFiles] = useState<FilesystemItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isDragging, setIsDragging] = useState(false);

  const currentFolder = currentPath[currentPath.length - 1] ?? {
    id: "00000000-00000000-00000000-00000000",
    name: "/",
  };

  const fetchFiles = useCallback(async (folderId: string | null) => {
    setIsLoading(true);
    const { data, error } = await tauriApi.safeInvoke("browse_vault", {
      parentId: folderId,
    });

    if (data != null && error == null) {
      setCurrentFiles(data);
    }
    setIsLoading(false);
  }, []);

  useEffect(() => {
    // @todo-now fix this..
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void fetchFiles(currentFolder.id);
  }, [currentFolder.id, fetchFiles]);

  const performUpload = useCallback(
    async (path: string) => {
      const { error } = await tauriApi.safeInvoke("upload_file", {
        parentId: currentFolder.id,
        sourcePath: path,
      });

      if (error == null) {
        await fetchFiles(currentFolder.id);
      }
    },
    [currentFolder.id, fetchFiles],
  );

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
  }, [performUpload]);

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
    currentFiles,
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
