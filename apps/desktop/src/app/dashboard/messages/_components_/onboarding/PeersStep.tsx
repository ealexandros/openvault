import { Button } from "@/components/ui/shadcn/button";
import { UserPlus, Users } from "lucide-react";

export function PeersStep({ openImportPicker }: { openImportPicker: () => void }) {
  return (
    <div className="space-y-8 text-center">
      <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
        <Users className="h-10 w-10" />
      </div>

      <div>
        <h3 className="text-3xl font-bold tracking-tight">Connect with Peers</h3>

        <p className="text-muted-foreground">
          Import public keys to start messaging securely.
        </p>
      </div>

      <Button
        variant="outline"
        onClick={openImportPicker}
        className="h-20 w-full gap-4 border-dashed">
        <UserPlus className="size-6" />
        Import peer identities
      </Button>
    </div>
  );
}
