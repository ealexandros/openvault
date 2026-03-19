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
              className="truncate text-[15px] font-semibold tracking-tight text-foreground/90">
              {vaultName}
            </span>
          </div>
          <MoreVertical className="size-4 shrink-0 text-muted-foreground/60" />
        </Button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        align="end"
        side="left"
        sideOffset={12}
        className="w-56 animate-in overflow-hidden">
        <DropdownMenuLabel className="flex items-center gap-2">
          <Badge variant="outline" className="size-9 rounded-md text-xs">
            {vaultName?.slice(0, 2).toUpperCase()}
          </Badge>
          <div className="flex flex-col">
            <span className="mb-1 text-[10px] leading-none font-bold tracking-widest text-muted-foreground/60 uppercase">
              Current Vault
            </span>
            <span className="truncate text-sm font-bold text-foreground">{vaultName}</span>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <Link href={hrefs.dashboard.settings.get()}>
          <DropdownMenuItem className="text-sm">
            <Settings className="size-4" />
            Settings
          </DropdownMenuItem>
        </Link>
        <DropdownMenuSeparator />
        <DropdownMenuItem
          onClick={onLogout}
          className="text-sm text-destructive! hover:bg-destructive/5!">
          <LogOut className="size-4" color="var(--color-destructive)" />
          Logout
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>

    <span className="mx-auto text-[10px] font-bold tracking-widest text-muted-foreground/60 uppercase">
      OPENVAULT • V{env.VERSION}
    </span>
  </section>
);
