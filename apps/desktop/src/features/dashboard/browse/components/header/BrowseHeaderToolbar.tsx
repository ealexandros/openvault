"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { FileUp, FolderUp, LayoutGridIcon, UploadIcon } from "lucide-react";
import { CreateFolderDialog } from "../dialogs";

type BrowseHeaderToolbarProps = {
  currentFolderId?: string;
  onFolderCreate: () => void;
  onUploadFile: () => void;
  onUploadFolder: () => void;
};

export const BrowseHeaderToolbar = ({
  currentFolderId,
  onFolderCreate,
  onUploadFile,
  onUploadFolder,
}: BrowseHeaderToolbarProps) => (
  <div className="flex flex-row items-center justify-between gap-6">
    <div className="flex items-center gap-4">
      <div className="rounded-xl bg-primary/10 p-4">
        <LayoutGridIcon className="size-6 text-primary" />
      </div>
      <div className="space-y-1">
        <h1 className="text-2xl font-semibold tracking-tight">Browse Files</h1>
        <p className="text-sm text-muted-foreground">Encrypted workspace</p>
      </div>
    </div>

    <div className="flex items-center gap-2 xl:gap-3">
      <CreateFolderDialog parentId={currentFolderId} onSuccess={onFolderCreate} />

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button className="h-10 gap-2 px-5 text-sm font-medium">
            <UploadIcon className="size-4" />
            Upload
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent align="end" className="w-48 p-2">
          <DropdownMenuItem onClick={onUploadFile} className="cursor-pointer text-[13px]">
            <FolderUp className="size-4 text-foreground/80" />
            Upload Files
          </DropdownMenuItem>
          <DropdownMenuItem onClick={onUploadFolder} className="cursor-pointer text-[13px]">
            <FileUp className="size-4 text-foreground/80" />
            Upload Folders
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </div>
);
