import { type FileItemResult, type FolderItemResult } from "@/types/filesystem";
import { useState } from "react";
import { BrowseViewState, type RenamingItem } from "../types";
import { useFile } from "./useFile";
import { useFolder } from "./useFolder";

const resolveBrowseViewState = (options: {
  isLoading: boolean;
  hasAnyItems: boolean;
  hasSearchResults: boolean;
}): BrowseViewState => {
  if (options.isLoading) {
    return BrowseViewState.Loading;
  }

  if (!options.hasAnyItems) {
    return BrowseViewState.Empty;
  }

  if (!options.hasSearchResults) {
    return BrowseViewState.NoResults;
  }

  return BrowseViewState.Results;
};

type PendingDeletionItem = {
  id: string;
  name: string;
  type: "file" | "folder";
};

export const useBrowse = () => {
  const [searchQuery, setSearchQuery] = useState("");

  const [selectedFileForProperties, setSelectedFileForProperties] =
    useState<FileItemResult | null>(null);
  const [selectedFolderForProperties, setSelectedFolderForProperties] =
    useState<FolderItemResult | null>(null);
  const [selectedFolderForIconChange, setSelectedFolderForIconChange] =
    useState<FolderItemResult | null>(null);
  const [pendingDeletionItem, setPendingDeletionItem] = useState<PendingDeletionItem | null>(
    null,
  );

  const folderStore = useFolder({ searchQuery });

  const fileStore = useFile({
    currentFolderId: folderStore.currentFolderId,
    files: folderStore.files,
    searchQuery,
    refresh: folderStore.refresh,
  });

  const hasSearchResults = folderStore.folders.length > 0 || fileStore.files.length > 0;

  const viewState = resolveBrowseViewState({
    isLoading: folderStore.isLoading,
    hasAnyItems: folderStore.hasAnyItems,
    hasSearchResults,
  });

  const renamingItem: RenamingItem | null = folderStore.renamingItem ?? fileStore.renamingItem;

  const clearSearch = () => {
    setSearchQuery("");
  };

  const openFolder = (folder: FolderItemResult) => {
    folderStore.handleFolderClick(folder);
    clearSearch();
  };

  const goBack = () => {
    folderStore.handleBreadcrumbClick(folderStore.currentPath.length - 2);
    clearSearch();
  };

  const openBreadcrumb = (index: number) => {
    folderStore.handleBreadcrumbClick(index);
    clearSearch();
  };

  const isRenameVisible = renamingItem !== null;

  const toggleRenameVisibility = (isVisible: boolean) => {
    if (isVisible) {
      return;
    }

    folderStore.clearRenamingItem();
    fileStore.clearRenamingItem();
  };

  const submitRename = async (newName: string) => {
    if (folderStore.renamingItem) {
      await folderStore.renameRenamingItem(newName);
      return;
    }

    if (fileStore.renamingItem) {
      await fileStore.renameRenamingItem(newName);
    }
  };

  const viewingItem = fileStore.viewingItem;
  const isFileViewerVisible = viewingItem !== null;

  const toggleFileViewerVisibility = (isVisible: boolean) => {
    fileStore.handleFileViewerOpenChange(isVisible);
  };

  const isFolderIconPickerVisible = selectedFolderForIconChange !== null;

  const requestFolderIconChange = (folder: FolderItemResult) => {
    setSelectedFolderForIconChange(folder);
  };

  const toggleFolderIconPickerVisibility = (isVisible: boolean) => {
    if (!isVisible) {
      setSelectedFolderForIconChange(null);
    }
  };

  const selectFolderIcon = async (iconName: string) => {
    if (selectedFolderForIconChange == null) {
      return;
    }

    await folderStore.handleChangeFolderIcon(selectedFolderForIconChange.id, iconName);
    setSelectedFolderForIconChange(null);
  };

  const isFilePropertiesVisible = selectedFileForProperties !== null;

  const toggleFilePropertiesVisibility = (isVisible: boolean) => {
    if (!isVisible) {
      setSelectedFileForProperties(null);
    }
  };

  const showFileProperties = (file: FileItemResult) => {
    setSelectedFileForProperties(file);
  };

  const isFolderPropertiesVisible = selectedFolderForProperties !== null;

  const toggleFolderPropertiesVisibility = (isVisible: boolean) => {
    if (!isVisible) {
      setSelectedFolderForProperties(null);
    }
  };

  const showFolderProperties = (folder: FolderItemResult) => {
    setSelectedFolderForProperties(folder);
  };

  const isDeleteConfirmationVisible = pendingDeletionItem !== null;

  const toggleDeleteConfirmationVisibility = (isVisible: boolean) => {
    if (!isVisible) {
      setPendingDeletionItem(null);
    }
  };

  const requestFileDeletion = (file: FileItemResult) => {
    setPendingDeletionItem({ id: file.id, name: file.name, type: "file" });
  };

  const requestFolderDeletion = (folder: FolderItemResult) => {
    setPendingDeletionItem({ id: folder.id, name: folder.name, type: "folder" });
  };

  const confirmDeleteSelection = async () => {
    if (pendingDeletionItem == null) {
      return;
    }

    if (pendingDeletionItem.type === "folder") {
      await folderStore.handleDeleteFolder(pendingDeletionItem.id);
      return;
    }

    await fileStore.handleDeleteFile(pendingDeletionItem.id);
  };

  const browseState = {
    currentPath: folderStore.currentPath,
    folderCount: folderStore.folderCount,
    fileCount: folderStore.fileCount,
    searchQuery,
    setSearchQuery,
    clearSearch,
    viewState,
    isNavigating: folderStore.isNavigating,
    canGoBack: folderStore.canGoBack,
  };

  const folderState = {
    folders: folderStore.folders,
    openFolder,
    goBack,
    openBreadcrumb,
    createFolder: folderStore.handleCreateFolder,
    requestRename: folderStore.handleRequestFolderRename,
    requestDelete: requestFolderDeletion,
    requestProperties: showFolderProperties,
    requestIconChange: requestFolderIconChange,
    toggleFavourite: folderStore.handleToggleFavourite,
  };

  const fileState = {
    files: fileStore.files,
    handleDropPaths: fileStore.uploadPaths,
    openFile: fileStore.handleFileClick,
    uploadFile: fileStore.handleUploadFile,
    uploadFolder: fileStore.handleUploadFolder,
    requestRename: fileStore.handleRequestFileRename,
    requestDelete: requestFileDeletion,
    requestProperties: showFileProperties,
    toggleFavourite: fileStore.handleToggleFavourite,
  };

  const dialogState = {
    isRenameVisible,
    isFileViewerVisible,
    isFolderIconPickerVisible,
    isFilePropertiesVisible,
    isFolderPropertiesVisible,
    isDeleteConfirmationVisible,
    renameInitialName: renamingItem?.name ?? "",
    renameItemType: renamingItem?.type ?? "file",
    viewingItem,
    fileForProperties: selectedFileForProperties,
    folderForProperties: selectedFolderForProperties,
    deleteItemName: pendingDeletionItem?.name ?? "",
    deleteItemType: pendingDeletionItem?.type ?? "file",
    toggleFolderPropertiesVisibility,
    toggleFilePropertiesVisibility,
    toggleFolderIconPickerVisibility,
    toggleDeleteConfirmationVisibility,
    toggleFileViewerVisibility,
    toggleRenameVisibility,
    selectFolderIcon,
    submitRename,
    confirmDeleteSelection,
  };

  return {
    browseState,
    folderState,
    fileState,
    dialogState,
  };
};
