"use client";

import { BrowseHeaderNavigation } from "./BrowseHeaderNavigation";
import { BrowseHeaderToolbar } from "./BrowseHeaderToolbar";

type BrowseHeaderProps = {
  currentPath: string[];
  folderCount: number;
  fileCount: number;
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  onBreadcrumbClick: (index: number) => void;
  onUploadFile: () => void;
  onUploadFolder: () => void;
  onCreateFolder: (name: string) => Promise<void>;
};

export const BrowseHeader = (props: BrowseHeaderProps) => (
  <div className="sticky top-0 z-20 space-y-6 bg-background/80 backdrop-blur-md">
    <BrowseHeaderToolbar
      onUploadFile={props.onUploadFile}
      onUploadFolder={props.onUploadFolder}
      onCreateFolder={props.onCreateFolder}
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
