export const UserListItemSkeleton = () => (
  <div className="w-full rounded-xl border border-transparent p-3">
    <div className="flex items-start gap-3">
      <div className="h-8 w-8 animate-pulse rounded-lg bg-muted" />
      <div className="flex-1 space-y-2">
        <div className="h-4 w-1/2 animate-pulse rounded bg-muted" />
        <div className="h-3 w-1/3 animate-pulse rounded bg-muted" />
      </div>
    </div>
  </div>
);
