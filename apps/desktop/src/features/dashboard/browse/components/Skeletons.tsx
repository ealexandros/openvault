import { Skeleton } from "@/components/ui/shadcn/skeleton";
import { cn } from "@/utils/cn";
import { FileIcon, FolderIcon } from "lucide-react";
import { BrowseSection } from "./BrowseSection";

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

export const BrowseSkeleton = () => (
  <div className="space-y-10">
    <BrowseSection title="Folders" count={0} icon={FolderIcon}>
      <GridSkeleton count={2} itemHeight="h-24" />
    </BrowseSection>
    <BrowseSection title="Files" count={0} icon={FileIcon}>
      <GridSkeleton count={8} itemHeight="h-32" />
    </BrowseSection>
  </div>
);
