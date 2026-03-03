import { Button } from "@/components/ui/shadcn/button";
import { cn } from "@/utils/cn";

export type EmptySearchResultProps = {
  searchQuery: string;
  onClearSearch: () => void;
};

export const EmptySearchResult = ({ searchQuery, onClearSearch }: EmptySearchResultProps) => (
  <div
    className={cn(
      "flex flex-col items-center gap-3 rounded-2xl border border-dashed px-8 py-16 text-center",
      "animate-in duration-300 fade-in slide-in-from-bottom-2",
    )}>
    <p className="text-base font-medium">No matches found</p>
    <p className="max-w-md text-sm text-muted-foreground">
      Nothing matches <span>&ldquo;{searchQuery}&rdquo;</span>. Try another keyword.
    </p>
    <Button variant="outline" onClick={onClearSearch}>
      Clear search
    </Button>
  </div>
);
