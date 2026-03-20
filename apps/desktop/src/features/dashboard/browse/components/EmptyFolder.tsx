"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";
import { FolderIcon, UploadIcon } from "lucide-react";

type EmptyFolderProps = {
  canGoBack: boolean;
  onGoBack: () => void;
  onUploadFile: () => void;
};

export const EmptyFolder = ({ canGoBack, onGoBack, onUploadFile }: EmptyFolderProps) => (
  <Empty className="border-2 border-dashed border-muted py-52">
    <EmptyHeader>
      <EmptyMedia variant="icon">
        <FolderIcon />
      </EmptyMedia>
      <EmptyTitle>Empty Folder</EmptyTitle>
      <EmptyDescription>
        This folder is empty. Upload files or create folders to get started.
      </EmptyDescription>
    </EmptyHeader>
    <EmptyContent className="flex-row justify-center gap-2">
      {canGoBack && (
        <Button variant="outline" className="h-8 px-3" onClick={onGoBack}>
          Go Back
        </Button>
      )}
      <Button className="h-8 px-3" onClick={onUploadFile}>
        <UploadIcon className="size-3.5" />
        <span className="mt-0.5">Upload Files</span>
      </Button>
    </EmptyContent>
  </Empty>
);
