import { hrefs } from "@/config/hrefs";
import { usePathname } from "next/navigation";
import { ReactNode, useState } from "react";
import { DashboardHeader } from "./header";
import { DashboardSidebar } from "./sidebar";

type DashboardLayoutProps = {
  children: ReactNode;
  onLogout: () => void;
  vaultName?: string;
  title?: string;
};

const routeTitleMap: Record<string, string> = {
  [hrefs.dashboard.browse]: "Browse Files",
  [hrefs.dashboard.passwords]: "Passwords",
  [hrefs.dashboard.notes]: "Notes",
  [hrefs.dashboard.logs]: "Activity Logs",
  [hrefs.dashboard.decoy]: "Decoy Vault",
  [hrefs.dashboard.settings]: "Settings",
};

export const DashboardLayout = ({
  children,
  onLogout,
  vaultName,
  title: explicitTitle,
}: DashboardLayoutProps) => {
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
  const pathname = usePathname();

  const title = explicitTitle ?? (pathname ? routeTitleMap[pathname] : undefined);

  return (
    <div className="flex h-screen w-full overflow-hidden bg-background">
      <DashboardSidebar
        onLogout={onLogout}
        vaultName={vaultName}
        isCollapsed={isSidebarCollapsed}
      />
      <main className="relative flex min-w-0 flex-1 flex-col overflow-hidden">
        <DashboardHeader
          title={title}
          onToggleSidebar={() => setIsSidebarCollapsed(!isSidebarCollapsed)}
        />
        <div className="scrollbar-thin scrollbar-track-transparent scrollbar-thumb-muted-foreground/10 hover:scrollbar-thumb-muted-foreground/20 flex-1 overflow-y-auto px-10 py-8">
          <div className="mx-auto h-full max-w-7xl">{children}</div>
        </div>
      </main>
    </div>
  );
};
