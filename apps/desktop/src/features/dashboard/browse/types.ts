import { ItemType } from "@/types/filesystem";

export type PathSegment = {
  id?: string;
  name: string;
};

export type RenameTarget = {
  id: string;
  name: string;
  type: ItemType;
};

export type DeletionTarget = {
  id: string;
  name: string;
  type: ItemType;
};

export type ExportTarget = {
  id: string;
  name: string;
  type: ItemType;
};

export type PreviewItem = {
  id: string;
  name: string;
  extension?: string;
  content: number[] | null;
};

export enum BrowseViewState {
  Loading = "loading",
  Empty = "empty",
  NoResults = "no-results",
  Results = "results",
}
