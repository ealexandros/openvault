"use client";

import { VaultProvider } from "@/context/vault-session";
import { usePreventBackspaceNavigation } from "@/hooks/usePreventBackspaceNavigation";
import { ReactQueryProvider } from "@/libraries/react-query";
import { ThemeProvider } from "next-themes";

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const _ = usePreventBackspaceNavigation();

  return (
    <ReactQueryProvider>
      <ThemeProvider attribute="class" defaultTheme="light" enableSystem={false}>
        <VaultProvider>{children}</VaultProvider>
      </ThemeProvider>
    </ReactQueryProvider>
  );
};
