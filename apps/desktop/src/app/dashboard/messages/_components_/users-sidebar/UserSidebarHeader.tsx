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
import { Clock, Settings } from "lucide-react";
import { useState } from "react";

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
};

export const UserSidebarHeader = ({
  credentials,
  onRenew,
  onReset,
  onUpdate,
}: UserSidebarHeaderProps) => {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [name, setName] = useState(credentials?.name ?? "");
  const [rotationMonths, setRotationMonths] = useState(12);

  const handleUpdate = async () => {
    await onUpdate({ name, rotationMonths });
    setIsSettingsOpen(false);
  };

  return (
    <div className="flex flex-col gap-4 border-b border-border bg-muted/20 p-6">
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <h3 className="text-lg font-semibold tracking-tight">User Profiles</h3>
          <p className="text-xs text-muted-foreground">Manage identities and keys.</p>
        </div>

        <Button
          variant="ghost"
          size="icon"
          onClick={() => setIsSettingsOpen(true)}
          className="h-8 w-8 text-muted-foreground">
          <Settings className="h-4 w-4" />
        </Button>
      </div>

      {credentials?.expiresAt != null && (
        <div className="flex items-center gap-1.5 text-[10px] font-bold text-amber-600/80 uppercase">
          <Clock className="h-2.5 w-2.5" />
          <span>
            Key expires:{" "}
            {new Date(credentials.expiresAt).toLocaleDateString()}
          </span>
        </div>
      )}

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
                  <span className="font-medium">
                    {credentials?.expiresAt != null
                      ? new Date(credentials.expiresAt).toLocaleDateString()
                      : "Never"}
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
    </div>
  );
};
