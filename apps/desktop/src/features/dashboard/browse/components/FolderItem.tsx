import { Badge } from "@/components/ui/shadcn/badge";
import { FolderItemResult } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { ChevronRightIcon, FolderIcon, StarIcon, type LucideProps } from "lucide-react";
import { FolderIconName, ICON_MAP } from "../hooks/useFolder";
import { FolderContextMenu } from "./FolderContextMenu";

type FolderItemProps = {
  folder: FolderItemResult;
  onClick: () => void;
  onDelete: () => void;
  onRename: () => void;
  onChangeIcon: () => void;
  onToggleFavourite: () => void;
  onProperties: () => void;
  onExport: () => void;
};

const renderFolderIcon = (iconName: string, props?: LucideProps) => {
  const Icon = iconName in ICON_MAP ? ICON_MAP[iconName as FolderIconName] : FolderIcon;
  return <Icon {...props} />;
};

export const FolderItem = ({
  folder,
  onClick,
  onDelete,
  onRename,
  onChangeIcon,
  onToggleFavourite,
  onProperties,
  onExport,
}: FolderItemProps) => (
  <div className="relative">
    <FolderContextMenu
      isFavourite={folder.isFavourite}
      onRename={onRename}
      onChangeIcon={onChangeIcon}
      onDelete={onDelete}
      onToggleFavourite={onToggleFavourite}
      onProperties={onProperties}
      onExport={onExport}>
      <div
        onClick={onClick}
        className={cn(
          "group relative flex cursor-pointer items-center gap-4 overflow-hidden rounded-2xl border p-3.5 transition-all duration-300",
          "border-border/40 bg-card/40 backdrop-blur-xs hover:border-primary/40 hover:bg-card hover:shadow-lg hover:shadow-primary/3",
        )}>
        <div
          className={cn(
            "flex h-11 w-11 items-center justify-center rounded-xl border transition-all duration-300",
            "border-primary/10 bg-primary/5 text-primary",
          )}>
          {renderFolderIcon(folder.icon, { className: "size-5 fill-current/10" })}
        </div>

        <div className="min-w-0 flex-1">
          <p
            title={folder.name}
            className="truncate text-sm font-semibold tracking-tight text-foreground/90 transition-colors group-hover:text-foreground">
            {folder.name}
          </p>
          <div className="flex items-center gap-2 opacity-70">
            <span className="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
              {folder.itemCount} items
            </span>
          </div>
        </div>

        <div className="mr-1 text-muted-foreground/30 transition-transform duration-300 group-hover:translate-x-1 group-hover:text-primary/50">
          <ChevronRightIcon className="size-4" />
        </div>

        <div
          className={cn(
            "pointer-events-none absolute inset-0 bg-linear-to-r from-primary/5 via-transparent to-transparent",
            "opacity-0 transition-opacity duration-300 group-hover:opacity-100",
          )}
        />
      </div>
    </FolderContextMenu>

    {folder.isFavourite && (
      <Badge variant="outline" className="absolute -top-1 -right-1 size-7 bg-card p-0">
        <StarIcon className="text-yellow-500" fill="currentColor" />
      </Badge>
    )}
  </div>
);
