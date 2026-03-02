import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { type FileItem } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { formatBytes } from "@/utils/format";
import { FileTextIcon, PencilIcon, Trash2Icon } from "lucide-react";

type FileCardProps = {
  item: FileItem;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
};

export const FileCard = ({ item, onClick, onDelete, onRename }: FileCardProps) => (
  <ContextMenu>
    <ContextMenuTrigger asChild>
      <div
        onClick={onClick}
        className={cn(
          "group relative flex cursor-pointer flex-col gap-3 overflow-hidden rounded-2xl border p-4 transition-all duration-300",
          "border-border/40 bg-card hover:border-border hover:bg-muted/30 hover:shadow-md",
        )}>
        <div className="flex items-start justify-between">
          <div
            className={cn(
              "rounded-xl border p-2.5 transition-all duration-300",
              "border-border/50 bg-muted/50 text-muted-foreground group-hover:border-foreground/10 group-hover:bg-background group-hover:text-foreground",
            )}>
            <FileTextIcon className="size-5" />
          </div>
        </div>

        <div className="space-y-0.5">
          <p className="truncate text-sm font-semibold text-foreground/90 transition-colors group-hover:text-foreground">
            {item.name}
          </p>
          <div className="flex items-center gap-2">
            <p className="text-[10px] font-medium tracking-wider text-muted-foreground/70 uppercase">
              {formatBytes(item.size)}
            </p>
            <span className="text-[10px] text-muted-foreground/30">•</span>
            <p className="text-[10px] font-medium tracking-wider text-muted-foreground/70 uppercase">
              {item.extension}
            </p>
          </div>
        </div>

        <div className="pointer-events-none absolute inset-0 bg-linear-to-tr from-white/5 via-transparent to-transparent opacity-0 transition-opacity duration-500 group-hover:opacity-100" />
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent>
      <ContextMenuItem
        onClick={e => {
          e.stopPropagation();
          onRename();
        }}>
        <PencilIcon className="mr-2 size-3.5" />
        Rename
      </ContextMenuItem>
      <ContextMenuItem
        variant="destructive"
        onClick={e => {
          e.stopPropagation();
          onDelete();
        }}>
        <Trash2Icon className="mr-2 size-3.5" />
        Delete
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
);
