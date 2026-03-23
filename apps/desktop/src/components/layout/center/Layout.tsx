import { PropsWithClassName } from "@/types/react";
import { cn } from "@/utils/cn";
import { PropsWithChildren } from "react";

export const CenterLayout = ({
  children,
  className,
}: PropsWithChildren<PropsWithClassName>) => (
  <div
    className={cn(
      "flex h-screen w-full items-center justify-center overflow-hidden bg-background",
      className,
    )}>
    {children}
  </div>
);
