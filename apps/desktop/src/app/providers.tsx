"use client";

import { VaultProvider } from "@/context/VaultContext";
import { ReactQueryProvider } from "@/libraries/react-query";
import { ThemeProvider } from "next-themes";

export const AppProvider = ({ children }: { children: React.ReactNode }) => (
  <ReactQueryProvider>
    <ThemeProvider attribute="class" defaultTheme="light" enableSystem={false}>
      <VaultProvider>{children}</VaultProvider>
    </ThemeProvider>
  </ReactQueryProvider>
);
