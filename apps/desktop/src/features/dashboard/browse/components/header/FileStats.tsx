import { FileIcon, FolderIcon } from "lucide-react";

type FileStatsProps = {
  folderCount: number;
  fileCount: number;
};

export const FileStats = ({ folderCount, fileCount }: FileStatsProps) => (
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
);
