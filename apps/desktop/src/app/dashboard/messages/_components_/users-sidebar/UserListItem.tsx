import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { type MessageContact } from "@/types/messages";
import { cn } from "@/utils/cn";
import { motion } from "framer-motion";
import { Edit2, ShieldCheck, Trash2 } from "lucide-react";
import { useState } from "react";

type UserListItemProps = {
  user: MessageContact;
  selected: boolean;
  onClick: () => void;
  onRename: (id: string, newName: string) => Promise<void>;
  onDelete: (id: string) => Promise<void>;
};

export const UserListItem = ({
  user,
  selected,
  onClick,
  onRename,
  onDelete,
}: UserListItemProps) => {
  const [isRenaming, setIsRenaming] = useState(false);
  const [newName, setNewName] = useState(user.name);

  const handleRename = async () => {
    if (newName.trim() && newName !== user.name) {
      await onRename(user.id, newName.trim());
    }
    setIsRenaming(false);
  };

  return (
    <>
      <ContextMenu>
        <ContextMenuTrigger asChild>
          <motion.button
            layout
            onClick={onClick}
            className={cn(
              "group w-full rounded-xl border p-3 text-left transition-all",
              selected
                ? "border-primary/50 bg-primary/5"
                : "border-transparent hover:border-border hover:bg-muted/50",
            )}>
            <div className="flex items-start gap-3">
              <div
                className={cn(
                  "flex h-8 w-8 items-center justify-center rounded-lg border text-xs font-semibold",
                  selected
                    ? "border-primary/20 bg-primary/10 text-primary"
                    : "border-border bg-muted/30 text-muted-foreground",
                )}>
                {user.name.charAt(0).toUpperCase()}
              </div>

              <div className="flex-1 overflow-hidden">
                <div className="flex justify-between">
                  <p className="truncate text-sm font-medium">{user.name}</p>

                  {user.secure && <ShieldCheck className="h-3 w-3 text-primary" />}
                </div>

                <p className="truncate text-xs text-muted-foreground">
                  {user.expiresAt != null
                    ? `Expires: ${new Date(user.expiresAt).toLocaleDateString()}`
                    : "Never expires"}
                </p>
              </div>
            </div>
          </motion.button>
        </ContextMenuTrigger>
        <ContextMenuContent className="w-48">
          <ContextMenuItem onClick={() => setIsRenaming(true)} className="gap-2">
            <Edit2 className="h-3.5 w-3.5" />
            <span>Rename Contact</span>
          </ContextMenuItem>
          <ContextMenuSeparator />
          <ContextMenuItem
            onClick={() => onDelete(user.id)}
            className="gap-2 text-destructive focus:bg-destructive/10 focus:text-destructive">
            <Trash2 className="h-3.5 w-3.5" />
            <span>Delete Contact</span>
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>

      <Dialog open={isRenaming} onOpenChange={setIsRenaming}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Rename Contact</DialogTitle>
            <DialogDescription>Change the display name for this contact.</DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="name">Display Name</Label>
              <Input
                id="name"
                value={newName}
                onChange={e => setNewName(e.target.value)}
                placeholder="Enter new name..."
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsRenaming(false)}>
              Cancel
            </Button>
            <Button onClick={handleRename}>Save Changes</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};
