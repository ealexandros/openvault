import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { cn } from "@/utils/cn";
import { FileAudioIcon, FileIcon, FileTextIcon, FileVideoIcon, ImageIcon } from "lucide-react";
import { AudioViewer } from "./AudioViewer";
import { ImageViewer } from "./ImageViewer";
import { PdfViewer } from "./PdfViewer";
import { TextViewer } from "./TextViewer";
import { VideoViewer } from "./VideoViewer";
import { ViewerLoading } from "./ViewerLoading";
import { useFileViewerDialog } from "./useFileViewerDialog";

type FileViewerDialogProps = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  fileName: string;
  extension?: string;
  content: number[] | null;
};

export const FileViewerDialog = ({
  isOpen,
  onOpenChange,
  fileName,
  extension,
  content,
}: FileViewerDialogProps) => {
  const { isImage, isPdf, isAudio, isVideo, blobUrl, text } = useFileViewerDialog(
    content,
    extension,
  );

  const renderContent = () => {
    if (!content) {
      return <ViewerLoading />;
    }

    if (isImage) {
      return <ImageViewer url={blobUrl} alt={fileName} />;
    }

    if (isPdf) {
      return <PdfViewer url={blobUrl} />;
    }

    if (isAudio) {
      return <AudioViewer url={blobUrl} fileName={fileName} />;
    }

    if (isVideo) {
      return <VideoViewer url={blobUrl} fileName={fileName} />;
    }

    return <TextViewer text={text} />;
  };

  const Icon = isImage
    ? ImageIcon
    : isVideo
      ? FileVideoIcon
      : isAudio
        ? FileAudioIcon
        : text != null
          ? FileTextIcon
          : FileIcon;

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[90vh] max-w-4xl flex-col gap-0 overflow-hidden p-0 sm:max-w-5xl">
        <DialogHeader className="flex flex-row items-center justify-between border-b border-border/40 bg-muted/20 px-6 py-4">
          <div className="flex items-center gap-3">
            <div className="flex size-10 items-center justify-center rounded-xl border border-border/50 bg-background shadow-xs">
              <Icon className="size-5 text-muted-foreground" />
            </div>
            <div>
              <DialogTitle className="text-base leading-none font-semibold tracking-tight">
                {fileName}
              </DialogTitle>
              {extension != null && (
                <p className="mt-1.5 text-xs font-medium tracking-wide text-muted-foreground uppercase">
                  {extension} FILE
                </p>
              )}
            </div>
          </div>
        </DialogHeader>

        <div
          className={cn(
            "relative flex h-[50vh] w-full flex-col items-center justify-center overflow-hidden bg-muted/10 sm:h-[60vh] md:h-[70vh]",
            !isImage &&
              !isPdf &&
              !isAudio &&
              !isVideo &&
              content &&
              "items-start justify-start bg-zinc-950",
          )}>
          {renderContent()}
        </div>
      </DialogContent>
    </Dialog>
  );
};
