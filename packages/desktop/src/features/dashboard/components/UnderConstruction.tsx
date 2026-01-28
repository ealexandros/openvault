"use client";

import { HistoryIcon } from "lucide-react";

type UnderConstructionProps = {
  title: string;
};

export const UnderConstruction = ({ title }: UnderConstructionProps) => (
  <div className="flex flex-col items-center justify-center space-y-4 py-20 text-center">
    <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
      <HistoryIcon className="size-8 text-muted-foreground" />
    </div>
    <div className="space-y-1">
      <h3 className="text-base font-medium">Under Construction</h3>
      <p className="text-sm text-muted-foreground">The {title} module is coming soon.</p>
    </div>
  </div>
);
