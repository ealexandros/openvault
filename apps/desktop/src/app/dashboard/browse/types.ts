import { LucideIcon } from "lucide-react";

export type PathSegment = {
  id: string;
  name: string;
};

export type FolderRenamingItem = {
  id: string;
  name: string;
  type: "folder";
};

export type FileRenamingItem = {
  id: string;
  name: string;
  type: "file";
};

export type RenamingItem = FolderRenamingItem | FileRenamingItem;

export type ViewingItem = {
  id: string;
  name: string;
  extension?: string;
  content: number[] | null;
};

export type FolderIconOption = {
  name: string;
  Icon: LucideIcon;
};

export enum BrowseViewState {
  Loading = "loading",
  Empty = "empty",
  NoResults = "no-results",
  Results = "results",
}
