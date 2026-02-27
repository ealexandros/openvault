import { cn } from "@/utils/cn";

type SkeletonProps = React.HTMLAttributes<HTMLDivElement>;

export const Skeleton = ({ className, ...props }: SkeletonProps) => (
  <div
    data-slot="skeleton"
    className={cn("animate-pulse rounded-md bg-muted", className)}
    {...props}
  />
);
