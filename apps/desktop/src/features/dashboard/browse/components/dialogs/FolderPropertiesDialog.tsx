import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { type FolderItemResult } from "@/types/filesystem";

type FolderPropertiesDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  item: FolderItemResult | null;
};

const formatDateTime = (value: string) => {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return date.toLocaleString();
};

const PropertyRow = ({ label, value }: { label: string; value: string }) => (
  <div className="flex items-center justify-between gap-4 rounded-md border border-border/60 bg-muted/20 px-3 py-2 text-sm">
    <span className="font-medium text-muted-foreground">{label}</span>
    <span className="truncate text-right font-mono text-foreground/90">{value}</span>
  </div>
);

export const FolderPropertiesDialog = ({
  isOpen,
  onOpenChange,
  item,
}: FolderPropertiesDialogProps) => (
  <Dialog open={isOpen} onOpenChange={onOpenChange}>
    <DialogContent className="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle className="text-base">Folder properties</DialogTitle>
        <DialogDescription className="text-sm">Metadata for this folder.</DialogDescription>
      </DialogHeader>

      <div className="space-y-2">
        <PropertyRow label="name" value={item?.name ?? "-"} />
        <PropertyRow label="size" value="-" />
        <PropertyRow label="extension" value="-" />
        <PropertyRow label="children" value={item ? String(item.itemCount) : "-"} />
        <PropertyRow label="created_at" value={item ? formatDateTime(item.createdAt) : "-"} />
        <PropertyRow label="updated_at" value={item ? formatDateTime(item.updatedAt) : "-"} />
      </div>
    </DialogContent>
  </Dialog>
);
