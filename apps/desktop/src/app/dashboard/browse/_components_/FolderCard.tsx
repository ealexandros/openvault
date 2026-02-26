import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { FolderItem } from "@/libraries/tauri-api";
import { cn } from "@/utils/cn";
import { FolderIcon, MoreVerticalIcon, PencilIcon, Trash2Icon } from "lucide-react";

type FolderCardProps = {
  item: FolderItem;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
};

export const FolderCard = ({ item, onClick, onDelete, onRename }: FolderCardProps) => (
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

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            onClick={e => e.stopPropagation()}
            className="rounded-lg p-1.5 text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 hover:bg-foreground/5 hover:text-foreground">
            <MoreVerticalIcon className="size-4" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem
            onClick={e => {
              e.stopPropagation();
              onRename();
            }}>
            <PencilIcon className="mr-2 size-3.5" />
            Rename
          </DropdownMenuItem>
          <DropdownMenuItem
            variant="destructive"
            onClick={e => {
              e.stopPropagation();
              onDelete();
            }}>
            <Trash2Icon className="mr-2 size-3.5" />
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>

    <div className="space-y-0.5">
      <p className="truncate text-sm font-semibold text-foreground/90 transition-colors group-hover:text-foreground">
        {item.name}
      </p>
      <p className="text-[10px] font-medium tracking-wider text-muted-foreground/70 uppercase">
        {item.item_count} items
      </p>
    </div>

    <div className="pointer-events-none absolute inset-0 bg-linear-to-tr from-primary/5 via-transparent to-transparent opacity-0 transition-opacity duration-500 group-hover:opacity-100" />
  </div>
);
