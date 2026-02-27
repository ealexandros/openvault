import { Shield } from "lucide-react";

type UnlockedVaultHeaderProps = {
  path: string;
};

export const UnlockedVaultHeader = ({ path }: UnlockedVaultHeaderProps) => (
  <div className="space-y-7 text-center">
    <div className="mx-auto flex size-20 items-center justify-center rounded-2xl bg-white ring-1 ring-slate-200/50">
      <Shield strokeWidth={2.5} className="size-9 text-primary" />
    </div>

    <div className="space-y-4">
      <div className="flex items-center justify-center gap-2 font-medium">
        <div className="h-px w-6 bg-slate-200" />
        <p className="text-[10px] tracking-[0.2em] text-slate-400 uppercase">
          Encrypted Vault
        </p>
        <div className="h-px w-6 bg-slate-200" />
      </div>
      <p className="text-sm tracking-tight text-muted-foreground">{path}</p>
    </div>
  </div>
);
