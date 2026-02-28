import { Toaster } from "@/components/ui/shadcn/sonner";
import { figtree, geistMono, geistSans } from "@/config/fonts";
import { globalMetadata } from "@/config/metadata";
import { cn } from "@/utils/cn";
import type { Metadata } from "next";
import { PropsWithChildren } from "react";
import { AppProvider } from "./providers";

import "@/styles/animations.css";
import "@/styles/globals.css";

export const metadata: Metadata = globalMetadata;

const AppLayout = ({ children }: PropsWithChildren) => (
  <html lang="en" className={figtree.variable}>
    <body className={cn(geistSans.variable, geistMono.variable)}>
      <Toaster position="bottom-right" />
      <AppProvider>{children}</AppProvider>
    </body>
  </html>
);

export default AppLayout;
