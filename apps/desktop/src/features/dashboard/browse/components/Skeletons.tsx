export const FileGridSkeleton = () => (
  <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
    {Array.from({ length: 8 }).map((_, index) => (
      <div key={index} className="h-32 animate-pulse rounded-2xl border bg-muted/30" />
    ))}
  </div>
);

export const FolderGridSkeleton = () => (
  <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
    {Array.from({ length: 8 }).map((_, index) => (
      <div key={index} className="h-24 animate-pulse rounded-2xl border bg-muted/30" />
    ))}
  </div>
);
