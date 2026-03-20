"use client";

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { FileItemResult } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { FileType } from "@/utils/mime-types";
import { FileAudioIcon, FileTextIcon, FileVideoIcon, ImageIcon } from "lucide-react";
import { useFileViewerDialog } from "./useFileViewerDialog";
import {
  AudioViewer,
  ImageViewer,
  PdfViewer,
  TextViewer,
  VideoViewer,
  ViewerLoading,
} from "./viewer";

type FileViewerDialogProps = {
  isOpen: boolean;
  item: FileItemResult | null;
  onOpenChange: (open: boolean) => void;
};

const IconMap: Record<NonNullable<FileType>, React.ElementType> = {
  image: ImageIcon,
  video: FileVideoIcon,
  audio: FileAudioIcon,
  pdf: FileTextIcon,
  text: FileTextIcon,
};

export const FilePreviewDialog = ({ isOpen, item, onOpenChange }: FileViewerDialogProps) => {
  const { fileType, contentUri, codeRef, isLoading } = useFileViewerDialog({ item });

  if (item == null) {
    return null;
  }

  const viewers: Record<NonNullable<FileType>, React.ReactNode> = {
    image: <ImageViewer url={contentUri} alt={item.name} />,
    pdf: <PdfViewer url={contentUri} />,
    audio: <AudioViewer url={contentUri} fileName={item.name} />,
    video: <VideoViewer url={contentUri} fileName={item.name} />,
    text: <TextViewer codeRef={codeRef} />,
  };

  const Icon = IconMap[fileType];

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[90vh] w-[90vw]! max-w-5xl! flex-col gap-0 overflow-hidden p-0">
        <DialogHeader className="flex items-start justify-between border-b border-border/40 bg-muted/20 px-6 py-4">
          <div className="flex items-center gap-3">
            <div className="flex size-10 items-center justify-center rounded-xl border border-border/50 bg-background shadow-xs">
              <Icon className="size-5 text-muted-foreground" />
            </div>
            <div>
              <DialogTitle className="text-base font-semibold tracking-tight">
                {item.name}
              </DialogTitle>
              <p className="mt-1.5 text-xs font-medium tracking-wide text-muted-foreground uppercase">
                {item.extension} FILE
              </p>
            </div>
          </div>
        </DialogHeader>

        <div
          className={cn(
            "relative flex w-full overflow-hidden",
            "h-[50vh] flex-col items-center justify-center sm:h-[60vh] md:h-[70vh]",
          )}>
          {viewers[fileType]}

          {isLoading && (
            <div className="absolute inset-0 z-50 flex flex-col items-center justify-center bg-background/80 backdrop-blur-sm">
              <ViewerLoading />
            </div>
          )}
        </div>
      </DialogContent>
      <DialogDescription className="sr-only">{item.name}</DialogDescription>
    </Dialog>
  );
};
