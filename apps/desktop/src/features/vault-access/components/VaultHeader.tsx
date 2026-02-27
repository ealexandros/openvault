import { Button } from "@/components/ui/shadcn/button";
import { hrefs } from "@/config/hrefs";
import { PlusIcon, Shield } from "lucide-react";
import Link from "next/link";
import { VaultView } from "../hooks/useVaultAccess";

type VaultHeaderProps = {
  view: VaultView;
  path: string;
};

export const VaultHeader = ({ view, path }: VaultHeaderProps) => (
  <div className="flex flex-col gap-8">
    {view === "selection" ? (
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="flex items-center gap-2 text-3xl font-semibold">
            <Shield className="size-9 text-primary" strokeWidth={2.5} />
            <span className="mt-0.5">OpenVault</span>
          </h1>
          <p className="text-sm text-muted-foreground/60">
            Secure your data with geometric precision.
          </p>
        </div>
        <Button
          variant="outline"
          size="sm"
          className="gap-2 border-primary/20 bg-primary/5 p-4 text-xs font-semibold text-primary transition-all hover:bg-primary hover:text-primary-foreground"
          asChild>
          <Link href={hrefs.create.get()}>
            <PlusIcon className="size-4" />
            Create New Vault
          </Link>
        </Button>
      </div>
    ) : (
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
    )}
  </div>
);
