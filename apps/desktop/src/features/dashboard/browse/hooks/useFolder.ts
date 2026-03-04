import { tauriApi } from "@/libraries/tauri-api";
import { FolderItemResult, ItemType, type BrowseResult } from "@/types/filesystem";
import {
  ArchiveIcon,
  BookOpenIcon,
  BriefcaseIcon,
  CameraIcon,
  CodeIcon,
  FolderIcon,
  ImageIcon,
  MusicIcon,
  ShieldIcon,
  StarIcon,
  type LucideIcon,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { type FolderRenamingItem, type PathSegment } from "../types";

export const ICON_MAP = {
  folder: FolderIcon,
  star: StarIcon,
  briefcase: BriefcaseIcon,
  archive: ArchiveIcon,
  "book-open": BookOpenIcon,
  image: ImageIcon,
  music: MusicIcon,
  camera: CameraIcon,
  code: CodeIcon,
  shield: ShieldIcon,
} as const satisfies Record<string, LucideIcon>;

export type FolderIconName = keyof typeof ICON_MAP;

export const FOLDER_ICON_OPTIONS = Object.keys(ICON_MAP).map(name => ({
  name: name as FolderIconName,
  Icon: ICON_MAP[name as FolderIconName],
}));

const ROOT_FOLDER_ID = "00000000-0000-0000-0000-000000000000";
const ROOT_FOLDER: PathSegment = { id: ROOT_FOLDER_ID, name: "Home" };

const folderRequests = new Map<string, Promise<BrowseResult | null>>();
const folderCache = new Map<string, BrowseResult>();

const isSameFolder = (left: FolderItemResult, right: FolderItemResult) =>
  left.id === right.id &&
  left.name === right.name &&
  left.icon === right.icon &&
  left.isFavourite === right.isFavourite &&
  left.itemCount === right.itemCount;

const isSameFile = (
  left: BrowseResult["files"][number],
  right: BrowseResult["files"][number],
) =>
  left.id === right.id &&
  left.name === right.name &&
  left.extension === right.extension &&
  left.size === right.size &&
  left.isFavourite === right.isFavourite;

const isSameListing = (left: BrowseResult, right: BrowseResult) => {
  if (
    left.folders.length !== right.folders.length ||
    left.files.length !== right.files.length
  ) {
    return false;
  }

  const hasDifferentFolder = left.folders.some((folder, index) => {
    const rightFolder = right.folders[index];
    return rightFolder == null || !isSameFolder(folder, rightFolder);
  });
  if (hasDifferentFolder) {
    return false;
  }

  const hasDifferentFile = left.files.some((file, index) => {
    const rightFile = right.files[index];
    return rightFile == null || !isSameFile(file, rightFile);
  });

  return !hasDifferentFile;
};

const fetchFolderListing = async (
  folderId: string,
  options: { dedupeRequest: boolean },
): Promise<BrowseResult | null> => {
  if (options.dedupeRequest) {
    const activeRequest = folderRequests.get(folderId);
    if (activeRequest) {
      return activeRequest;
    }
  }

  const request = tauriApi
    .browseFs({ parentId: folderId })
    .then(result => (result.success ? result.data : null));

  if (!options.dedupeRequest) {
    return request;
  }

  folderRequests.set(folderId, request);
  return request.finally(() => {
    if (folderRequests.get(folderId) === request) {
      folderRequests.delete(folderId);
    }
  });
};

type UseFolderOptions = {
  searchQuery: string;
};

const sortFolders = (items: FolderItemResult[]) =>
  [...items].sort((left, right) => {
    if (left.isFavourite !== right.isFavourite) {
      return left.isFavourite ? -1 : 1;
    }

    return left.name.localeCompare(right.name);
  });

export const useFolder = ({ searchQuery }: UseFolderOptions) => {
  const [currentPath, setCurrentPath] = useState<PathSegment[]>([ROOT_FOLDER]);
  const [listing, setListing] = useState<BrowseResult>({ folders: [], files: [] });
  const [loadedFolderId, setLoadedFolderId] = useState<string | null>(null);
  const [renamingItem, setRenamingItem] = useState<FolderRenamingItem | null>(null);

  const canGoBack = currentPath.length > 1;

  const loadSequenceRef = useRef(0);

  const currentFolder = currentPath[currentPath.length - 1] ?? ROOT_FOLDER;
  const currentFolderId = currentFolder.id;

  const isLoading = loadedFolderId === null;
  const isNavigating = loadedFolderId !== null && loadedFolderId !== currentFolderId;

  const applyListing = (data: BrowseResult, folderId: string) => {
    folderCache.set(folderId, data);

    setListing(previousListing =>
      isSameListing(previousListing, data) ? previousListing : data,
    );

    setLoadedFolderId(previousFolderId =>
      previousFolderId === folderId ? previousFolderId : folderId,
    );
  };

  const refresh = async () => {
    const loadSequence = ++loadSequenceRef.current;
    const data = await fetchFolderListing(currentFolderId, { dedupeRequest: false });

    if (loadSequenceRef.current !== loadSequence) {
      return;
    }

    if (data) {
      applyListing(data, currentFolderId);
      return;
    }

    setLoadedFolderId(previousFolderId =>
      previousFolderId === currentFolderId ? previousFolderId : currentFolderId,
    );
  };

  const handleFolderClick = (item: FolderItemResult) => {
    setCurrentPath(previousPath => {
      const activeItem = previousPath[previousPath.length - 1] ?? ROOT_FOLDER;

      if (activeItem.id === item.id) {
        return previousPath;
      }

      return [...previousPath, { id: item.id, name: item.name }];
    });
  };

  const handleBreadcrumbClick = (index: number) => {
    setCurrentPath(previousPath => {
      const isOutOfRange = index < 0 || index >= previousPath.length;
      const isSameLevel = index === previousPath.length - 1;

      if (isOutOfRange || isSameLevel) {
        return previousPath;
      }

      return previousPath.slice(0, index + 1);
    });
  };

  const handleCreateFolder = async (name: string) => {
    const trimmedName = name.trim();
    if (!trimmedName) {
      return;
    }

    const result = await tauriApi.createFolder({
      parentId: currentFolderId,
      name: trimmedName,
    });

    if (result.success) {
      await refresh();
    }
  };

  const handleDeleteFolder = async (id: string) => {
    const result = await tauriApi.deleteItem({ id, itemType: "folder" });

    if (result.success) {
      await refresh();
    }
  };

  const handleRenameFolder = async (id: string, newName: string) => {
    const trimmedName = newName.trim();
    if (!trimmedName) {
      return;
    }

    const result = await tauriApi.renameItem({
      id,
      itemType: "folder",
      newName: trimmedName,
    });

    if (result.success) {
      await refresh();
    }
  };

  const handleRequestFolderRename = (item: FolderItemResult) => {
    setRenamingItem({ id: item.id, name: item.name, type: "folder" });
  };

  const handleChangeFolderIcon = async (id: string, iconName: string) => {
    const result = await tauriApi.setFolderIcon({ id, icon: iconName });

    if (result.success) {
      await refresh();
      return;
    }

    toast.error("Failed to change folder icon");
  };

  const clearRenamingItem = () => {
    setRenamingItem(null);
  };

  const renameRenamingItem = async (newName: string) => {
    if (!renamingItem) {
      return;
    }

    await handleRenameFolder(renamingItem.id, newName);
  };

  const handleToggleFavourite = async (folder: FolderItemResult) => {
    const result = await tauriApi.setFavourtieItem({
      id: folder.id,
      isFavourite: !folder.isFavourite,
      itemType: ItemType.FOLDER,
    });

    if (result.success) {
      await refresh();
    }
  };

  const normalizedSearch = searchQuery.trim().toLowerCase();
  const filteredFolders = normalizedSearch
    ? listing.folders.filter(item => item.name.toLowerCase().includes(normalizedSearch))
    : listing.folders;
  const folders = sortFolders(filteredFolders);

  useEffect(() => {
    let isMounted = true;
    const loadSequence = ++loadSequenceRef.current;

    const load = async () => {
      const cachedListing = folderCache.get(currentFolderId);
      if (cachedListing) {
        applyListing(cachedListing, currentFolderId);
      }

      const data = await fetchFolderListing(currentFolderId, { dedupeRequest: true });

      if (!isMounted || loadSequenceRef.current !== loadSequence) {
        return;
      }

      if (data) {
        applyListing(data, currentFolderId);
        return;
      }

      setLoadedFolderId(previousFolderId =>
        previousFolderId === currentFolderId ? previousFolderId : currentFolderId,
      );
    };

    void load();

    return () => {
      isMounted = false;
    };
  }, [currentFolderId]);

  return {
    currentPath: currentPath.map(pathSegment => pathSegment.name),
    currentFolderId,
    folders,
    files: listing.files,
    folderCount: listing.folders.length,
    fileCount: listing.files.length,
    hasAnyItems: listing.folders.length > 0 || listing.files.length > 0,
    isLoading,
    isNavigating,
    renamingItem,
    canGoBack,
    refresh,
    handleFolderClick,
    handleBreadcrumbClick,
    handleCreateFolder,
    handleDeleteFolder,
    handleRequestFolderRename,
    handleChangeFolderIcon,
    clearRenamingItem,
    renameRenamingItem,
    handleToggleFavourite,
  };
};
