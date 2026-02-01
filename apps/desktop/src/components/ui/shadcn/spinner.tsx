import { cn } from "@/utils/cn";
import { Loader2Icon } from "lucide-react";

export const Spinner = ({ className, ...props }: React.ComponentProps<"svg">) => (
  <Loader2Icon
    role="status"
    aria-label="Loading"
    className={cn("size-4 animate-spin", className)}
    {...props}
  />
);
