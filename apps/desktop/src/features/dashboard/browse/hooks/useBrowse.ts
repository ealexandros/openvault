"use client";

import { tauriApi } from "@/libraries/tauri-api";
import { ItemType, type FileItemResult, type FolderItemResult } from "@/types/filesystem";
import { useQuery } from "@tanstack/react-query";
import { Home } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";

import { getFolderIcon } from "../data/folder-icons";
import { BrowseViewState, PathSegment } from "../types";
import { useDialogs } from "./useDialogs";
import { useFileUploader } from "./useFileUploader";
import { useListings } from "./useListings";

const ROOT_FOLDER: PathSegment = {
  id: undefined,
  name: "Home",
  icon: Home,
};

type BrowseHistoryState = {
  history: PathSegment[][];
  index: number;
};

export const getBrowseKey = (folderId?: string) => ["browse", folderId];

export const useBrowse = () => {
  const [historyState, setHistoryState] = useState<BrowseHistoryState>({
    history: [[ROOT_FOLDER]],
    index: 0,
  });

  const currentPath = historyState.history[historyState.index] ?? [ROOT_FOLDER];
  const currentFolderId = currentPath[currentPath.length - 1]?.id;

  const browseQuery = useQuery({
    queryKey: getBrowseKey(currentFolderId),
    queryFn: async () => {
      const result = await tauriApi.browseFs({ parentId: currentFolderId });
      return result.success ? result.data : null;
    },
  });

  const refresh = async () => {
    await browseQuery.refetch();
  };

  const dialogs = useDialogs();
  const fileUploader = useFileUploader({ folderId: currentFolderId, refresh });

  const listings = useListings({
    initialFiles: browseQuery.data?.files ?? [],
    initialFolders: browseQuery.data?.folders ?? [],
  });

  const toggleFileFavourite = async (file: FileItemResult) => {
    const result = await tauriApi.setFavouriteItem({
      id: file.id,
      isFavourite: !file.isFavourite,
      itemType: ItemType.FILE,
    });

    if (!result.success) {
      toast.error("Failed to make the file favourite");
      return;
    }
    await refresh();
  };

  const toggleFolderFavourite = async (folder: FolderItemResult) => {
    const result = await tauriApi.setFavouriteItem({
      id: folder.id,
      isFavourite: !folder.isFavourite,
      itemType: ItemType.FOLDER,
    });

    if (!result.success) {
      toast.error("Failed to make the folder favourite");
      return;
    }
    await refresh();
  };

  const navigateToPath = (newPath: PathSegment[]) => {
    setHistoryState(prev => {
      const current = prev.history[prev.index];

      if (JSON.stringify(current) === JSON.stringify(newPath)) return prev;
      const nextHistory = prev.history.slice(0, prev.index + 1);

      return { history: [...nextHistory, newPath], index: prev.index + 1 };
    });

    listings.clearSearch();
  };

  const navigateToFolder = (folder: FolderItemResult) => {
    if (folder.id === currentFolderId) return;

    navigateToPath([
      ...currentPath,
      { id: folder.id, name: folder.name, icon: getFolderIcon(folder.icon) },
    ]);
  };

  const goBack = () => {
    setHistoryState(prev => {
      if (prev.index === 0) return prev;
      return { ...prev, index: prev.index - 1 };
    });
    listings.clearSearch();
  };

  const goForward = () => {
    setHistoryState(prev => {
      if (prev.index >= prev.history.length - 1) return prev;
      return { ...prev, index: prev.index + 1 };
    });
    listings.clearSearch();
  };

  const navigateToBreadcrumb = (index: number) => {
    if (index < 0 || index >= currentPath.length) return;
    for (let i = index + 1; i < currentPath.length; i++) {
      goBack();
    }
  };

  const resolveBrowseViewState = (): BrowseViewState => {
    if (browseQuery.isLoading) return BrowseViewState.Loading;
    if (!listings.hasAnyItems) return BrowseViewState.Empty;
    if (listings.isSearching && !listings.hasSearchResults) return BrowseViewState.NoResults;
    return BrowseViewState.Results;
  };

  const viewState = resolveBrowseViewState();

  return {
    dialogs,
    browseState: {
      viewState,
      currentPath,
      currentFolderId,
      folders: listings.folders,
      files: listings.files,
      folderCount: listings.folderCount,
      fileCount: listings.fileCount,
      searchQuery: listings.searchQuery,
      isNavigating: browseQuery.isLoading,
      canGoBack: historyState.index > 0,
      canGoForward: historyState.index < historyState.history.length - 1,
      setSearchQuery: listings.setSearchQuery,
      clearSearch: listings.clearSearch,
      goBack,
      goForward,
      navigateToFolder,
      navigateToBreadcrumb,
    },
    upload: {
      files: fileUploader.uploadFile,
      folders: fileUploader.uploadFolder,
      paths: fileUploader.uploadPaths,
    },
    toggleFileFavourite,
    toggleFolderFavourite,
    refresh,
  };
};
