import { FolderIcon } from "lucide-react";

type SidebarHeaderProps = {
  vaultName?: string;
};

export const SidebarHeader = ({ vaultName }: SidebarHeaderProps) => (
  <div className="mb-4 px-2 py-6">
    <div className="flex items-center gap-3">
      <div className="flex h-10 w-10 items-center justify-center rounded-xl border border-primary/20 bg-primary/10">
        <FolderIcon className="h-5 w-5 text-primary" />
      </div>
      <div className="min-w-0">
        <h2 className="truncate text-sm font-semibold text-foreground">
          {vaultName ?? "OpenVault"}
        </h2>
        <p className="text-[10px] font-medium tracking-wider text-emerald-500 uppercase">
          Securely Unlocked
        </p>
      </div>
    </div>
  </div>
);
