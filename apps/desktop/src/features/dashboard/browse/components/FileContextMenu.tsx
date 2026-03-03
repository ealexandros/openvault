import { ContextMenu } from "@/components/ui/shadcn-ext/ContextMenu";
import { EllipsisVertical, PencilIcon, StarIcon, Trash2Icon } from "lucide-react";
import { type ReactNode } from "react";

type FileContextMenuProps = {
  isFavourite: boolean;
  children: ReactNode;
  onRename: () => void;
  onToggleFavourite: () => void;
  onDelete: () => void;
  onProperties: () => void;
};

export const FileContextMenu = ({
  isFavourite,
  children,
  onRename,
  onToggleFavourite,
  onDelete,
  onProperties,
}: FileContextMenuProps) => {
  const menuItems = [
    {
      label: isFavourite ? "Unfavourite" : "Favourite",
      icon: StarIcon,
      onClick: onToggleFavourite,
    },
    {
      label: "Rename file",
      icon: PencilIcon,
      onClick: onRename,
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

  return <ContextMenu items={menuItems}>{children}</ContextMenu>;
};
