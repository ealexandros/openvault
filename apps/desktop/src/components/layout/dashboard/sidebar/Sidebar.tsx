import { Brand } from "@/components/icons";
import { hrefs } from "@/config/hrefs";
import { cn } from "@/utils/cn";
import {
  ActivityIcon,
  FolderIcon,
  LockIcon,
  MessageCircle,
  NotebookIcon,
  ShieldAlertIcon,
} from "lucide-react";
import { SidebarFooter } from "./Footer";
import { NavItem } from "./NavItem";

type SidebarProps = {
  vaultName?: string;
  isCollapsed?: boolean;
  onLogout: () => Promise<void>;
};

const mainNavItems = [
  { href: hrefs.dashboard.browse.get(), label: "Browse Files", icon: FolderIcon },
  { href: hrefs.dashboard.messages.get(), label: "Messages", icon: MessageCircle },
  { href: hrefs.dashboard.passwords.get(), label: "Passwords", icon: LockIcon },
  { href: hrefs.dashboard.notes.get(), label: "Notes", icon: NotebookIcon },
  { href: hrefs.dashboard.logs.get(), label: "Activity Logs", icon: ActivityIcon },
  { href: hrefs.dashboard.decoy.get(), label: "Decoy Vault", icon: ShieldAlertIcon },
] as const;

export const DashboardSidebar = ({ vaultName, isCollapsed, onLogout }: SidebarProps) => (
  <aside
    className={cn(
      "relative flex h-screen flex-col border-r border-muted-foreground/10 bg-foreground/1 px-4 py-10 lg:px-6",
      isCollapsed === true ? "w-0 overflow-hidden border-r-0 p-0 opacity-0" : "w-64 lg:w-70",
    )}>
    <div className={cn("flex h-full flex-col gap-10", isCollapsed === true && "invisible")}>
      <header className="py-3">
        <Brand nameClassName="text-xl font-bold tracking-tight" />
      </header>

      <nav className="flex-1">
        {mainNavItems.map(item => (
          <NavItem key={item.href} href={item.href} label={item.label} icon={item.icon} />
        ))}
      </nav>

      <SidebarFooter vaultName={vaultName} onLogout={onLogout} />
    </div>
  </aside>
);
