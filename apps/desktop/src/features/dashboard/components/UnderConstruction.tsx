import { HistoryIcon } from "lucide-react";

type UnderConstructionProps = {
  title: string;
};

export const UnderConstruction = ({ title }: UnderConstructionProps) => (
  <div className="flex h-screen flex-col items-center justify-center space-y-6 pb-30 text-center">
    <div className="flex size-16 items-center justify-center rounded-full bg-muted">
      <HistoryIcon className="size-8 text-muted-foreground" />
    </div>
    <div className="space-y-1">
      <h3 className="text-lg font-medium">Under Construction</h3>
      <p className="text-sm text-muted-foreground">The {title} module is coming soon.</p>
    </div>
  </div>
);
