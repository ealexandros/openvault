"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { Separator } from "@/components/ui/shadcn/separator";
import {
  FileIcon,
  FolderIcon,
  LayoutGridIcon,
  SearchIcon,
  UploadCloudIcon,
} from "lucide-react";
import { Breadcrumbs } from "./Breadcrumbs";
import { NewFolderDialog } from "./NewFolderDialog";

type BrowseHeaderProps = {
  currentPath: string[];
  folderCount: number;
  fileCount: number;
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  onBreadcrumbClick: (index: number) => void;
  onUploadFile: () => void;
  onCreateFolder: (name: string) => Promise<void>;
};

export const BrowseHeader = ({
  currentPath,
  folderCount,
  fileCount,
  searchQuery,
  onSearchQueryChange,
  onBreadcrumbClick,
  onUploadFile,
  onCreateFolder,
}: BrowseHeaderProps) => (
  <div className="sticky top-0 z-20 animate-in rounded-2xl border bg-background/95 backdrop-blur-sm duration-300 fade-in slide-in-from-top-2">
    <div className="flex flex-col gap-6 px-6 py-6 xl:flex-row xl:items-center xl:justify-between">
      <div className="flex items-center gap-4">
        <div className="flex size-10 items-center justify-center rounded-xl border bg-muted/40">
          <LayoutGridIcon className="size-5 text-muted-foreground" />
        </div>
        <div>
          <h1 className="text-xl font-semibold tracking-tight">Browse Files</h1>
          <p className="text-sm text-muted-foreground">Encrypted workspace</p>
        </div>
      </div>

      <div className="flex items-center gap-6 text-sm">
        <div className="flex items-center gap-2">
          <FolderIcon className="size-4 text-muted-foreground" />
          <span className="font-medium tabular-nums">{folderCount}</span>
          <span className="text-muted-foreground">Folders</span>
        </div>

        <div className="flex items-center gap-2">
          <FileIcon className="size-4 text-muted-foreground" />
          <span className="font-medium tabular-nums">{fileCount}</span>
          <span className="text-muted-foreground">Files</span>
        </div>
      </div>

      <div className="flex items-center gap-3">
        <NewFolderDialog onCreate={onCreateFolder} />
        <Button onClick={onUploadFile} className="h-10 px-5 text-sm">
          <UploadCloudIcon className="mr-2 size-4" />
          Upload
        </Button>
      </div>
    </div>

    <Separator />

    <div className="flex flex-col gap-4 px-6 py-4 xl:flex-row xl:items-center xl:justify-between">
      <Breadcrumbs currentPath={currentPath} onClick={onBreadcrumbClick} />

      <div className="relative w-full xl:w-72">
        <SearchIcon className="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={searchQuery}
          onChange={event => onSearchQueryChange(event.target.value)}
          placeholder="Search files and folders"
          className="h-10 pl-10 text-sm"
        />
      </div>
    </div>
  </div>
);
