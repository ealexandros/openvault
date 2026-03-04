import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { type FileItemResult } from "@/types/filesystem";
import { formatBytes, formatFromIsoString } from "@/utils/format";

type FilePropertiesDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  item: FileItemResult | null;
};

const PropertyRow = ({ label, value }: { label: string; value: string }) => (
  <div className="flex items-center justify-between gap-4 rounded-md border border-border/60 bg-muted/20 px-3 py-2 text-sm">
    <span className="font-medium text-muted-foreground">{label}</span>
    <span className="truncate text-right font-mono text-foreground/90">{value}</span>
  </div>
);

export const FilePropertiesDialog = ({
  isOpen,
  onOpenChange,
  item,
}: FilePropertiesDialogProps) => {
  const extension = item?.extension.trim();

  return (
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
            value={extension != null && extension.length > 0 ? extension : "-"}
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
};
