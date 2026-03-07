import { Button } from "@/components/ui/shadcn/button";
import { hrefs } from "@/config/hrefs";
import { PlusIcon, Shield } from "lucide-react";
import Link from "next/link";

export const SelectionVaultHeader = () => (
  <div className="flex flex-col gap-8">
    <div className="flex items-center justify-between">
      <div className="space-y-4">
        <h1 className="flex items-center gap-2 text-3xl font-semibold">
          <Shield className="size-9 text-primary" strokeWidth={2.5} />
          <span className="mt-0.5">OpenVault</span>
        </h1>
        <p className="text-base text-muted-foreground/60">
          Secure your data with geometric precision.
        </p>
      </div>
      <Button
        variant="outline"
        size="sm"
        className="gap-2 border-primary/20 bg-primary/5 p-4 py-4.5 text-sm font-semibold text-primary transition-all hover:bg-primary hover:text-primary-foreground"
        asChild>
        <Link href={hrefs.create.get()}>
          <PlusIcon className="size-4" />
          <span>Create New Vault</span>
        </Link>
      </Button>
    </div>
  </div>
);
