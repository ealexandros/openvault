import { Button } from "@/components/ui/shadcn/button";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";

export type EmptySearchResultProps = {
  searchQuery: string;
  onClearSearch: () => void;
};

export const EmptySearchResult = ({ searchQuery, onClearSearch }: EmptySearchResultProps) => (
  <Empty className="border-2 border-dashed border-muted py-52">
    <EmptyHeader>
      <EmptyTitle>No matches found</EmptyTitle>
      <EmptyDescription>
        Nothing matches <span>&ldquo;{searchQuery}&rdquo;</span>. Try another keyword.
      </EmptyDescription>
    </EmptyHeader>
    <EmptyContent className="flex-row justify-center gap-2">
      <Button variant="outline" onClick={onClearSearch}>
        Clear search
      </Button>
    </EmptyContent>
  </Empty>
);
