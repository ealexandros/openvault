import { getFileTypeOrDefault, getMimeType } from "@/utils/mime-types";
import { safeUint8ArrayParse } from "@/utils/safe-parse";
import { useEffect, useEffectEvent, useState } from "react";

export const useFileViewerDialog = (content: number[] | null, extension?: string) => {
  const [fileUrl, setFileUrl] = useState<string | null>(null);

  const fileType = getFileTypeOrDefault(extension);
  const bytes = content ? new Uint8Array(content) : null;

  const setFileUrlEvent = useEffectEvent((url: string | null) => {
    setFileUrl(url);
  });

  useEffect(() => {
    if (!content || fileType === "text") {
      setFileUrlEvent(null);
      return;
    }

    const bytes = new Uint8Array(content);
    const type = getMimeType(fileType, extension);

    const url = URL.createObjectURL(new Blob([bytes], { type }));
    setFileUrlEvent(url);

    return () => URL.revokeObjectURL(url);
  }, [content, fileType, extension]);

  let text: string | null = null;

  if (fileType === "text" && bytes) {
    text = safeUint8ArrayParse(bytes) ?? "Binary content cannot be displayed.";
  }

  return {
    fileType,
    isImage: fileType === "image",
    isPdf: fileType === "pdf",
    isAudio: fileType === "audio",
    isVideo: fileType === "video",
    fileUrl,
    text,
  };
};
