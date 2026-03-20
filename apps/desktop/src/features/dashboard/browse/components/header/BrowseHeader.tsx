"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { Kbd } from "@/components/ui/shadcn/kbd";
import { Separator } from "@/components/ui/shadcn/separator";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { PathSegment } from "@/features/dashboard/browse/types";
import { isMacintosh } from "@/utils/navigator";
import { useHotkey } from "@tanstack/react-hotkeys";
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  FileInput,
  FolderInput,
  FolderPlus,
  SearchIcon,
} from "lucide-react";
import { useRef } from "react";
import { CreateFolderDialog } from "../dialogs";
import { BrowseBreadcrumbs } from "./Breadcrumbs";

type BrowseHeaderProps = {
  currentPath: PathSegment[];
  folderCount: number;
  fileCount: number;
  searchQuery: string;
  currentFolderId?: string;
  canGoBack: boolean;
  canGoForward: boolean;
  onFolderCreate: () => void;
  onSearchQueryChange: (value: string) => void;
  onBreadcrumbClick: (index: number) => void;
  onUploadFile: () => void;
  onUploadFolder: () => void;
  onBack: () => void;
  onForward: () => void;
};

export const BrowseHeader = ({
  currentPath,
  folderCount,
  fileCount,
  searchQuery,
  currentFolderId,
  canGoBack,
  canGoForward,
  onFolderCreate,
  onSearchQueryChange,
  onBreadcrumbClick,
  onUploadFile,
  onUploadFolder,
  onBack,
  onForward,
}: BrowseHeaderProps) => {
  const searchRef = useRef<HTMLInputElement>(null);

  useHotkey("Mod+F", () => searchRef.current?.focus());
  useHotkey("Escape", () => searchRef.current?.blur());

  const totalItems = folderCount + fileCount;

  return (
    <header className="space-y-8">
      <section className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 rounded-full py-1.5">
            <Button
              variant="ghost"
              size="icon"
              className="group p-4"
              onClick={onBack}
              disabled={!canGoBack}>
              <ChevronLeftIcon className="size-6 text-foreground group-disabled:text-muted-foreground/60" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="group p-4"
              onClick={onForward}
              disabled={!canGoForward}>
              <ChevronRightIcon className="size-6 text-foreground group-disabled:text-muted-foreground/60" />
            </Button>
          </div>
          <h1 className="text-2xl font-semibold">
            {currentPath[currentPath.length - 1]?.name}
          </h1>
          <p className="mt-1 text-sm font-medium tracking-wide text-muted-foreground/80">
            {totalItems} item{totalItems !== 1 ? "s" : ""}
          </p>
        </div>
        <div className="flex items-center gap-5">
          <Tooltip>
            <TooltipTrigger className="cursor-pointer" asChild>
              <button onClick={onUploadFile}>
                <FileInput className="size-4.5 text-muted-foreground transition-colors hover:text-primary" />
              </button>
            </TooltipTrigger>
            <TooltipContent>Upload Files</TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger className="cursor-pointer" asChild>
              <button onClick={onUploadFolder}>
                <FolderInput className="size-5 text-muted-foreground transition-colors hover:text-primary" />
              </button>
            </TooltipTrigger>
            <TooltipContent>Upload Folders</TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger className="cursor-pointer" asChild>
              <div className="mt-1.5">
                <CreateFolderDialog parentId={currentFolderId} onSuccess={onFolderCreate}>
                  <button className="cursor-pointer">
                    <FolderPlus className="size-5 text-muted-foreground transition-colors hover:text-primary" />
                  </button>
                </CreateFolderDialog>
              </div>
            </TooltipTrigger>
            <TooltipContent>Create Folder</TooltipContent>
          </Tooltip>

          <Separator orientation="vertical" className="my-2" />

          <div className="relative w-full xl:w-72">
            <SearchIcon className="absolute top-1/2 left-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              ref={searchRef}
              value={searchQuery}
              onChange={e => onSearchQueryChange(e.target.value)}
              placeholder="Search files and folders"
              className="h-9.5 bg-gray-50/30 pr-16 pl-8 text-sm"
            />
            <Kbd className="absolute top-1/2 right-3 -translate-y-1/2 text-sm">
              {isMacintosh() ? "⌘ + f" : "Ctrl + f"}
            </Kbd>
          </div>
        </div>
      </section>

      <nav className="mx-2">
        <BrowseBreadcrumbs pathSegments={currentPath} onPathClick={onBreadcrumbClick} />
      </nav>
    </header>
  );
};
