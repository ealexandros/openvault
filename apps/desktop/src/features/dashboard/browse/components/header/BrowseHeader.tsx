"use client";

import { PathSegment } from "@/features/dashboard/browse/types";
import { BrowseHeaderNavigation } from "./BrowseHeaderNavigation";
import { BrowseHeaderToolbar } from "./BrowseHeaderToolbar";

type BrowseHeaderProps = {
  currentPath: PathSegment[];
  folderCount: number;
  fileCount: number;
  searchQuery: string;
  currentFolderId: string;
  onFolderCreate: () => void;
  onSearchQueryChange: (value: string) => void;
  onBreadcrumbClick: (index: number) => void;
  onUploadFile: () => void;
  onUploadFolder: () => void;
};

export const BrowseHeader = (props: BrowseHeaderProps) => (
  <div className="space-y-6">
    <BrowseHeaderToolbar
      currentFolderId={props.currentFolderId}
      onFolderCreate={props.onFolderCreate}
      onUploadFile={props.onUploadFile}
      onUploadFolder={props.onUploadFolder}
    />
    <BrowseHeaderNavigation
      currentPath={props.currentPath}
      folderCount={props.folderCount}
      fileCount={props.fileCount}
      searchQuery={props.searchQuery}
      onSearchQueryChange={props.onSearchQueryChange}
      onBreadcrumbClick={props.onBreadcrumbClick}
    />
  </div>
);
