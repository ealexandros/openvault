import { Brand } from "@/components/icons";

type SidebarHeaderProps = {
  vaultName?: string;
};

export const SidebarHeader = ({ vaultName }: SidebarHeaderProps) => (
  <header className="flex flex-col gap-8 py-5">
    <Brand nameClassName="text-[22px]" />

    {vaultName != null && (
      <div className="relative flex items-center gap-3 rounded-lg border border-border bg-muted/30 p-3">
        <div className="relative flex size-9 shrink-0 items-center justify-center rounded-lg bg-emerald-500/10">
          <span className="relative size-2.5 animate-pulse rounded-full bg-emerald-500 duration-700" />
        </div>
        <div className="flex min-w-0 flex-col leading-tight">
          <span className="text-xs font-semibold text-muted-foreground/60 uppercase">
            Vault Name
          </span>
          <span className="truncate text-base font-semibold text-foreground/80">
            {vaultName}
          </span>
        </div>
      </div>
    )}
  </header>
);
