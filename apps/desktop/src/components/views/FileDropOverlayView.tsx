"use client";

import { UploadCloudIcon } from "lucide-react";

type FileDropOverlayViewProps = {
  isVisible: boolean;
};

export const FileDropOverlayView = ({ isVisible }: FileDropOverlayViewProps) => {
  if (!isVisible) return null;

  return (
    <div className="fixed inset-0 z-50 flex animate-in items-center justify-center bg-background/80 duration-200 fade-in">
      <div className="flex animate-in flex-col items-center gap-4 rounded-2xl border bg-background px-12 py-14 text-center shadow-lg duration-200 zoom-in-95 slide-in-from-bottom-2">
        <UploadCloudIcon className="size-12 animate-bounce text-muted-foreground" />
        <div>
          <h2 className="text-xl font-semibold">Drop files to upload</h2>
          <p className="text-sm text-muted-foreground">Release to securely add them</p>
        </div>
      </div>
    </div>
  );
};
