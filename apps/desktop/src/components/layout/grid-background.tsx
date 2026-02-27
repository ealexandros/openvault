"use client";

import { cn } from "@/utils/cn";

type GridBackgroundProps = {
  className?: string;
  children?: React.ReactNode;
};

export const GridBackground = ({ className, children }: GridBackgroundProps) => (
  <div
    className={cn(
      "relative flex min-h-screen w-full flex-col items-center justify-center overflow-hidden bg-background",
      className,
    )}>
    <div className="absolute inset-x-0 top-0 h-full w-full bg-[radial-gradient(circle_at_center,transparent_0%,var(--background)_100%)]" />
    <div className="relative z-10 w-full">{children}</div>
  </div>
);
