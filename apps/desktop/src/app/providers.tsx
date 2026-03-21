"use client";

import { VaultProvider } from "@/context/vault-session";
import { RouteGuard, usePreventBackspaceNavigation } from "@/features/providers";
import { ReactQueryProvider } from "@/libraries/react-query";
import { ThemeProvider } from "next-themes";

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  usePreventBackspaceNavigation();

  return (
    <VaultProvider>
      <ReactQueryProvider>
        <ThemeProvider attribute="class" defaultTheme="light" enableSystem={false}>
          <RouteGuard>{children}</RouteGuard>
        </ThemeProvider>
      </ReactQueryProvider>
    </VaultProvider>
  );
};
