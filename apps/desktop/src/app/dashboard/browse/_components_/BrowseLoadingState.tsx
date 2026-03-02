import { Spinner } from "@/components/ui/shadcn/spinner";

export const BrowseLoadingState = () => (
  <div className="rounded-2xl border p-6 animate-in fade-in duration-200">
    <div className="mb-6 flex items-center gap-2 text-sm text-muted-foreground">
      <Spinner className="size-4" />
      Syncing folder contents...
    </div>
    <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
      {Array.from({ length: 8 }).map((_, index) => (
        <div key={index} className="h-28 animate-pulse rounded-xl border bg-muted/30" />
      ))}
    </div>
  </div>
);
