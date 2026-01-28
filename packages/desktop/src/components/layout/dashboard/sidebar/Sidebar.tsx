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
  { href: hrefs.dashboard.decoy, label: "Decoy Vault", icon: ShieldAlertIcon },
  { href: hrefs.dashboard.notes, label: "Notes", icon: NotebookIcon },
  { href: hrefs.dashboard.passwords, label: "Passwords", icon: LockIcon },
  { href: hrefs.dashboard.logs, label: "Logs", icon: ActivityIcon },
] as const;

const bottomNavItems = [
  { href: hrefs.dashboard.settings, label: "Settings", icon: SettingsIcon },
] as const;

export const DashboardSidebar = ({ onLogout, vaultName }: SidebarProps) => (
  <div className="flex h-screen w-64 flex-col border-r border-border bg-card/30 p-4 backdrop-blur-xl">
    <SidebarHeader vaultName={vaultName} />

    <nav className="flex-1 space-y-1">
      {mainNavItems.map(item => (
        <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
      ))}
    </nav>

    <div className="space-y-1 pb-4">
      {bottomNavItems.map(item => (
        <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
      ))}
    </div>

    <SidebarFooter onLogout={onLogout} />
  </div>
);
