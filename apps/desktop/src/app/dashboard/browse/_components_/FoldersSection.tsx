import { Button } from "@/components/ui/shadcn/button";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";
import { BrowseSection, FolderItem } from "@/features/dashboard/browse";
import { FolderItemResult } from "@/types/filesystem";
import { FolderIcon, UploadIcon } from "lucide-react";

type FoldersSectionProps = {
  folders: FolderItemResult[];
  onFolderClick: (folder: FolderItemResult) => void;
  onFolderDelete: (folder: FolderItemResult) => void;
  onFolderRename: (folder: FolderItemResult) => void;
  onFolderToggleFavourite: (folder: FolderItemResult) => Promise<void>;
  onFolderChangeIcon: (folder: FolderItemResult) => void;
  onFolderProperties: (folder: FolderItemResult) => void;
  onFolderExport: (folder: FolderItemResult) => void;
  onUploadFolder: () => void;
};

export const FoldersSection = ({
  folders,
  onFolderClick,
  onFolderDelete,
  onFolderRename,
  onFolderToggleFavourite,
  onFolderChangeIcon,
  onFolderProperties,
  onFolderExport,
  onUploadFolder,
}: FoldersSectionProps) => (
  <BrowseSection title="Folders" count={folders.length} icon={FolderIcon}>
    {folders.length > 0 ? (
      <div className="grid grid-cols-2 gap-5 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
        {folders.map(item => (
          <FolderItem
            key={item.id}
            folder={item}
            onClick={() => onFolderClick(item)}
            onDelete={() => onFolderDelete(item)}
            onRename={() => onFolderRename(item)}
            onChangeIcon={() => onFolderChangeIcon(item)}
            onToggleFavourite={() => onFolderToggleFavourite(item)}
            onProperties={() => onFolderProperties(item)}
            onExport={() => onFolderExport(item)}
          />
        ))}
      </div>
    ) : (
      <Empty className="border-2 border-dashed border-muted py-12">
        <EmptyHeader>
          <EmptyTitle>No Folders Found</EmptyTitle>
          <EmptyDescription>No folders were found in this folder.</EmptyDescription>
        </EmptyHeader>
        <EmptyContent className="flex-row justify-center gap-2">
          <Button className="h-8 px-3" onClick={onUploadFolder}>
            <UploadIcon className="size-3.5" />
            <span className="mt-0.5">Upload Foler</span>
          </Button>
        </EmptyContent>
      </Empty>
    )}
  </BrowseSection>
);
