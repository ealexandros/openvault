import { env } from "@/config/env";
import { LogOutIcon } from "lucide-react";

type SidebarFooterProps = {
  onLogout: () => void;
};

export const SidebarFooter = ({ onLogout }: SidebarFooterProps) => (
  <div className="mt-auto space-y-6 border-t border-border/50 pt-4">
    <button
      onClick={onLogout}
      className="flex w-full cursor-pointer items-center gap-3 rounded-xl px-3 py-2.5 text-muted-foreground transition-all hover:bg-destructive/5 hover:text-destructive">
      <LogOutIcon className="size-4 transition-transform" />
      <span className="text-sm font-medium">Lock Vault</span>
    </button>
    <div className="text-center text-[10px] leading-relaxed text-muted-foreground/40">
      OpenVault v{env.VERSION}
      <br />
      Secure & Private
    </div>
  </div>
);
