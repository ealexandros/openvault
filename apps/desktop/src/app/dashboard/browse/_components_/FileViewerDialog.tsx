import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/shadcn/dialog";
import { useEffect, useMemo, useState } from "react";

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
  const isImage = extension === "png" || extension === "jpg";

  const bytes = useMemo(() => (content ? new Uint8Array(content) : null), [content]);

  const [imageUrl, setImageUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!isImage || !bytes) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setImageUrl(null);
      return;
    }

    const blob = new Blob([bytes], { type: `image/${extension}` });
    const url = URL.createObjectURL(blob);
    setImageUrl(url);

    return () => URL.revokeObjectURL(url);
  }, [isImage, bytes, extension]);

  const text = useMemo(() => {
    if (isImage || !bytes) return null;

    try {
      return new TextDecoder().decode(bytes);
    } catch {
      return "Binary content cannot be displayed.";
    }
  }, [isImage, bytes]);

  const renderContent = () => {
    if (!content) return "Loading...";

    if (isImage) {
      if (imageUrl == null) return "Loading image...";

      return (
        // eslint-disable-next-line @next/next/no-img-element
        <img src={imageUrl} alt={fileName} className="max-h-full max-w-full object-contain" />
      );
    }

    return (
      <div className="w-full p-4 font-mono text-sm whitespace-pre-wrap text-foreground">
        {text}
      </div>
    );
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl sm:max-w-4xl">
        <DialogHeader>
          <DialogTitle>{fileName}</DialogTitle>
        </DialogHeader>

        <div className="flex max-h-[70vh] items-center justify-center overflow-auto rounded-lg border bg-muted/30 p-1">
          {renderContent()}
        </div>
      </DialogContent>
    </Dialog>
  );
};
