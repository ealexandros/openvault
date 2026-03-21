import { SidebarInset, SidebarProvider } from "@/components/ui/shadcn/sidebar";
import { hrefs } from "@/config/hrefs";
import { useVaultSession } from "@/context/vault-session";
import { usePathname } from "next/navigation";
import { PropsWithChildren } from "react";
import { DashboardHeader } from "./Header";
import { DashboardSidebar } from "./sidebar/Sidebar";

const routeTitleMap: Record<string, string> = {
  [hrefs.dashboard.browse.get()]: "Secure Files & Folders",
  [hrefs.dashboard.secrets.get()]: "Secure Passwords & Credentials",
  [hrefs.dashboard.notes.get()]: "Manage Secret Notes",
  [hrefs.dashboard.messages.get()]: "Communicate Securely (E2EE)",
  [hrefs.dashboard.logs.get()]: "Monitor Activity Logs",
  [hrefs.dashboard.decoy.get()]: "Decoy Vault",
  [hrefs.dashboard.settings.get()]: "Manage Settings",
};

const sidebarStyle = {
  "--sidebar-width": "calc(var(--spacing) * 72)",
  "--header-height": "calc(var(--spacing) * 12)",
} as React.CSSProperties;

export const DashboardLayout = ({ children }: PropsWithChildren) => {
  const { metadata, lockVault } = useVaultSession();
  const pathname = usePathname();

  const title = routeTitleMap[pathname] ?? metadata?.name;

  return (
    <SidebarProvider style={sidebarStyle}>
      <DashboardSidebar variant="inset" metadata={metadata} onLock={lockVault} />
      <SidebarInset>
        <DashboardHeader title={title} />
        <section className="flex flex-1 flex-col">{children}</section>
      </SidebarInset>
    </SidebarProvider>
  );
};
