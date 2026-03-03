import { Skeleton } from "@/components/ui/shadcn/skeleton";
import { cn } from "@/utils/cn";

type GridSkeletonProps = {
  count: number;
  itemHeight: string;
};

const GridSkeleton = ({ count, itemHeight }: GridSkeletonProps) => (
  <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
    {Array.from({ length: count }).map((_, index) => (
      <Skeleton key={index} className={cn("rounded-2xl border", itemHeight)} />
    ))}
  </div>
);

export const FileGridSkeleton = () => <GridSkeleton count={8} itemHeight="h-32" />;

export const FolderGridSkeleton = () => <GridSkeleton count={2} itemHeight="h-24" />;
