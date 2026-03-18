import { FileItemResult, FolderItemResult } from "@/types/filesystem";
import { useState } from "react";

type UseListingsOptions = {
  initialFolders: FolderItemResult[];
  initialFiles: FileItemResult[];
};

const sortItems = <T extends { name: string; isFavourite: boolean }>(items: T[]) =>
  [...items].sort((a, b) =>
    a.isFavourite !== b.isFavourite ? (a.isFavourite ? -1 : 1) : a.name.localeCompare(b.name),
  );

export const useListings = ({ initialFolders, initialFiles }: UseListingsOptions) => {
  const [searchQuery, setSearchQuery] = useState("");

  const normalizedSearchQuery = searchQuery.trim().toLowerCase();
  const isSearching = normalizedSearchQuery.length > 0;

  const folders = sortItems(
    isSearching
      ? initialFolders.filter(folder =>
          folder.name.toLowerCase().includes(normalizedSearchQuery),
        )
      : initialFolders,
  );

  const files = sortItems(
    isSearching
      ? initialFiles.filter(file => {
          const fullName =
            `${file.name}${file.extension ? `.${file.extension}` : ""}`.toLowerCase();
          return fullName.includes(normalizedSearchQuery);
        })
      : initialFiles,
  );

  const hasSearchResults = isSearching && (folders.length > 0 || files.length > 0);
  const hasNoResults = isSearching && folders.length === 0 && files.length === 0;

  return {
    folders,
    files,
    folderCount: initialFolders.length,
    fileCount: initialFiles.length,
    hasAnyItems: initialFolders.length > 0 || initialFiles.length > 0,
    searchQuery,
    isSearching,
    hasSearchResults,
    hasNoResults,
    setSearchQuery,
    clearSearch: () => setSearchQuery(""),
  };
};
