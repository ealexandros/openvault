"use client";

import { Badge } from "@/components/ui/shadcn/badge";
import { FolderIcon } from "lucide-react";

export type RecentVault = {
  id: string;
  name: string;
  path: string;
  lastAccessed: string;
  isEncrypted: boolean;
};

type RecentVaultItemProps = {
  vault: RecentVault;
  onConnect: (path: string) => void;
};

export const RecentVaultItem = ({ vault, onConnect }: RecentVaultItemProps) => (
  <div
    onClick={() => onConnect(vault.path)}
    className="group flex cursor-pointer items-center justify-between rounded-xl border border-border bg-muted/30 p-4 transition-all hover:border-primary/30">
    <div className="flex min-w-0 items-center gap-4">
      <div className="rounded-lg border border-border bg-background p-2 shadow-sm">
        <FolderIcon className="h-4 w-4 text-muted-foreground transition-colors group-hover:text-primary" />
      </div>
      <div className="min-w-0">
        <h4 className="truncate text-sm font-medium">{vault.name}</h4>
        <p className="mt-0.5 truncate text-[10px] text-muted-foreground">{vault.path}</p>
      </div>
    </div>
    <div className="flex shrink-0 items-center gap-3">
      <span className="hidden text-[10px] text-muted-foreground sm:block">
        {vault.lastAccessed}
      </span>
      {vault.isEncrypted ? (
        <Badge
          variant="outline"
          className="h-5 border-primary/30 bg-primary/5 px-1.5 text-[9px] font-medium text-primary">
          SECURE
        </Badge>
      ) : (
        <Badge
          variant="outline"
          className="h-5 border-border bg-transparent px-1.5 text-[9px] font-medium text-muted-foreground">
          OPEN
        </Badge>
      )}
    </div>
  </div>
);
