"use client";

import { Spinner } from "@/components/ui/shadcn/spinner";
import { hrefs } from "@/config/hrefs";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

const DashboardPage = () => {
  const router = useRouter();

  useEffect(() => router.replace(hrefs.dashboard.browse), []);

  return (
    <div className="flex h-screen w-64 flex-col border-r border-border bg-card/30 p-4 backdrop-blur-xl">
      <Spinner />
    </div>
  );
};

export default DashboardPage;
