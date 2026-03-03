"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { FilePlus, FolderPlus, LayoutGridIcon, UploadCloudIcon } from "lucide-react";
import { NewFolderDialog } from "../dialogs";

type BrowseHeaderToolbarProps = {
  onUploadFile: () => void;
  onUploadFolder: () => void;
  onCreateFolder: (name: string) => Promise<void>;
};

export const BrowseHeaderToolbar = ({
  onUploadFile,
  onUploadFolder,
  onCreateFolder,
}: BrowseHeaderToolbarProps) => (
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
            Upload Files
          </DropdownMenuItem>

          <DropdownMenuItem
            onClick={onUploadFolder}
            className="cursor-pointer gap-2 rounded-sm py-2">
            <FolderPlus className="size-4" />
            Upload Folder
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </div>
);
