"use client";

import { UploadCloudIcon } from "lucide-react";

type BrowseDropOverlayProps = {
  isVisible: boolean;
};

export const BrowseDropOverlay = ({ isVisible }: BrowseDropOverlayProps) => {
  if (!isVisible) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-background/80 animate-in fade-in duration-200">
      <div className="flex flex-col items-center gap-4 rounded-2xl border bg-background px-12 py-14 text-center shadow-lg animate-in zoom-in-95 slide-in-from-bottom-2 duration-200">
        <UploadCloudIcon className="size-12 text-muted-foreground" />
        <div>
          <h2 className="text-xl font-semibold">Drop files to upload</h2>
          <p className="text-sm text-muted-foreground">Release to securely add them</p>
        </div>
      </div>
    </div>
  );
};
