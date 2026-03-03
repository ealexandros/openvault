"use client";

import { Input } from "@/components/ui/shadcn/input";
import { SearchIcon } from "lucide-react";
import { BrowseBreadcrumbs } from "./Breadcrumbs";
import { FileStats } from "./FileStats";

type BrowseHeaderNavigationProps = {
  currentPath: string[];
  folderCount: number;
  fileCount: number;
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  onBreadcrumbClick: (index: number) => void;
};

export const BrowseHeaderNavigation = ({
  currentPath,
  folderCount,
  fileCount,
  searchQuery,
  onSearchQueryChange,
  onBreadcrumbClick,
}: BrowseHeaderNavigationProps) => (
  <div className="flex flex-col gap-5 xl:flex-row xl:items-center xl:justify-between">
    <BrowseBreadcrumbs currentPath={currentPath} onPathClick={onBreadcrumbClick} />

    <div className="flex flex-col gap-4 xl:flex-row xl:items-center xl:gap-6">
      <FileStats folderCount={folderCount} fileCount={fileCount} />

      <div className="relative w-full xl:w-72">
        <SearchIcon className="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={searchQuery}
          onChange={e => onSearchQueryChange(e.target.value)}
          placeholder="Search files and folders"
          className="h-10 pl-10 text-sm"
        />
      </div>
    </div>
  </div>
);
