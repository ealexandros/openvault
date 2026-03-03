export type FolderItemResult = {
  id: string;
  name: string;
  icon: string;
  itemCount: number;
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
