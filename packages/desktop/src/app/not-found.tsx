"use client";

import { Spinner } from "@/components/ui/shadcn/spinner";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

const NotFound = () => {
  const router = useRouter();

  useEffect(() => router.replace("/"), [router]);

  return (
    <div className="flex h-screen w-64 flex-col border-r border-border bg-card/30 p-4 backdrop-blur-xl">
      <Spinner />
    </div>
  );
};

export default NotFound;
