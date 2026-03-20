import { BrowseSection, FileItem } from "@/features/dashboard/browse";
import { FileItemResult } from "@/types/filesystem";
import { FileIcon } from "lucide-react";

type FilesSectionProps = {
  files: FileItemResult[];
  onFileClick: (file: FileItemResult) => void;
  onFileDelete: (file: FileItemResult) => void;
  onFileRename: (file: FileItemResult) => void;
  onFileToggleFavourite: (file: FileItemResult) => void;
  onFileProperties: (file: FileItemResult) => void;
  onFileExport: (file: FileItemResult) => void;
};

export const FilesSection = ({
  files,
  onFileClick,
  onFileDelete,
  onFileRename,
  onFileToggleFavourite,
  onFileProperties,
  onFileExport,
}: FilesSectionProps) => {
  if (files.length === 0) {
    return null;
  }

  return (
    <BrowseSection title="Files" count={files.length} icon={FileIcon}>
      <div className="grid grid-cols-3 gap-5 lg:grid-cols-4 xl:grid-cols-5">
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
    </BrowseSection>
  );
};
