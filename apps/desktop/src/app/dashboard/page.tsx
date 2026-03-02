"use client";

import { Spinner } from "@/components/ui/shadcn/spinner";
import { hrefs } from "@/config/hrefs";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

const DashboardPage = () => {
  const router = useRouter();

  useEffect(() => router.replace(hrefs.dashboard.browse), [router]);

  return (
    <div className="flex h-full w-full items-center justify-center">
      <Spinner className="size-6" />
    </div>
  );
};

export default DashboardPage;
