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
  <div
    className="flex h-screen w-full overflow-hidden bg-background"
    style={{ colorScheme: "dark" }}>
    <DashboardSidebar onLogout={onLogout} vaultName={vaultName} />
    <main className="flex min-w-0 flex-1 flex-col">
      <DashboardHeader />
      <div className="flex-1 overflow-auto p-8">{children}</div>
    </main>
  </div>
);
