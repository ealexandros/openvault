import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";
import { InboxIcon } from "lucide-react";
import { RecentVaultItem, type RecentVault } from "./RecentVaultItem";

type RecentVaultsListProps = {
  vaults: RecentVault[];
  onConnect: (path: string) => void;
};

export const RecentVaultsList = ({ vaults, onConnect }: RecentVaultsListProps) => (
  <div className="space-y-6">
    <div className="flex items-center gap-3">
      <div className="h-4 w-1 rounded-full bg-primary/60" />
      <h3 className="text-xs font-bold tracking-wide text-muted-foreground uppercase">
        Recent Activity
      </h3>
    </div>

    {vaults.length > 0 ? (
      <div className="grid gap-3">
        {vaults.map(vault => (
          <RecentVaultItem key={vault.id} vault={vault} onConnect={onConnect} />
        ))}
      </div>
    ) : (
      <Empty className="border-none bg-gray-50/80 py-16">
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <InboxIcon className="size-4" />
          </EmptyMedia>
          <EmptyTitle>No recent files</EmptyTitle>
          <EmptyDescription>
            Your recently accessed files will appear here for quick access.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )}
  </div>
);
