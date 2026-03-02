import { env } from "@/config/env";
import { hrefs } from "@/config/hrefs";
import {
  ActivityIcon,
  FolderIcon,
  LockIcon,
  LogOut,
  NotebookIcon,
  SettingsIcon,
  ShieldAlertIcon,
} from "lucide-react";
import { NavItem } from "./NavItem";
import { SidebarHeader } from "./SidebarHeader";

type SidebarProps = {
  onLogout: () => void;
  vaultName?: string;
};

const mainNavItems = [
  { href: hrefs.dashboard.browse, label: "Browse Files", icon: FolderIcon },
  { href: hrefs.dashboard.passwords, label: "Passwords", icon: LockIcon },
  { href: hrefs.dashboard.notes, label: "Notes", icon: NotebookIcon },
  { href: hrefs.dashboard.logs, label: "Activity Logs", icon: ActivityIcon },
  { href: hrefs.dashboard.decoy, label: "Decoy Vault", icon: ShieldAlertIcon },
] as const;

const bottomNavItems = [
  { href: hrefs.dashboard.settings, label: "Settings", icon: SettingsIcon },
] as const;

export const DashboardSidebar = ({ onLogout, vaultName }: SidebarProps) => (
  <aside className="relative flex h-screen w-72 flex-col border-r border-muted-foreground/10 bg-foreground/1 p-6">
    <SidebarHeader />

    <nav className="flex-1 space-y-1.5 py-3">
      {mainNavItems.map(item => (
        <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
      ))}
    </nav>

    <div className="space-y-6 pb-2">
      <div className="space-y-1.5">
        {bottomNavItems.map(item => (
          <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
        ))}
        <NavItem
          href={hrefs.samesite.get()}
          label="Logout"
          icon={LogOut}
          onClick={onLogout}
          className="hover:text-destructive"
          iconClassName="group-hover:text-destructive"
        />
      </div>

      <div className="px-3 text-xs font-semibold text-muted-foreground/60 uppercase">
        OPENVAULT • V{env.VERSION}
      </div>
    </div>

    <div className="pointer-events-none absolute inset-x-0 bottom-0 h-24 bg-linear-to-t from-background/40 to-transparent" />
  </aside>
);
