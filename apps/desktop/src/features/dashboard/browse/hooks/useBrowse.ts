import { tauriApi } from "@/libraries/tauri-api";
import { ItemType, type FileItemResult, type FolderItemResult } from "@/types/filesystem";
import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { toast } from "sonner";
import { BrowseViewState, PathSegment } from "../types";
import { useDialogs } from "./useDialogs";
import { useFileUploader } from "./useFileUploader";
import { useListings } from "./useListings";

const getBrowseKey = (folderId?: string) => ["browse-key", folderId];

const ROOT_FOLDER: PathSegment = { id: undefined, name: "Home" };

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([ROOT_FOLDER]);

  const currentFolder = currentPath[currentPath.length - 1];
  const currentFolderId = currentFolder?.id;

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

  const navigateToFolder = ({ id, name }: FolderItemResult) => {
    setCurrentPath(prev => {
      if (prev[prev.length - 1]?.id === id) return prev;
      return [...prev, { id, name }];
    });
    listings.clearSearch();
  };

  const navigateToIndex = (index: number) => {
    setCurrentPath(path => {
      if (index < 0 || index >= path.length - 1) return path;
      return path.slice(0, index + 1);
    });
    listings.clearSearch();
  };

  const goBack = () => {
    return navigateToIndex(currentPath.length - 2);
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
      canGoBack: currentPath.length > 1,
      setSearchQuery: listings.setSearchQuery,
      clearSearch: listings.clearSearch,
      goBack,
      navigateToFolder,
      navigateToIndex,
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
