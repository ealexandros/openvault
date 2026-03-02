"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { Input } from "@/components/ui/shadcn/input";
import {
  FileIcon,
  FilePlus,
  FolderIcon,
  FolderPlus,
  LayoutGridIcon,
  SearchIcon,
  UploadCloudIcon,
} from "lucide-react";
import { NewFolderDialog } from "./NewFolderDialog";
import { PathBreadcrumbs } from "./PathBreadcrumbs";

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

export const BrowseHeader = ({
  currentPath,
  folderCount,
  fileCount,
  searchQuery,
  onSearchQueryChange,
  onBreadcrumbClick,
  onUploadFile,
  onUploadFolder,
  onCreateFolder,
}: BrowseHeaderProps) => (
  <div className="sticky top-0 z-20 space-y-6 bg-background/80 backdrop-blur-md">
    <div className="flex flex-col gap-6 xl:flex-row xl:items-center xl:justify-between">
      <div className="flex items-center gap-4">
        <div className="rounded-xl bg-primary/10 p-4">
          <LayoutGridIcon className="size-6 text-primary" />
        </div>
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">Browse Files</h1>
          <p className="mt-1 text-sm text-muted-foreground">Encrypted workspace</p>
        </div>
      </div>

      <div className="flex items-center gap-3">
        <NewFolderDialog onCreate={onCreateFolder} />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button className="h-10 gap-2 px-5 text-sm font-medium">
              <UploadCloudIcon className="size-4" />
              Upload
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="w-48 p-2">
            <DropdownMenuItem
              onClick={onUploadFile}
              className="cursor-pointer gap-2 rounded-sm py-2">
              <FilePlus className="size-4" />
              <span>Upload Files</span>
            </DropdownMenuItem>
            <DropdownMenuItem
              onClick={onUploadFolder}
              className="cursor-pointer gap-2 rounded-sm py-2">
              <FolderPlus className="size-4" />
              <span>Upload Folder</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>

    <div className="flex flex-col gap-5 xl:flex-row xl:items-center xl:justify-between">
      <PathBreadcrumbs currentPath={currentPath} onClick={onBreadcrumbClick} />

      <div className="flex flex-col gap-4 xl:flex-row xl:items-center xl:gap-6">
        <div className="flex items-center gap-6 text-sm text-muted-foreground">
          <div className="flex items-center gap-1.5">
            <FolderIcon className="size-4 opacity-70" />
            <span className="tabular-nums">{folderCount}</span>
            <span>Folders</span>
          </div>

          <div className="flex items-center gap-1.5">
            <FileIcon className="size-4 opacity-70" />
            <span className="tabular-nums">{fileCount}</span>
            <span>Files</span>
          </div>
        </div>

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
  </div>
);
