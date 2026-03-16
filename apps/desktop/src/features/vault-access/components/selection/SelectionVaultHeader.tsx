import { OpenVaultLogo } from "@/components/icons";
import { Button } from "@/components/ui/shadcn/button";
import { hrefs } from "@/config/hrefs";
import { PlusIcon } from "lucide-react";
import Link from "next/link";

export const SelectionVaultHeader = () => (
  <div className="flex flex-col gap-8">
    <div className="flex items-center justify-between">
      <div className="space-y-4">
        <OpenVaultLogo />
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
