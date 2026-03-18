import { BrowseSection, FolderBackButton, FolderItem } from "@/features/dashboard/browse";
import { FolderItemResult } from "@/types/filesystem";
import { FolderIcon } from "lucide-react";

type FoldersSectionProps = {
  folders: FolderItemResult[];
  canGoBack: boolean;
  isNavigating: boolean;
  onBackClick: () => void;
  onFolderClick: (folder: FolderItemResult) => void;
  onFolderDelete: (folder: FolderItemResult) => void;
  onFolderRename: (folder: FolderItemResult) => void;
  onFolderToggleFavourite: (folder: FolderItemResult) => Promise<void>;
  onFolderChangeIcon: (folder: FolderItemResult) => void;
  onFolderProperties: (folder: FolderItemResult) => void;
  onFolderExport: (folder: FolderItemResult) => void;
};

export const FoldersSection = ({
  folders,
  canGoBack,
  isNavigating,
  onBackClick,
  onFolderClick,
  onFolderDelete,
  onFolderRename,
  onFolderToggleFavourite,
  onFolderChangeIcon,
  onFolderProperties,
  onFolderExport,
}: FoldersSectionProps) => {
  if (folders.length === 0 && !canGoBack) {
    return null;
  }

  return (
    <BrowseSection title="Folders" count={folders.length} icon={FolderIcon}>
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {canGoBack && !isNavigating && <FolderBackButton onBackClick={onBackClick} />}
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
    </BrowseSection>
  );
};
