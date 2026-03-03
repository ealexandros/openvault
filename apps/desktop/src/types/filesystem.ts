export type FolderItem = {
  id: string;
  name: string;
  icon: string;
  itemCount: number;
  isFavourite: boolean;
  createdAt: string;
  updatedAt: string;
};

export type FileItem = {
  id: string;
  name: string;
  size: number;
  extension: string;
  isFavourite: boolean;
  createdAt: string;
  updatedAt: string;
};

export type BrowseResult = {
  folders: FolderItem[];
  files: FileItem[];
};
