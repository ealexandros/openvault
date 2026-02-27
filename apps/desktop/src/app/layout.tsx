import { Toaster } from "@/components/ui/shadcn/sonner";
import { figtree, geistMono, geistSans } from "@/config/fonts";
import { globalMetadata } from "@/config/metadata";
import { VaultProvider } from "@/context/VaultContext";
import { cn } from "@/utils/cn";
import type { Metadata } from "next";
import { PropsWithChildren } from "react";

import "@/styles/animations.css";
import "@/styles/globals.css";

export const metadata: Metadata = globalMetadata;

const AppLayout = ({ children }: PropsWithChildren) => (
  <html lang="en" className={figtree.variable}>
    <body className={cn(geistSans.variable, geistMono.variable)}>
      <VaultProvider>{children}</VaultProvider>
      <Toaster position="bottom-right" />
    </body>
  </html>
);

export default AppLayout;
