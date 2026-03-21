export const ItemType = {
  FILE: "file",
  FOLDER: "folder",
} as const;

export type ItemType = (typeof ItemType)[keyof typeof ItemType];

export type FolderItemResult = {
  id: string;
  name: string;
  icon: string;
  itemCount: number;
  totalSizeBytes: number;
  isFavourite: boolean;
  createdAt: string;
  updatedAt: string;
};

export type FileItemResult = {
  id: string;
  name: string;
  size: number;
  extension: string;
  isFavourite: boolean;
  createdAt: string;
  updatedAt: string;
};

export type BrowseResult = {
  folders: FolderItemResult[];
  files: FileItemResult[];
};

export type VaultMetaResult = {
  name: string;
  path: string;
  sizeInBytes: number;
  version: number;
};
