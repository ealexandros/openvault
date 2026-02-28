"use client";

import { VaultProvider } from "@/context/VaultContext";
import { ThemeProvider } from "next-themes";

export const AppProvider = ({ children }: { children: React.ReactNode }) => (
  <ThemeProvider attribute="class" defaultTheme="light" enableSystem={false}>
    <VaultProvider>{children}</VaultProvider>
  </ThemeProvider>
);
