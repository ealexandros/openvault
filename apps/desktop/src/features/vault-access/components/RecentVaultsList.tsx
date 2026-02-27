import { cn } from "@/utils/cn";
import { useEffect, useState } from "react";
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
}: RecentVaultsListProps) => {
  const [isConfirming, setIsConfirming] = useState(false);

  useEffect(() => {
    if (isConfirming) {
      const timer = setTimeout(() => setIsConfirming(false), 2000);
      return () => clearTimeout(timer);
    }
  }, [isConfirming]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="h-4 w-1 rounded-full bg-primary/60" />
          <h3 className="text-xs font-bold tracking-wide text-muted-foreground uppercase">
            Recent Activity
          </h3>
        </div>
        {!Boolean(isLoading) && vaults.length > 0 && (
          <button
            onClick={() => {
              if (isConfirming) {
                onClear();
                setIsConfirming(false);
              } else {
                setIsConfirming(true);
              }
            }}
            className={cn(
              "cursor-pointer text-[10px] font-bold tracking-wider uppercase transition-all duration-300",
              isConfirming
                ? "scale-105 text-red-500"
                : "text-muted-foreground/40 hover:text-red-500",
            )}>
            {isConfirming ? "Are you sure?" : "Clear All"}
          </button>
        )}
      </div>

      {Boolean(isLoading) ? (
        <div className="grid gap-3">
          <RecentVaultSkeleton />
          <RecentVaultSkeleton />
          <RecentVaultSkeleton />
        </div>
      ) : (
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
      )}
    </div>
  );
};
