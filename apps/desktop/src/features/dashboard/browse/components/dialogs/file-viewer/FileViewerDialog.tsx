import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
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
  fileName: string;
  extension?: string;
  content: number[] | null;
  onOpenChange: (open: boolean) => void;
};

const IconMap: Record<NonNullable<FileType>, React.ElementType> = {
  image: ImageIcon,
  video: FileVideoIcon,
  audio: FileAudioIcon,
  pdf: FileTextIcon,
  text: FileTextIcon,
};

export const FileViewerDialog = ({
  isOpen,
  onOpenChange,
  fileName,
  extension,
  content,
}: FileViewerDialogProps) => {
  const { fileType, fileUrl, text } = useFileViewerDialog(content, extension);

  const viewers: Record<NonNullable<FileType>, React.ReactNode> = {
    image: <ImageViewer url={fileUrl} alt={fileName} />,
    pdf: <PdfViewer url={fileUrl} />,
    audio: <AudioViewer url={fileUrl} fileName={fileName} />,
    video: <VideoViewer url={fileUrl} fileName={fileName} />,
    text: <TextViewer text={text} />,
  };

  const Icon = IconMap[fileType];

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[90vh] max-w-4xl flex-col overflow-hidden p-0 sm:max-w-5xl">
        <DialogHeader className="flex items-start justify-between border-b border-border/40 bg-muted/20 px-6 py-4">
          <div className="flex items-center gap-3">
            <div className="flex size-10 items-center justify-center rounded-xl border border-border/50 bg-background shadow-xs">
              <Icon className="size-5 text-muted-foreground" />
            </div>
            <div>
              <DialogTitle className="text-base font-semibold tracking-tight">
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
            "relative flex w-full overflow-hidden bg-muted/10",
            "h-[50vh] flex-col items-center justify-center sm:h-[60vh] md:h-[70vh]",
            fileType === "text" && content && "items-start justify-start bg-zinc-950",
          )}>
          {content ? viewers[fileType] : <ViewerLoading />}
        </div>
      </DialogContent>
    </Dialog>
  );
};
