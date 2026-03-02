import { type FolderItem } from "@/types/filesystem";
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

export { BrowseViewState };

export const useBrowse = () => {
  const [searchQuery, setSearchQuery] = useState("");

  const folderState = useFolder({ searchQuery });
  const fileState = useFile({
    currentFolderId: folderState.currentFolderId,
    files: folderState.files,
    searchQuery,
    refresh: folderState.refresh,
  });

  const hasSearchResults = folderState.folders.length > 0 || fileState.files.length > 0;

  const viewState = resolveBrowseViewState({
    isLoading: folderState.isLoading,
    hasAnyItems: folderState.hasAnyItems,
    hasSearchResults,
  });

  const renamingItem: RenamingItem | null = folderState.renamingItem ?? fileState.renamingItem;

  const clearSearch = () => {
    setSearchQuery("");
  };

  const handleFolderClick = (item: FolderItem) => {
    folderState.handleFolderClick(item);
    clearSearch();
  };

  const handleBreadcrumbClick = (index: number) => {
    folderState.handleBreadcrumbClick(index);
    clearSearch();
  };

  const handleRenameDialogOpenChange = (open: boolean) => {
    if (open) {
      return;
    }

    folderState.clearRenamingItem();
    fileState.clearRenamingItem();
  };

  const handleRenameFromDialog = async (newName: string) => {
    if (folderState.renamingItem) {
      await folderState.renameRenamingItem(newName);
      return;
    }

    if (fileState.renamingItem) {
      await fileState.renameRenamingItem(newName);
    }
  };

  return {
    currentPath: folderState.currentPath,
    folders: folderState.folders,
    files: fileState.files,
    folderCount: folderState.folderCount,
    fileCount: folderState.fileCount,
    searchQuery,
    viewState,
    isNavigating: folderState.isNavigating,
    renamingItem,
    viewingItem: fileState.viewingItem,
    canGoBack: folderState.canGoBack,
    folderIdForIconChange: folderState.folderIdForIconChange,
    setSearchQuery,
    clearSearch,
    handleDropPaths: fileState.uploadPaths,
    handleFolderClick,
    handleBreadcrumbClick,
    handleCreateFolder: folderState.handleCreateFolder,
    handleUploadFile: fileState.handleUploadFile,
    handleDeleteFolder: folderState.handleDeleteFolder,
    handleDeleteFile: fileState.handleDeleteFile,
    handleRequestFolderRename: folderState.handleRequestFolderRename,
    handleChangeFolderIcon: folderState.handleChangeFolderIcon,
    handleRequestFileRename: fileState.handleRequestFileRename,
    handleRenameDialogOpenChange,
    handleRenameFromDialog,
    handleFileClick: fileState.handleFileClick,
    handleFileViewerOpenChange: fileState.handleFileViewerOpenChange,
    handleIconDialogOpenChange: folderState.handleIconDialogOpenChange,
    handleIconSelect: folderState.handleIconSelect,
    setFolderIdForIconChange: folderState.setFolderIdForIconChange,
  };
};
