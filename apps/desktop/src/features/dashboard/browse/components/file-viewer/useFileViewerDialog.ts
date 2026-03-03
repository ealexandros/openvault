import { getFileKind, getMimeType } from "@/utils/mime-types";
import { useEffect, useEffectEvent, useState } from "react";

export const useFileViewerDialog = (content: number[] | null, extension?: string) => {
  const fileKind = getFileKind(extension);
  const bytes = content ? new Uint8Array(content) : null;

  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const setBlobUrlEvent = useEffectEvent((url: string | null) => {
    setBlobUrl(url);
  });

  useEffect(() => {
    if (!content || !fileKind || fileKind === "text") {
      setBlobUrlEvent(null);
      return;
    }

    const bytes = new Uint8Array(content);
    const type = getMimeType(fileKind, extension);

    const url = URL.createObjectURL(new Blob([bytes], { type }));
    setBlobUrlEvent(url);

    return () => URL.revokeObjectURL(url);
  }, [content, fileKind, extension]);

  let text: string | null = null;

  if (fileKind === "text" && bytes) {
    try {
      text = new TextDecoder().decode(bytes);
    } catch {
      text = "Binary content cannot be displayed.";
    }
  }

  return {
    isImage: fileKind === "image",
    isPdf: fileKind === "pdf",
    isAudio: fileKind === "audio",
    isVideo: fileKind === "video",
    blobUrl,
    text,
  };
};
