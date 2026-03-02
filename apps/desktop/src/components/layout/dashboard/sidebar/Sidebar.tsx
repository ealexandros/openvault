import { hrefs } from "@/config/hrefs";
import {
  ActivityIcon,
  FolderIcon,
  LockIcon,
  NotebookIcon,
  SettingsIcon,
  ShieldAlertIcon,
} from "lucide-react";
import { NavItem } from "./NavItem";
import { SidebarFooter } from "./SidebarFooter";
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
  <aside className="relative flex h-screen w-72 flex-col border-r border-border/40 bg-card/10 p-6 backdrop-blur-3xl">
    <SidebarHeader />

    <nav className="flex-1 space-y-2 py-4">
      {mainNavItems.map(item => (
        <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
      ))}
    </nav>

    <div className="space-y-3 pb-2">
      {bottomNavItems.map(item => (
        <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
      ))}
      <SidebarFooter vaultName={vaultName} onLogout={onLogout} />
    </div>

    <div className="pointer-events-none absolute inset-x-0 bottom-0 h-24 bg-linear-to-t from-background/40 to-transparent" />
  </aside>
);
