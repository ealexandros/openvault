import { Button } from "@/components/ui/shadcn/button";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";
import { BrowseSection, FileItem } from "@/features/dashboard/browse";
import { FileItemResult } from "@/types/filesystem";
import { FileIcon, UploadIcon } from "lucide-react";

type FilesSectionProps = {
  files: FileItemResult[];
  onFileClick: (file: FileItemResult) => void;
  onFileDelete: (file: FileItemResult) => void;
  onFileRename: (file: FileItemResult) => void;
  onFileToggleFavourite: (file: FileItemResult) => void;
  onFileProperties: (file: FileItemResult) => void;
  onFileExport: (file: FileItemResult) => void;
  onUploadFile: () => void;
};

export const FilesSection = ({
  files,
  onFileClick,
  onFileDelete,
  onFileRename,
  onFileToggleFavourite,
  onFileProperties,
  onFileExport,
  onUploadFile,
}: FilesSectionProps) => (
  <BrowseSection title="Files" count={files.length} icon={FileIcon}>
    {files.length > 0 ? (
      <div className="grid grid-cols-2 gap-5 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
        {files.map(file => (
          <FileItem
            key={file.id}
            file={file}
            onClick={() => onFileClick(file)}
            onDelete={() => onFileDelete(file)}
            onRename={() => onFileRename(file)}
            onToggleFavourite={() => onFileToggleFavourite(file)}
            onProperties={() => onFileProperties(file)}
            onExport={() => onFileExport(file)}
          />
        ))}
      </div>
    ) : (
      <Empty className="border-2 border-dashed border-muted py-12">
        <EmptyHeader>
          <EmptyTitle>No Files Found</EmptyTitle>
          <EmptyDescription>No files were found in this folder.</EmptyDescription>
        </EmptyHeader>
        <EmptyContent className="flex-row justify-center gap-2">
          <Button className="h-8 px-3" onClick={onUploadFile}>
            <UploadIcon className="size-3.5" />
            <span className="mt-0.5">Upload Files</span>
          </Button>
        </EmptyContent>
      </Empty>
    )}
  </BrowseSection>
);
