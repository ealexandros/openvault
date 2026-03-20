import { ContextMenu } from "@/components/ui/shadcn-ext/ContextMenu";
import {
  EllipsisVertical,
  HeartIcon,
  ImageIcon,
  PencilIcon,
  Trash2Icon,
  UploadIcon,
} from "lucide-react";
import { type ReactNode } from "react";

type FolderContextMenuProps = {
  isFavourite: boolean;
  children: ReactNode;
  onRename: () => void;
  onChangeIcon: () => void;
  onDelete: () => void;
  onToggleFavourite: () => void;
  onProperties: () => void;
  onExport: () => void;
};

export const FolderContextMenu = ({
  isFavourite,
  children,
  onRename,
  onChangeIcon,
  onDelete,
  onToggleFavourite,
  onProperties,
  onExport,
}: FolderContextMenuProps) => {
  const items = [
    {
      label: isFavourite ? "Unfavourite" : "Favourite",
      icon: HeartIcon,
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
      label: "Export",
      icon: UploadIcon,
      onClick: onExport,
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
