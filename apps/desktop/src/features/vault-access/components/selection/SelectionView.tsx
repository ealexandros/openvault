import { useSelectVault } from "../../hooks/useSelectVault";
import { ActionSection } from "./ActionSection";
import { RecentVaultsList } from "./RecentVaultsList";
import { SelectionVaultHeader } from "./SelectionVaultHeader";

type SelectionViewProps = {
  onConnect: (path: string) => void;
};

export const SelectionView = ({ onConnect }: SelectionViewProps) => {
  const {
    recentVaults,
    isLoadingVaults,
    handleSelect,
    handleRemoveRecent,
    handleClearRecent,
  } = useSelectVault(onConnect);

  return (
    <div className="space-y-12">
      <SelectionVaultHeader />
      <ActionSection onBrowse={handleSelect} />
      <RecentVaultsList
        vaults={recentVaults}
        onConnect={onConnect}
        onRemove={handleRemoveRecent}
        onClear={handleClearRecent}
        isLoading={isLoadingVaults}
      />
    </div>
  );
};
