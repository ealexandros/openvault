import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { PencilIcon, StarIcon, Trash2Icon } from "lucide-react";
import { type ReactNode } from "react";

type FileContextMenuProps = {
  isFavourite: boolean;
  children: ReactNode;
  onRename: () => void;
  onToggleFavourite: () => void;
  onDelete: () => void;
};

export const FileContextMenu = ({
  isFavourite,
  children,
  onRename,
  onToggleFavourite,
  onDelete,
}: FileContextMenuProps) => (
  <ContextMenu>
    <ContextMenuTrigger asChild>{children}</ContextMenuTrigger>
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
        onClick={e => {
          e.stopPropagation();
          onToggleFavourite();
        }}
        className="gap-2.5 py-2.5">
        <StarIcon className="size-4" />
        <span className="font-medium">{isFavourite ? "Unfavourite" : "Favourite"}</span>
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
