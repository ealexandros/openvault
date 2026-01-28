import { Button } from "@/components/ui/shadcn/button";
import { hrefs } from "@/config/hrefs";
import { PlusIcon } from "lucide-react";
import { useRouter } from "next/navigation";

export const VaultHeader = () => {
  const router = useRouter();

  return (
    <div className="flex items-center justify-between">
      <h1 className="text-xl font-semibold tracking-tight">Vault</h1>
      <Button
        onClick={() => router.push(hrefs.setup.get())}
        variant="ghost"
        size="sm"
        className="rounded-lg text-xs text-muted-foreground hover:bg-muted hover:text-foreground">
        <PlusIcon className="mr-2 size-3.5" />
        New Vault
      </Button>
    </div>
  );
};
