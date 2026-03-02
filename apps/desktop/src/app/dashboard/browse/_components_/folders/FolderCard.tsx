import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { type FolderItem } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { motion } from "framer-motion";
import { ChevronRightIcon, FolderIcon, PencilIcon, Trash2Icon } from "lucide-react";

type FolderCardProps = {
  item: FolderItem;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
};

export const FolderCard = ({ item, onClick, onDelete, onRename }: FolderCardProps) => (
  <ContextMenu>
    <ContextMenuTrigger asChild>
      <motion.div
        layout
        whileHover={{ y: -2 }}
        onClick={onClick}
        className={cn(
          "group relative flex cursor-pointer items-center gap-4 overflow-hidden rounded-2xl border p-3.5 transition-all duration-300",
          "border-border/40 bg-card/40 backdrop-blur-xs hover:border-primary/40 hover:bg-card hover:shadow-lg hover:shadow-primary/5",
        )}>
        <div
          className={cn(
            "flex size-11 items-center justify-center rounded-xl border transition-all duration-300",
            "border-primary/10 bg-primary/5 text-primary group-hover:bg-primary group-hover:text-white group-hover:shadow-lg group-hover:shadow-primary/20",
          )}>
          <FolderIcon className="size-5.5 fill-current/10" strokeWidth={2.25} />
        </div>

        <div className="min-w-0 flex-1">
          <p className="truncate text-sm font-semibold tracking-tight text-foreground/90 transition-colors group-hover:text-foreground">
            {item.name}
          </p>
          <div className="flex items-center gap-1.5 opacity-70">
            <span className="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
              {item.itemCount} items
            </span>
          </div>
        </div>

        <div className="mr-1 text-muted-foreground/30 transition-transform duration-300 group-hover:translate-x-1 group-hover:text-primary/50">
          <ChevronRightIcon className="size-4" />
        </div>

        <div className="pointer-events-none absolute inset-0 bg-linear-to-tr from-primary/5 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
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
        <span className="font-medium">Rename folder</span>
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
