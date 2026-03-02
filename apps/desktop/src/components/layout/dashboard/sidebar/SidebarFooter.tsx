import { Button } from "@/components/ui/shadcn/button";
import { env } from "@/config/env";
import { LogOutIcon } from "lucide-react";

type SidebarFooterProps = {
  vaultName?: string;
  onLogout: () => void;
};

export const SidebarFooter = ({ onLogout }: SidebarFooterProps) => (
  <footer className="mt-auto space-y-6">
    <Button
      variant="destructive"
      size="lg"
      onClick={onLogout}
      className="w-full justify-start gap-2 bg-destructive/5 px-4 py-5.5 text-base font-medium hover:bg-destructive/70 hover:text-white">
      <LogOutIcon />
      <span>Lock Vault</span>
    </Button>
    <div className="px-1 text-center text-xs font-semibold text-muted-foreground uppercase">
      OPENVAULT • V{env.VERSION}
    </div>
  </footer>
);
