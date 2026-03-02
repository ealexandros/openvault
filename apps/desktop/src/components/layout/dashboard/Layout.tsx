"use client";

import { ReactNode } from "react";
import { DashboardHeader } from "./header";
import { DashboardSidebar } from "./sidebar";

type DashboardLayoutProps = {
  children: ReactNode;
  onLogout: () => void;
  vaultName?: string;
};

export const DashboardLayout = ({ children, onLogout, vaultName }: DashboardLayoutProps) => (
  <div className="flex h-screen w-full overflow-hidden bg-background">
    <DashboardSidebar onLogout={onLogout} vaultName={vaultName} />
    <main className="relative flex min-w-0 flex-1 flex-col overflow-hidden">
      <DashboardHeader />
      <div className="scrollbar-thin scrollbar-track-transparent scrollbar-thumb-muted-foreground/10 hover:scrollbar-thumb-muted-foreground/20 flex-1 overflow-y-auto px-10 py-8">
        <div className="mx-auto h-full max-w-7xl">{children}</div>
      </div>
    </main>
  </div>
);
