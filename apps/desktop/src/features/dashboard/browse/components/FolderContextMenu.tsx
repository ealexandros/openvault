import { ContextMenu } from "@/components/ui/shadcn-ext/ContextMenu";
import { EllipsisVertical, ImageIcon, PencilIcon, StarIcon, Trash2Icon } from "lucide-react";
import { type ReactNode } from "react";

type FolderContextMenuProps = {
  isFavourite: boolean;
  children: ReactNode;
  onRename: () => void;
  onChangeIcon: () => void;
  onDelete: () => void;
  onToggleFavourite: () => void;
  onProperties: () => void;
};

export const FolderContextMenu = ({
  isFavourite,
  children,
  onRename,
  onChangeIcon,
  onDelete,
  onToggleFavourite,
  onProperties,
}: FolderContextMenuProps) => {
  const items = [
    {
      label: isFavourite ? "Unfavourite" : "Favourite",
      icon: StarIcon,
      onClick: onToggleFavourite,
    },
    {
      label: "Rename folder",
      icon: PencilIcon,
      onClick: onRename,
    },
    {
      label: "Change icon",
      icon: ImageIcon,
      onClick: onChangeIcon,
    },
    {
      label: "Properties",
      icon: EllipsisVertical,
      onClick: onProperties,
    },
    {
      label: "Move to Trash",
      icon: Trash2Icon,
      onClick: onDelete,
      variant: "destructive" as const,
    },
  ];

  return <ContextMenu items={items}>{children}</ContextMenu>;
};
