export type FolderItem = {
  id: string;
  name: string;
  itemCount: number;
};

export type FileItem = {
  id: string;
  name: string;
  size: number;
  extension: string;
};

export type BrowseResult = {
  folders: FolderItem[];
  files: FileItem[];
};
