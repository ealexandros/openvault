import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { type FolderItemResult } from "@/types/filesystem";
import { formatBytes, formatFromIsoString } from "@/utils/format";
import { PropertyRow } from "./PropertyRow";

type FolderPropertiesDialogProps = {
  isOpen: boolean;
  item: FolderItemResult | null;
  onOpenChange: (open: boolean) => void;
};

export const FolderPropertiesDialog = ({
  isOpen,
  item,
  onOpenChange,
}: FolderPropertiesDialogProps) => (
  <Dialog open={isOpen} onOpenChange={onOpenChange}>
    <DialogContent className="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle className="text-base">Folder properties</DialogTitle>
        <DialogDescription className="text-sm">Metadata for this folder.</DialogDescription>
      </DialogHeader>

      <div className="space-y-2">
        <PropertyRow label="name" value={item?.name ?? "-"} />
        <PropertyRow label="size" value={item ? formatBytes(item.totalSizeBytes) : "-"} />
        <PropertyRow label="children" value={item ? String(item.itemCount) : "-"} />
        <PropertyRow
          label="created_at"
          value={item ? formatFromIsoString(item.createdAt) : "-"}
        />
        <PropertyRow
          label="updated_at"
          value={item ? formatFromIsoString(item.updatedAt) : "-"}
        />
      </div>
    </DialogContent>
  </Dialog>
);
