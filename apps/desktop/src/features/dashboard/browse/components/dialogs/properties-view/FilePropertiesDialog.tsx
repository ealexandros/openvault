import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { type FileItemResult } from "@/types/filesystem";
import { formatBytes, formatFromIsoString } from "@/utils/format";
import { PropertyRow } from "./PropertyRow";

type FilePropertiesDialogProps = {
  isOpen: boolean;
  item: FileItemResult | null;
  onOpenChange: (open: boolean) => void;
};

export const FilePropertiesDialog = ({
  isOpen,
  item,
  onOpenChange,
}: FilePropertiesDialogProps) => (
  <Dialog open={isOpen} onOpenChange={onOpenChange}>
    <DialogContent className="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle className="text-base">File properties</DialogTitle>
        <DialogDescription className="text-sm">Metadata for this file.</DialogDescription>
      </DialogHeader>

      <div className="space-y-2">
        <PropertyRow label="name" value={item?.name ?? "-"} />
        <PropertyRow label="size" value={item != null ? formatBytes(item.size) : "-"} />
        <PropertyRow
          label="extension"
          value={item?.extension != null && item.extension.length > 0 ? item.extension : "-"}
        />
        <PropertyRow
          label="created_at"
          value={item != null ? formatFromIsoString(item.createdAt) : "-"}
        />
        <PropertyRow
          label="updated_at"
          value={item != null ? formatFromIsoString(item.updatedAt) : "-"}
        />
      </div>
    </DialogContent>
  </Dialog>
);
