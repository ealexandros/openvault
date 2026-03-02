import { useFileDragDrop } from "@/hooks/useFileDragDrop";
import { tauriApi } from "@/libraries/tauri-api";
import { BrowseResult, type FileItem, type FolderItem } from "@/types/filesystem";
import { open } from "@tauri-apps/plugin-dialog";
import { useEffect, useRef, useState } from "react";

type PathSegment = {
  id: string;
  name: string;
};

type RenamingItem = {
  id: string;
  name: string;
  type: "file" | "folder";
};

type ViewingItem = {
  id: string;
  name: string;
  extension?: string;
  content: number[] | null;
};

const ROOT_FOLDER_ID = "00000000-0000-0000-0000-000000000000";
const ROOT_FOLDER: PathSegment = { id: ROOT_FOLDER_ID, name: "Home" };

const folderRequests = new Map<string, Promise<BrowseResult | null>>();

const fetchFolderListing = async (folderId: string, options: { dedupeRequest: boolean }) => {
  if (options.dedupeRequest) {
    const activeRequest = folderRequests.get(folderId);
    if (activeRequest) return activeRequest;
  }

  const request = tauriApi
    .browseVault({ parentId: folderId })
    .then(result => (result.success ? result.data : null));

  if (!options.dedupeRequest) return request;

  folderRequests.set(folderId, request);

  return request.finally(() => {
    if (folderRequests.get(folderId) === request) folderRequests.delete(folderId);
  });
};

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([ROOT_FOLDER]);
  const [listing, setListing] = useState<BrowseResult>({ folders: [], files: [] });
  const [loadedFolderId, setLoadedFolderId] = useState<string | null>(null);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [renamingItem, setRenamingItem] = useState<RenamingItem | null>(null);
  const [viewingItem, setViewingItem] = useState<ViewingItem | null>(null);

  const loadSequenceRef = useRef(0);
  const previewSequenceRef = useRef(0);

  const currentFolder = currentPath[currentPath.length - 1] ?? ROOT_FOLDER;
  const isLoading = loadedFolderId !== currentFolder.id || isRefreshing;

  const requestCurrentFolder = async (options: { dedupeRequest: boolean }) => {
    return fetchFolderListing(currentFolder.id, options);
  };

  const refresh = async () => {
    setIsRefreshing(true);
    const loadSequence = ++loadSequenceRef.current;
    const data = await requestCurrentFolder({ dedupeRequest: false });
    if (loadSequenceRef.current === loadSequence) {
      if (data) setListing(data);
      setLoadedFolderId(currentFolder.id);
    }
    setIsRefreshing(false);
  };

  const uploadFiles = async (paths: string[]) => {
    if (paths.length === 0) return;

    const results = await Promise.all(
      paths.map(path => tauriApi.uploadFile({ parentId: currentFolder.id, sourcePath: path })),
    );

    if (results.some(result => result.success)) {
      await refresh();
    }
  };

  const handleDrop = async (event: unknown) => {
    const paths = (event as { payload?: { paths?: string[] } }).payload?.paths ?? [];
    await uploadFiles(paths);
  };

  const { isDragging } = useFileDragDrop({ onDrop: handleDrop });

  const handleFolderClick = (item: FolderItem) => {
    setCurrentPath(prevPath => {
      const currentItem = prevPath[prevPath.length - 1] ?? ROOT_FOLDER;
      if (currentItem.id === item.id) return prevPath;
      return [...prevPath, { id: item.id, name: item.name }];
    });
    setSearchQuery("");
  };

  const handleBreadcrumbClick = (index: number) => {
    setCurrentPath(prevPath => {
      if (index < 0 || index >= prevPath.length || index === prevPath.length - 1)
        return prevPath;
      return prevPath.slice(0, index + 1);
    });
    setSearchQuery("");
  };

  const handleCreateFolder = async (name: string) => {
    const trimmedName = name.trim();
    if (!trimmedName) return;

    const result = await tauriApi.createFolder({
      parentId: currentFolder.id,
      name: trimmedName,
    });
    if (result.success) await refresh();
  };

  const handleDeleteItem = async (id: string, itemType: "file" | "folder") => {
    const result = await tauriApi.deleteItem({ id, itemType });
    if (result.success) await refresh();
  };

  const handleRenameItem = async (
    id: string,
    itemType: "file" | "folder",
    newName: string,
  ) => {
    const trimmedName = newName.trim();
    if (!trimmedName) return;

    const result = await tauriApi.renameItem({ id, itemType, newName: trimmedName });
    if (result.success) await refresh();
  };

  const handleUploadFile = async () => {
    const selected = await open({ multiple: false, directory: false });
    if (typeof selected === "string") await uploadFiles([selected]);
  };

  const getFileContent = async (id: string) => {
    const result = await tauriApi.getFileContent({ id });
    if (!result.success) return null;
    return result.data;
  };

  const handleFileClick = async (item: Pick<FileItem, "id" | "name" | "extension">) => {
    const previewSequence = ++previewSequenceRef.current;

    setViewingItem({ id: item.id, name: item.name, extension: item.extension, content: null });

    const content = await getFileContent(item.id);

    if (previewSequenceRef.current !== previewSequence) return;

    setViewingItem(prevItem =>
      prevItem?.id === item.id ? { ...prevItem, content } : prevItem,
    );
  };

  const handleFileViewerOpenChange = (open: boolean) => {
    if (!open) return;
    previewSequenceRef.current += 1;
    setViewingItem(null);
  };

  const handleRequestFolderRename = (item: FolderItem) => {
    setRenamingItem({ id: item.id, name: item.name, type: "folder" });
  };

  const handleRequestFileRename = (item: FileItem) => {
    setRenamingItem({ id: item.id, name: item.name, type: "file" });
  };

  const handleRenameDialogOpenChange = (open: boolean) => {
    if (!open) setRenamingItem(null);
  };

  const handleRenameFromDialog = async (newName: string) => {
    if (!renamingItem) return;
    await handleRenameItem(renamingItem.id, renamingItem.type, newName);
  };

  const normalizedSearch = searchQuery.trim().toLowerCase();

  const filteredFolders = normalizedSearch
    ? listing.folders.filter(item => item.name.toLowerCase().includes(normalizedSearch))
    : listing.folders;

  const filteredFiles = normalizedSearch
    ? listing.files.filter(item =>
        `${item.name}.${item.extension}`.toLowerCase().includes(normalizedSearch),
      )
    : listing.files;

  const hasAnyItems = listing.folders.length > 0 || listing.files.length > 0;
  const hasSearchResults = filteredFolders.length > 0 || filteredFiles.length > 0;

  const viewState: "loading" | "empty" | "no-results" | "results" = isLoading
    ? "loading"
    : !hasAnyItems
      ? "empty"
      : !hasSearchResults
        ? "no-results"
        : "results";

  const currentPathLabels = currentPath.map(pathSegment => pathSegment.name);

  const clearSearch = () => setSearchQuery("");

  const handleDeleteFolder = async (id: string) => {
    await handleDeleteItem(id, "folder");
  };

  const handleDeleteFile = async (id: string) => {
    await handleDeleteItem(id, "file");
  };

  useEffect(() => {
    let isMounted = true;
    const loadSequence = ++loadSequenceRef.current;

    const load = async () => {
      const data = await requestCurrentFolder({ dedupeRequest: true });
      if (!isMounted || loadSequenceRef.current !== loadSequence) return;
      if (data) setListing(data);
      setLoadedFolderId(currentFolder.id);
    };

    void load();

    return () => {
      isMounted = false;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentFolder.id]);

  return {
    currentPath: currentPathLabels,
    folders: filteredFolders,
    files: filteredFiles,
    folderCount: listing.folders.length,
    fileCount: listing.files.length,
    searchQuery,
    viewState,
    isDragging,
    renamingItem,
    viewingItem,
    setSearchQuery,
    clearSearch,
    handleFolderClick,
    handleBreadcrumbClick,
    handleCreateFolder,
    handleUploadFile,
    handleDeleteFolder,
    handleDeleteFile,
    handleRequestFolderRename,
    handleRequestFileRename,
    handleRenameDialogOpenChange,
    handleRenameFromDialog,
    handleFileClick,
    handleFileViewerOpenChange,
  };
};
