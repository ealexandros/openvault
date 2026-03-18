import { Badge } from "@/components/ui/shadcn/badge";
import { Button } from "@/components/ui/shadcn/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { env } from "@/config/env";
import { hrefs } from "@/config/hrefs";
import { LogOut, MoreVertical, Settings } from "lucide-react";
import Link from "next/link";

type SidebarFooterProps = {
  vaultName?: string;
  onLogout: () => Promise<void>;
};

export const SidebarFooter = ({ vaultName, onLogout }: SidebarFooterProps) => (
  <section className="flex flex-col gap-4">
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          className="group h-auto w-full gap-2 rounded-xl border-none py-2 focus-visible:ring-0">
          <Badge variant="outline" className="size-10 rounded-md text-sm">
            {vaultName?.slice(0, 2).toUpperCase()}
          </Badge>
          <div className="flex flex-1 flex-col items-start gap-0.5 overflow-hidden leading-tight">
            <span className="text-xs font-bold text-muted-foreground/60 uppercase">
              Current Vault
            </span>
            <span
              title={vaultName}
              className="truncate text-[15px] font-semibold tracking-tight text-foreground/90 group-hover:text-foreground">
              {vaultName}
            </span>
          </div>
          <MoreVertical className="size-4 shrink-0 text-muted-foreground/60 transition-colors group-hover:text-muted-foreground/80" />
        </Button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        align="end"
        side="left"
        sideOffset={12}
        className="w-56 animate-in overflow-hidden">
        <DropdownMenuLabel className="px-3 py-2.5">
          <div className="flex flex-col gap-0.5">
            <span className="mb-1 text-[10px] leading-none font-bold tracking-widest text-muted-foreground/60 uppercase">
              Current Vault
            </span>
            <span className="truncate text-sm font-bold text-foreground">{vaultName}</span>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator className="bg-white/5" />
        <Link href={hrefs.dashboard.settings.get()}>
          <DropdownMenuItem className="group/item flex cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors focus:bg-white/5 focus:text-white">
            <Settings className="size-4 text-muted-foreground/70 transition-colors group-hover/item:text-foreground" />
            Settings
          </DropdownMenuItem>
        </Link>
        <DropdownMenuSeparator className="bg-white/5" />
        <DropdownMenuItem
          onClick={onLogout}
          className="group/logout flex cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium text-red-500/80 transition-colors focus:bg-red-500/10 focus:text-red-500">
          <LogOut className="size-4 text-red-500/60 transition-colors group-hover/logout:text-red-500" />
          Logout
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>

    <span className="mx-auto text-[10px] font-bold tracking-widest text-muted-foreground/60 uppercase">
      OPENVAULT • V{env.VERSION}
    </span>
  </section>
);
