import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import {
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/shadcn/sidebar";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/shadcn/tooltip";
import { type MessageContact } from "@/types/messages";
import { cn } from "@/utils/cn";
import { Download, Edit2, MoreVertical, Trash2 } from "lucide-react";
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

  const isExpired = user.expiresAt != null && new Date(user.expiresAt) < new Date();

  return (
    <>
      <SidebarMenuItem>
        <SidebarMenuButton
          size="lg"
          isActive={selected}
          onClick={onClick}
          className="group h-14 cursor-pointer rounded-lg transition-all duration-200">
          <div className="relative">
            <div className="flex size-10 items-center justify-center rounded-lg border border-primary/10 bg-primary/5 text-sm font-bold text-primary">
              {user.name.charAt(0).toUpperCase()}
            </div>
            <TooltipProvider delayDuration={400}>
              <Tooltip>
                <TooltipTrigger asChild>
                  <div
                    className={cn(
                      "absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-background shadow-xs",
                      isExpired ? "bg-destructive" : "bg-emerald-500",
                    )}
                  />
                </TooltipTrigger>
                <TooltipContent side="bottom" className="text-[10px]">
                  <p>{isExpired ? "Profile Expired" : "Profile Active"}</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
          <div className="flex flex-1 flex-col gap-0.5 overflow-hidden px-1">
            <p className="truncate text-sm font-semibold tracking-tight">{user.name}</p>
            <p className="truncate text-[10px] font-medium text-muted-foreground/70 uppercase">
              {user.expiresAt != null
                ? `Expires ${new Date(user.expiresAt).toLocaleDateString()}`
                : "Lifetime Access"}
            </p>
          </div>

          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <SidebarMenuAction
                className="my-2.5 mr-2 ml-auto flex cursor-pointer p-1 transition-all hover:bg-foreground/5"
                onClick={e => e.stopPropagation()}>
                <MoreVertical className="size-4 text-muted-foreground hover:text-foreground" />
              </SidebarMenuAction>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-48">
              <DropdownMenuItem
                onClick={e => {
                  e.stopPropagation();
                  setIsRenaming(true);
                }}
                className="gap-2 text-xs font-medium">
                <Edit2 className="h-3.5 w-3.5" />
                <span>Rename Contact</span>
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={e => {
                  e.stopPropagation();
                }}
                className="gap-2 text-xs font-medium">
                <Download className="h-3.5 w-3.5" />
                <span>Export Profile</span>
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={e => {
                  e.stopPropagation();
                  void onDelete(user.id);
                }}
                className="gap-2 text-xs font-medium text-destructive focus:bg-destructive/10 focus:text-destructive">
                <Trash2 className="h-3.5 w-3.5" />
                <span>Delete Contact</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </SidebarMenuButton>
      </SidebarMenuItem>

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
                onKeyDown={e => {
                  if (e.key === "Enter") void handleRename();
                }}
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
