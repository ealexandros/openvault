import { Button } from "@/components/ui/shadcn/button";
import { hrefs } from "@/config/hrefs";
import { PlusIcon } from "lucide-react";
import { useRouter } from "next/navigation";

export const VaultHeader = () => {
  const router = useRouter();

  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold tracking-tight">OpenVault</h1>
          <p className="text-sm text-muted-foreground/60">
            Secure your data with geometric precision.
          </p>
        </div>
        <Button
          onClick={() => router.push(hrefs.setup.get())}
          variant="outline"
          size="sm"
          className="gap-2 rounded-xl border-primary/20 bg-primary/5 p-4 text-xs font-semibold text-primary transition-all hover:bg-primary hover:text-primary-foreground">
          <PlusIcon className="size-3.5" />
          Create New Vault
        </Button>
      </div>
    </div>
  );
};
