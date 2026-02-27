import { cn } from "@/utils/cn";
import { History } from "lucide-react";
import { RecentVault } from "../hooks/useVaultAccess";
import { RecentVaultItem, RecentVaultSkeleton } from "./RecentVaultItem";

type RecentVaultsListProps = {
  vaults: RecentVault[];
  onConnect: (path: string) => void;
  onRemove: (id: string) => void;
  onClear: () => void;
  isLoading?: boolean;
};

export const RecentVaultsList = ({
  vaults,
  onConnect,
  onRemove,
  onClear,
  isLoading,
}: RecentVaultsListProps) => (
  <div className="space-y-6">
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-3">
        <div className="h-4 w-1 rounded-full bg-primary/70" />
        <h3 className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
          Recent Activity
        </h3>
      </div>
      {!Boolean(isLoading) && vaults.length > 0 && (
        <button
          onClick={onClear}
          className={cn(
            "cursor-pointer text-[10px] font-bold tracking-wider uppercase transition-all duration-300",
            "text-muted-foreground/40 hover:text-red-500",
          )}>
          Clear All
        </button>
      )}
    </div>

    {Boolean(isLoading) ? (
      <div className="grid gap-3">
        <RecentVaultSkeleton />
        <RecentVaultSkeleton />
        <RecentVaultSkeleton />
      </div>
    ) : vaults.length > 0 ? (
      <div className="grid gap-3">
        {vaults.map(vault => (
          <RecentVaultItem
            key={vault.id}
            vault={vault}
            onConnect={onConnect}
            onRemove={onRemove}
          />
        ))}
      </div>
    ) : (
      <div className="flex flex-col items-center justify-center space-y-2 rounded-2xl border border-slate-200/50 bg-slate-50/20 py-12 text-center">
        <History className="size-5 text-muted-foreground/50" />
        <p className="text-xs font-semibold text-muted-foreground/50">
          There are no recent vaults
        </p>
      </div>
    )}
  </div>
);
