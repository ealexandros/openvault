"use client";

import { FolderIcon, PlusIcon } from "lucide-react";

export const EmptyState = () => (
  <div className="col-span-full flex animate-in flex-col items-center justify-center space-y-6 py-32 text-center duration-500 fade-in slide-in-from-bottom-4">
    <div className="relative">
      <div className="absolute inset-0 scale-150 rounded-full bg-primary/10 blur-3xl" />
      <div className="relative flex h-24 w-24 items-center justify-center rounded-3xl border border-primary/20 bg-card/50">
        <FolderIcon className="size-12 text-primary/40" strokeWidth={1.5} />
        <div className="absolute -top-2 -right-2 flex size-8 items-center justify-center rounded-full border border-primary/50 bg-primary text-white">
          <PlusIcon className="size-4" />
        </div>
      </div>
    </div>
    <div className="space-y-2">
      <h3 className="text-xl font-bold tracking-tight text-foreground">
        Welcome to your vault
      </h3>
      <p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground">
        This folder is currently empty. Start by creating a new folder or uploading your
        important files to keep them secure.
      </p>
    </div>
  </div>
);
