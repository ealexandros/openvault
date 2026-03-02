import { Empty, EmptyHeader } from "@/components/ui/shadcn/empty";
import { cn } from "@/utils/cn";
import { History } from "lucide-react";
import { RecentVault } from "../hooks/useVaultAccess";
import { RecentVaultItem } from "./RecentVaultItem";
import { RecentVaultListSkeleton } from "./Skeletons";

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
        <div className="h-6 w-1 rounded-full bg-primary/70" />
        <h3 className="text-sm font-bold tracking-wide text-muted-foreground uppercase">
          Recent Activity
        </h3>
      </div>
      {!Boolean(isLoading) && vaults.length > 0 && (
        <button
          onClick={onClear}
          className={cn(
            "cursor-pointer text-xs font-bold tracking-wider uppercase transition-all duration-300",
            "text-muted-foreground/40 hover:text-destructive",
          )}>
          Clear All
        </button>
      )}
    </div>

    <RecentVaultListSkeleton isLoading={Boolean(isLoading)}>
      {vaults.length > 0 ? (
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
        <Empty className="border-2 border-muted-foreground/10 py-12">
          <EmptyHeader>
            <History className="size-6 text-muted-foreground/50" />
            <p className="text-sm font-semibold text-muted-foreground/50">
              There are no recent vaults
            </p>
          </EmptyHeader>
        </Empty>
      )}
    </RecentVaultListSkeleton>
  </div>
);
