import { ShieldIcon } from "lucide-react";

type SidebarHeaderProps = {
  vaultName?: string;
};

export const SidebarHeader = ({ vaultName }: SidebarHeaderProps) => (
  <header className="flex flex-col gap-8 py-5">
    <h1 className="flex cursor-default items-center gap-2 text-2xl font-semibold tracking-tight">
      <ShieldIcon className="size-8 text-primary" strokeWidth={2.5} />
      <span className="mt-0.5">OpenVault</span>
    </h1>

    {vaultName != null && (
      <div className="relative flex items-center gap-3 rounded-lg border border-border bg-muted/30 p-3">
        <div className="relative flex size-9 shrink-0 items-center justify-center rounded-lg bg-emerald-500/10">
          <span className="relative size-2.5 animate-pulse rounded-full bg-emerald-500 duration-700" />
        </div>
        <div className="flex min-w-0 flex-col leading-tight">
          <span className="text-xs font-semibold text-muted-foreground/60 uppercase">
            Authenticated
          </span>
          <span className="truncate text-base font-semibold text-foreground/80">
            {vaultName}
          </span>
        </div>
      </div>
    )}
  </header>
);
