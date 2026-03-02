import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { type FolderItem } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { FolderIcon, PencilIcon, Trash2Icon } from "lucide-react";

type FolderCardProps = {
  item: FolderItem;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
};

export const FolderCard = ({ item, onClick, onDelete, onRename }: FolderCardProps) => (
  <ContextMenu>
    <ContextMenuTrigger asChild>
      <div
        onClick={onClick}
        className={cn(
          "group relative flex cursor-pointer flex-col gap-3 overflow-hidden rounded-2xl border p-4 transition-all duration-300",
          "border-primary/10 bg-linear-to-br from-primary/5 to-transparent hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5",
        )}>
        <div className="flex items-start justify-between">
          <div
            className={cn(
              "rounded-xl border p-2.5 transition-all duration-300",
              "border-primary/20 bg-primary/10 text-primary group-hover:scale-110 group-hover:rotate-3",
            )}>
            <FolderIcon className="size-5" />
          </div>
        </div>

        <div className="space-y-0.5">
          <p className="truncate text-sm font-semibold text-foreground/90 transition-colors group-hover:text-foreground">
            {item.name}
          </p>
          <p className="text-[10px] font-medium tracking-wider text-muted-foreground/70 uppercase">
            {item.itemCount} items
          </p>
        </div>

        <div className="pointer-events-none absolute inset-0 bg-linear-to-tr from-primary/5 via-transparent to-transparent opacity-0 transition-opacity duration-500 group-hover:opacity-100" />
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
