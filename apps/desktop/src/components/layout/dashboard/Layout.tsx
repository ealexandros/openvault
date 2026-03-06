import { hrefs } from "@/config/hrefs";
import { usePathname } from "next/navigation";
import { ReactNode, useState } from "react";
import { DashboardHeader } from "./header";
import { DashboardSidebar } from "./sidebar";

type DashboardLayoutProps = {
  children: ReactNode;
  vaultName?: string;
  title?: string;
  onLogout: () => Promise<void>;
};

const routeTitleMap: Record<string, string> = {
  [hrefs.dashboard.browse.get()]: "Browse Files",
  [hrefs.dashboard.passwords.get()]: "Passwords",
  [hrefs.dashboard.notes.get()]: "Notes",
  [hrefs.dashboard.messages.get()]: "Messages",
  [hrefs.dashboard.logs.get()]: "Activity Logs",
  [hrefs.dashboard.decoy.get()]: "Decoy Vault",
  [hrefs.dashboard.settings.get()]: "Settings",
};

export const DashboardLayout = ({
  children,
  vaultName,
  title: explicitTitle,
  onLogout,
}: DashboardLayoutProps) => {
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
  const pathname = usePathname();

  const title = explicitTitle ?? (pathname ? routeTitleMap[pathname] : undefined);

  return (
    <div className="flex h-screen w-full overflow-hidden bg-background">
      <DashboardSidebar
        vaultName={vaultName}
        isCollapsed={isSidebarCollapsed}
        onLogout={onLogout}
      />
      <main className="relative flex min-w-0 flex-1 flex-col overflow-hidden">
        <DashboardHeader
          title={title}
          onToggleSidebar={() => setIsSidebarCollapsed(!isSidebarCollapsed)}
        />
        <div className="flex-1 overflow-y-auto">
          <div className="mx-auto h-full">{children}</div>
        </div>
      </main>
    </div>
  );
};
