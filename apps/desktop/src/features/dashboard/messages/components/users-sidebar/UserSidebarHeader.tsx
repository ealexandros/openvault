import { Button } from "@/components/ui/shadcn/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/shadcn/select";
import {
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/shadcn/sidebar";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/shadcn/tooltip";
import { Clock, Download, Settings } from "lucide-react";
import { useState } from "react";
import { cn } from "@/utils/cn";

type UserCredentials = {
  name: string;
  signingPubKey: number[];
  ephemeralPubKey: number[];
  expiresAt: string | null;
};

type UserSidebarHeaderProps = {
  credentials: UserCredentials | null;
  onRenew: () => Promise<void>;
  onReset: () => Promise<void>;
  onUpdate: (data: { name: string; rotationMonths: number }) => Promise<void>;
  onExport: () => void;
};

export const UserSidebarHeader = ({
  credentials,
  onRenew,
  onReset,
  onUpdate,
  onExport,
}: UserSidebarHeaderProps) => {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [name, setName] = useState(credentials?.name ?? "");
  const [rotationMonths, setRotationMonths] = useState(12);

  const handleUpdate = async () => {
    await onUpdate({ name, rotationMonths });
    setIsSettingsOpen(false);
  };

  const isExpired = credentials?.expiresAt != null && new Date(credentials.expiresAt) < new Date();

  return (
    <>
      <SidebarMenu>
        <SidebarMenuItem>
          <div className="flex items-center gap-2 rounded-lg bg-sidebar-accent/30 p-2">
            <div className="relative">
              <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 text-sm font-bold text-primary">
                {credentials?.name.charAt(0).toUpperCase() ?? "M"}
              </div>
              <TooltipProvider delayDuration={400}>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <div
                      className={cn(
                        "absolute -right-0.5 -bottom-0.5 h-3 w-3 rounded-full border-2 border-background shadow-xs",
                        isExpired ? "bg-destructive" : "bg-emerald-500",
                      )}
                    />
                  </TooltipTrigger>
                  <TooltipContent side="bottom" className="text-[10px]">
                    <p>{isExpired ? "Identity Expired" : "Identity Active"}</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>

            <div className="flex flex-1 flex-col overflow-hidden px-1">
              <span className="truncate text-sm font-bold tracking-tight">
                {credentials?.name ?? "My Profile"}
              </span>
              <span className="truncate text-[10px] font-medium text-muted-foreground/70 uppercase tracking-wider">
                Active Session
              </span>
            </div>

            <div className="flex items-center gap-0.5">
              <TooltipProvider delayDuration={400}>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-8 w-8 rounded-md text-muted-foreground hover:bg-sidebar-accent hover:text-foreground"
                      onClick={onExport}>
                      <Download className="h-4 w-4" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="bottom">
                    <p>Export Identity</p>
                  </TooltipContent>
                </Tooltip>

                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-8 w-8 rounded-md text-muted-foreground hover:bg-sidebar-accent hover:text-foreground"
                      onClick={() => setIsSettingsOpen(true)}>
                      <Settings className="h-4 w-4" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="bottom">
                    <p>Settings</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
          </div>
        </SidebarMenuItem>
      </SidebarMenu>

      <Dialog open={isSettingsOpen} onOpenChange={setIsSettingsOpen}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Profile Settings</DialogTitle>
            <DialogDescription>
              Manage your cryptographic identity and credential policies.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-6 py-4">
            <div className="space-y-2">
              <Label htmlFor="profile-name">Display Name</Label>
              <Input
                id="profile-name"
                value={name}
                onChange={e => setName(e.target.value)}
                placeholder="Your name..."
              />
            </div>

            <div className="space-y-2">
              <Label>Key Rotation Policy</Label>
              <Select
                value={rotationMonths.toString()}
                onValueChange={v => setRotationMonths(parseInt(v))}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="3">Every 3 months</SelectItem>
                  <SelectItem value="6">Every 6 months</SelectItem>
                  <SelectItem value="12">Every 12 months</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="rounded-lg border bg-muted/30 p-4">
              <h4 className="mb-2 text-xs font-bold text-muted-foreground uppercase">
                Credential Info
              </h4>
              <div className="space-y-1 text-xs">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Expires:</span>
                  <span className="flex items-center gap-1.5 font-medium">
                    {credentials?.expiresAt != null ? (
                      <>
                        <Clock className={cn("size-3", isExpired ? "text-destructive" : "text-emerald-600")} />{" "}
                        {new Date(credentials.expiresAt).toLocaleDateString()}
                      </>
                    ) : (
                      "Never"
                    )}
                  </span>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <Button variant="outline" size="sm" onClick={onRenew} className="w-full">
                Renew Keys
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={onReset}
                className="w-full text-destructive hover:bg-destructive/5 hover:text-destructive">
                Reset All Keys
              </Button>
            </div>
          </div>

          <DialogFooter>
            <Button variant="ghost" onClick={() => setIsSettingsOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleUpdate}>Update Profile</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};

