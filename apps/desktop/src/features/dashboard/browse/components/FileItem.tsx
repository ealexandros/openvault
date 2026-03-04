import { Badge } from "@/components/ui/shadcn/badge";
import { type FileItemResult } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { formatBytes } from "@/utils/format";
import { getFileTypeOrDefault } from "@/utils/mime-types";
import {
  FileAudioIcon,
  FileCodeIcon,
  FileIcon,
  FileImageIcon,
  FileTextIcon,
  FileVideoIcon,
  StarIcon,
  type LucideIcon,
} from "lucide-react";
import { FileContextMenu } from "./FileContextMenu";

type FileItemProps = {
  file: FileItemResult;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
  onToggleFavourite: () => void;
  onProperties: () => void;
  onExport: () => void;
};

const ICON_MAP: Record<string, LucideIcon> = {
  image: FileImageIcon,
  video: FileVideoIcon,
  audio: FileAudioIcon,
  code: FileCodeIcon,
  text: FileTextIcon,
  default: FileIcon,
};

export const FileItem = ({
  file,
  onClick,
  onDelete,
  onRename,
  onToggleFavourite,
  onProperties,
  onExport,
}: FileItemProps) => {
  const fileType = getFileTypeOrDefault(file.extension);
  const FileIconComponent = ICON_MAP[fileType] ?? FileTextIcon;

  return (
    <div className="relative">
      <FileContextMenu
        onRename={onRename}
        onDelete={onDelete}
        onToggleFavourite={onToggleFavourite}
        onProperties={onProperties}
        onExport={onExport}
        isFavourite={file.isFavourite}>
        <div
          onClick={onClick}
          className={cn(
            "flex cursor-pointer flex-col gap-4 overflow-hidden rounded-2xl border border-border/40 bg-card p-4",
            "group transition-all duration-300 hover:border-border hover:bg-card hover:shadow-lg hover:shadow-gray-300/5",
          )}>
          <div className="flex items-start justify-between">
            <div
              className={cn(
                "flex size-10 items-center justify-center rounded-xl border border-border/40 bg-card/40 text-muted-foreground",
                "transition-all duration-300 group-hover:border-foreground/10 group-hover:bg-foreground group-hover:text-background group-hover:shadow-md",
              )}>
              <FileIconComponent className="size-5" />
            </div>
          </div>

          <div className="space-y-1">
            <p title={file.name} className="truncate text-sm font-semibold text-foreground">
              {file.name}
            </p>
            <div className="flex items-center gap-1.5 text-[10px] text-muted-foreground">
              <span className="font-bold uppercase">{formatBytes(file.size)}</span>
              <span>•</span>
              <span className="font-bold uppercase">{file.extension}</span>
            </div>
          </div>
        </div>
      </FileContextMenu>

      {file.isFavourite && (
        <Badge variant="outline" className="absolute -top-1 -right-1 size-7 bg-card">
          <StarIcon className="text-yellow-500" fill="currentColor" />
        </Badge>
      )}
    </div>
  );
};
