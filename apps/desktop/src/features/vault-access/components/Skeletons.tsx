import { Skeleton } from "@/components/ui/shadcn/skeleton";
import { SkeletonProps } from "@/types/general";

const RecentVaultSkeleton = () => (
  <div className="flex items-center justify-between rounded-xl border border-slate-200/60 bg-slate-50/50 p-3.5">
    <div className="flex min-w-0 items-center gap-4">
      <Skeleton className="size-11 shrink-0 rounded-lg" />
      <div className="space-y-2">
        <Skeleton className="h-4 w-24" />
        <Skeleton className="h-3 w-32" />
      </div>
    </div>
    <div className="flex items-center gap-4">
      <Skeleton className="size-4 rounded-full" />
    </div>
  </div>
);

export const RecentVaultListSkeleton = ({ children, isLoading }: SkeletonProps) => {
  if (!isLoading) return children;

  return (
    <div className="grid gap-3">
      <RecentVaultSkeleton />
      <RecentVaultSkeleton />
      <RecentVaultSkeleton />
    </div>
  );
};
