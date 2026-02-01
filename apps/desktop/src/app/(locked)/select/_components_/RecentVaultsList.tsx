import { HistoryIcon } from "lucide-react";
import { RecentVaultItem, type RecentVault } from "./RecentVaultItem";

type RecentVaultsListProps = {
  vaults: RecentVault[];
  onConnect: (path: string) => void;
};

export const RecentVaultsList = ({ vaults, onConnect }: RecentVaultsListProps) => (
  <div className="space-y-4">
    <div className="flex items-center gap-2 px-1">
      <HistoryIcon className="h-3.5 w-3.5 text-muted-foreground" />
      <h3 className="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase">
        Recently opened
      </h3>
    </div>

    <div className="space-y-2">
      {vaults.map(vault => (
        <RecentVaultItem key={vault.id} vault={vault} onConnect={onConnect} />
      ))}
    </div>
  </div>
);
