"use client";

import { DashboardLayout } from "@/components/layout/dashboard";
import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { useRouter } from "next/navigation";
import { ReactNode, useEffect } from "react";

type LayoutProps = {
  children: ReactNode;
};

const Layout = ({ children }: LayoutProps) => {
  const { vaultName, isUnlocked, lockVault } = useVault();
  const router = useRouter();

  useEffect(() => {
    if (!isUnlocked) {
      router.push(hrefs.home.get());
    }
  }, [isUnlocked, router]);

  if (!isUnlocked) return null;

  return (
    <DashboardLayout onLogout={lockVault} vaultName={vaultName}>
      {children}
    </DashboardLayout>
  );
};

export default Layout;
