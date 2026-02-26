import { useFileDragDrop } from "@/hooks/useFileDragDrop";
import { tauriApi } from "@/libraries/tauri-api";
import { type FileItem, type FolderItem } from "@/types/filesystem";
import { open } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useState } from "react";

type PathSegment = {
  id: string;
  name: string;
};

const ROOT_FOLDER_ID = "00000000-0000-0000-0000-000000000000";
const ROOT_FOLDER: PathSegment = { id: ROOT_FOLDER_ID, name: "Home" };

// @todo-now refactor everything from here..
// @todo-now performUpload executes twice

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([ROOT_FOLDER]);
  const [folders, setFolders] = useState<FolderItem[]>([]);
  const [files, setFiles] = useState<FileItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const currentFolder = currentPath[currentPath.length - 1] ?? ROOT_FOLDER;

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
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void fetchFiles(currentFolder.id);
  }, [currentFolder.id, fetchFiles]);

  const performUpload = useCallback(
    async (path: string) => {
      const result = await tauriApi.uploadFile({
        parentId: currentFolder.id,
        sourcePath: path,
      });

      if (result.success) {
        await fetchFiles(currentFolder.id);
      }
    },
    [currentFolder.id, fetchFiles],
  );

  const { isDragging } = useFileDragDrop({
    onDrop: async event => {
      const paths = (event as { payload: { paths: string[] } }).payload.paths;
      for (const path of paths) {
        await performUpload(path);
      }
    },
  });

  const handleFolderClick = (item: FolderItem) => {
    setCurrentPath(prev => [...prev, { id: item.id, name: item.name }]);
  };

  const handleBreadcrumbClick = (index: number) => {
    setCurrentPath(prev => prev.slice(0, index + 1));
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
    handleCreateFolder,
    handleDeleteItem,
    handleRenameItem,
    handleUploadFile,
    getFileContent,
    refresh: () => fetchFiles(currentFolder.id),
  };
};
