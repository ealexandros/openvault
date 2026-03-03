import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { ImageIcon, PencilIcon, StarIcon, Trash2Icon } from "lucide-react";
import { type ReactNode } from "react";

type FolderContextMenuProps = {
  isFavourite: boolean;
  children: ReactNode;
  onRename: () => void;
  onChangeIcon: () => void;
  onDelete: () => void;
  onToggleFavourite: () => void;
};

export const FolderContextMenu = ({
  isFavourite,
  children,
  onRename,
  onChangeIcon,
  onDelete,
  onToggleFavourite,
}: FolderContextMenuProps) => (
  <ContextMenu>
    <ContextMenuTrigger asChild>{children}</ContextMenuTrigger>

    <ContextMenuContent className="w-48 overflow-hidden border-border/50 bg-background/95 backdrop-blur-xl">
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
        onClick={e => {
          e.stopPropagation();
          onToggleFavourite();
        }}
        className="gap-2.5 py-2.5">
        <StarIcon className="size-4" />
        <span className="font-medium">{isFavourite ? "Unfavourite" : "Favourite"}</span>
      </ContextMenuItem>
      <ContextMenuItem
        onClick={e => {
          e.stopPropagation();
          onChangeIcon();
        }}
        className="gap-2.5 py-2.5">
        <ImageIcon className="size-4 text-muted-foreground" />
        <span className="font-medium">Change icon</span>
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
