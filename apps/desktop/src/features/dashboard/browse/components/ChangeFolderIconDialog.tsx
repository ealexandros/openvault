import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { FOLDER_ICON_OPTIONS } from "../hooks/useFolder";

type ChangeFolderIconDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onSelectIcon: (iconName: string) => void;
};

export const ChangeFolderIconDialog = ({
  isOpen,
  onOpenChange,
  onSelectIcon,
}: ChangeFolderIconDialogProps) => (
  <Dialog open={isOpen} onOpenChange={onOpenChange}>
    <DialogContent>
      <DialogHeader>
        <DialogTitle className="text-base">Change folder icon</DialogTitle>
        <DialogDescription className="text-sm">
          Pick an icon for this folder.
        </DialogDescription>
      </DialogHeader>

      <div className="grid grid-cols-5 gap-2">
        {FOLDER_ICON_OPTIONS.map(({ name, Icon }) => (
          <Button
            key={name}
            type="button"
            variant="outline"
            onClick={() => onSelectIcon(name)}
            className="h-12 rounded-lg p-0"
            title={name}>
            <Icon className="size-5" />
            <span className="sr-only">{name}</span>
          </Button>
        ))}
      </div>
    </DialogContent>
  </Dialog>
);
