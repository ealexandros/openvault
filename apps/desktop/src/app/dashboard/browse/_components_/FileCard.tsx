import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { type FileItem } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { formatBytes } from "@/utils/format";
import { motion } from "framer-motion";
import {
  FileAudioIcon,
  FileCodeIcon,
  FileIcon,
  FileImageIcon,
  FileTextIcon,
  FileVideoIcon,
  PencilIcon,
  Trash2Icon,
  X,
  type LucideIcon,
} from "lucide-react";

type FileCardProps = {
  item: FileItem;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
};

const ICON_MAP: Record<string, LucideIcon> = {
  image: FileImageIcon,
  video: FileVideoIcon,
  audio: FileAudioIcon,
  code: FileCodeIcon,
  text: FileTextIcon,
  default: FileIcon,
};

const getFileType = (extension: string) => {
  const ext = extension.toLowerCase();
  if (["png", "jpg", "jpeg", "gif", "webp", "svg"].includes(ext)) return "image";
  if (["mp4", "mov", "avi", "webm"].includes(ext)) return "video";
  if (["mp3", "wav", "ogg", "flac"].includes(ext)) return "audio";
  if (["js", "ts", "tsx", "html", "css", "json", "md"].includes(ext)) return "code";
  if (["pdf", "doc", "docx", "txt"].includes(ext)) return "text";
  return "default";
};

export const FileCard = ({ item, onClick, onDelete, onRename }: FileCardProps) => {
  const fileType = getFileType(item.extension);
  const IconComponent = ICON_MAP[fileType] ?? ICON_MAP.default ?? X;

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <motion.div
          layout
          whileHover={{ y: -2 }}
          onClick={onClick}
          className={cn(
            "group relative flex cursor-pointer flex-col gap-4 overflow-hidden rounded-2xl border p-4 transition-all duration-300",
            "border-border/40 bg-card/40 backdrop-blur-xs hover:border-border hover:bg-card hover:shadow-lg hover:shadow-black/5",
          )}>
          <div className="flex items-start justify-between">
            <div
              className={cn(
                "flex size-10 items-center justify-center rounded-xl border transition-all duration-300",
                "border-border/50 bg-muted/50 text-muted-foreground group-hover:border-foreground/10 group-hover:bg-foreground group-hover:text-background group-hover:shadow-lg",
              )}>
              <IconComponent className="size-5" />
            </div>
          </div>

          <div className="space-y-1">
            <p className="truncate text-sm font-semibold tracking-tight text-foreground/90 transition-colors group-hover:text-foreground">
              {item.name}
            </p>
            <div className="flex items-center gap-1.5 opacity-70">
              <span className="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
                {formatBytes(item.size)}
              </span>
              <span className="text-[10px] text-muted-foreground/30">•</span>
              <span className="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
                {item.extension}
              </span>
            </div>
          </div>

          <div className="pointer-events-none absolute inset-0 bg-linear-to-tr from-muted/5 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        </motion.div>
      </ContextMenuTrigger>
      <ContextMenuContent className="w-48 overflow-hidden rounded-xl border-border/50 bg-background/95 backdrop-blur-xl">
        <ContextMenuItem
          onClick={e => {
            e.stopPropagation();
            onRename();
          }}
          className="gap-2.5 py-2.5">
          <PencilIcon className="size-4 text-muted-foreground" />
          <span className="font-medium">Rename file</span>
        </ContextMenuItem>
        <ContextMenuItem
          variant="destructive"
          onClick={e => {
            e.stopPropagation();
            onDelete();
          }}
          className="gap-2.5 py-2.5">
          <Trash2Icon className="size-4" />
          <span className="font-medium">Move to Trash</span>
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
};
