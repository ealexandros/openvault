"use client";

import { Separator } from "@/components/ui/shadcn/separator";
import { SidebarTrigger } from "@/components/ui/shadcn/sidebar";

type DashboardHeaderProps = {
  title?: string;
};

export const DashboardHeader = ({ title }: DashboardHeaderProps) => (
  <header className="border-b border-gray-100 py-4">
    <div className="flex w-full items-center gap-1 px-4 lg:gap-2 lg:px-6">
      <SidebarTrigger />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <h1 className="text-base font-medium">{title}</h1>
    </div>
  </header>
);
