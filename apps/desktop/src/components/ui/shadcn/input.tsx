import { cn } from "@/utils/cn";
import * as React from "react";

export const Input = ({ className, type, ...props }: React.ComponentProps<"input">) => (
  <input
    type={type}
    data-slot="input"
    className={cn(
      "h-13 w-full min-w-0 rounded-md border border-input bg-input/20 px-4 py-1 text-sm transition-colors outline-none file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-xs/relaxed file:font-medium file:text-foreground placeholder:text-muted-foreground/60 focus-visible:ring-2 focus-visible:ring-ring/15 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-2 aria-invalid:ring-destructive/20 dark:bg-input/30 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40",
      className,
    )}
    {...props}
  />
);
