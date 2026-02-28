import { LockKeyholeIcon } from "lucide-react";

type UnlockedVaultHeaderProps = {
  path: string;
};

export const UnlockedVaultHeader = ({ path }: UnlockedVaultHeaderProps) => {
  const filename = path.split(/[/\\]/).pop() ?? "";
  const parsedName = filename.replace(/\.[^/.]+$/, "");
  const vaultName = parsedName !== "" ? parsedName : "Encrypted Vault";

  return (
    <div className="flex flex-col items-center space-y-6 text-center">
      <div className="relative">
        <div className="absolute inset-0 scale-150 rounded-full bg-primary/8 blur-xl" />
        <div className="relative flex size-16 items-center justify-center rounded-xl border border-slate-200/50 bg-white ring-1 ring-slate-100">
          <LockKeyholeIcon strokeWidth={2} className="size-7 text-primary/80" />
        </div>
      </div>

      <div className="space-y-2">
        <h2 className="text-2xl font-bold tracking-tight text-slate-900">{vaultName}</h2>
        <div className="flex items-center justify-center gap-2">
          <span className="flex size-1.5 rounded-full bg-slate-300" />
          <p className="max-w-[280px] truncate text-[11px] font-medium tracking-wide text-slate-400/80">
            {path}
          </p>
          <span className="flex size-1.5 rounded-full bg-slate-300" />
        </div>
      </div>
    </div>
  );
};
