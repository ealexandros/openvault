"use client";

import { DashboardLayout as BaseDashboardLayout } from "@/features/dashboard/layout";
import { ReactNode } from "react";

type DashboardLayoutProps = {
  children: ReactNode;
};

const DashboardLayout = ({ children }: DashboardLayoutProps) => (
  <BaseDashboardLayout>{children}</BaseDashboardLayout>
);

export default DashboardLayout;
