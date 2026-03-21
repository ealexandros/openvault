"use client";

import { hrefs } from "@/config/hrefs";
import { useVaultSession } from "@/context/vault-session";
import { matchRoutes } from "@/utils/routes";
import { usePathname, useRouter } from "next/navigation";
import { PropsWithChildren, useEffect } from "react";

const publicRoutes = [hrefs.home.get(), hrefs.create.get()];
const protectedRoutes = [hrefs.dashboard.get()];
const sharedRoutes: string[] = [];

export const useRouteGuard = () => {
  const pathname = usePathname();
  const router = useRouter();
  const { isUnlocked } = useVaultSession();

  useEffect(() => {
    const isPublic = matchRoutes(pathname, publicRoutes);
    const isShared = matchRoutes(pathname, sharedRoutes);

    const isProtected =
      matchRoutes(pathname, protectedRoutes) ||
      protectedRoutes.some(route => pathname.startsWith(route));

    if (isShared) return;

    if (!isUnlocked && isProtected) {
      router.replace(hrefs.home.get());
      return;
    }

    if (isUnlocked && isPublic) {
      router.replace(hrefs.dashboard.get());
      return;
    }
  }, [isUnlocked, pathname, router]);
};

export const RouteGuard = ({ children }: PropsWithChildren) => {
  useRouteGuard();
  return <>{children}</>;
};
