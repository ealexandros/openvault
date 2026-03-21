"use client";

import { BrandIcon } from "@/components/icons";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/shadcn/sidebar";
import { hrefs } from "@/config/hrefs";
import { VaultMetadata } from "@/context/vault-session";
import {
  AlertTriangle,
  ChartBarIcon,
  Code,
  DatabaseIcon,
  FileTextIcon,
  FolderIcon,
  MessageCircleIcon,
  ShieldAlertIcon,
} from "lucide-react";
import Link from "next/link";
import * as React from "react";
import { DashboardFooter } from "./Footer";
import { NavigationGroup } from "./navigation-group";
import { PinnedNavigation } from "./PinnedNavigation";
import { StorageIndicator } from "./StorageIndicator";

type DashboardSidebarProps = React.ComponentProps<typeof Sidebar> & {
  metadata: VaultMetadata | null;
  onLock: () => void;
};

export type NavigationItem = {
  title: string;
  url: string;
  blank?: boolean;
  icon?: React.ReactNode;
};

export const navigation = {
  features: [
    {
      title: "Browse Files",
      url: hrefs.dashboard.browse.get(),
      icon: <FolderIcon />,
    },
    {
      title: "Secrets",
      url: hrefs.dashboard.secrets.get(),
      icon: <DatabaseIcon />,
    },
    {
      title: "Notes",
      url: hrefs.dashboard.notes.get(),
      icon: <FileTextIcon />,
    },
    {
      title: "Messages",
      url: hrefs.dashboard.messages.get(),
      icon: <MessageCircleIcon />,
    },
    {
      title: "Activity Logs",
      url: hrefs.dashboard.logs.get(),
      icon: <ChartBarIcon />,
    },
    {
      title: "Decoy Vault",
      url: hrefs.dashboard.decoy.get(),
      icon: <ShieldAlertIcon />,
    },
  ],
  info: [
    {
      title: "Source Code",
      url: hrefs.github.get(),
      blank: true,
      icon: <Code />,
    },
    {
      title: "Report Issue",
      url: hrefs.github.issue.get(),
      blank: true,
      icon: <AlertTriangle />,
    },
  ],
} satisfies Record<string, NavigationItem[]>;

export const DashboardSidebar = ({ metadata, onLock, ...props }: DashboardSidebarProps) => (
  <Sidebar className="py-5" collapsible="offcanvas" {...props}>
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <Link href={hrefs.dashboard.home.get()}>
            <BrandIcon nameClassName="text-xl" logoClassName="size-7!" />
          </Link>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>
    <SidebarContent className="gap-2 pt-4">
      <PinnedNavigation items={navigation.features} />
      <NavigationGroup items={navigation.features} title="Features" pinnable={true} />
      <section className="mt-auto">
        <NavigationGroup items={navigation.info} />
      </section>
    </SidebarContent>
    <SidebarFooter className="gap-3">
      <StorageIndicator sizeInBytes={metadata?.sizeInBytes} />
      <DashboardFooter vaultName={metadata?.name} onLogout={onLock} />
    </SidebarFooter>
  </Sidebar>
);
