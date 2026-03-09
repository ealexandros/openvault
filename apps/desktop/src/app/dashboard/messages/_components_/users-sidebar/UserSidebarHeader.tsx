import { Clock } from "lucide-react";

type UserSidebarHeaderProps = {
  keyExpiresAt: string;
};

export const UserSidebarHeader = ({ keyExpiresAt }: UserSidebarHeaderProps) => (
  <div className="flex flex-col gap-4 border-b border-border bg-muted/20 p-6">
    <div className="space-y-1">
      <h3 className="text-lg font-semibold tracking-tight">User Profiles</h3>

      <p className="text-xs text-muted-foreground">
        Manage public keys and identities for secure messaging.
      </p>

      {keyExpiresAt && (
        <div className="flex items-center gap-1.5 text-[10px] font-bold text-amber-600/80 uppercase">
          <Clock className="h-2.5 w-2.5" />
          <span>Key expires: {new Date(keyExpiresAt).toLocaleDateString()}</span>
        </div>
      )}
    </div>
  </div>
);
