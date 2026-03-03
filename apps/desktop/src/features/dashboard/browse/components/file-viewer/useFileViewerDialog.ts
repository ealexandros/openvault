import { getFileKind, getMimeType } from "@/utils/mime-types";
import { safeUint8ArrayParse } from "@/utils/safe-parse";
import { useEffect, useEffectEvent, useState } from "react";

export const useFileViewerDialog = (content: number[] | null, extension?: string) => {
  const [fileUrl, setFileUrl] = useState<string | null>(null);

  const fileKind = getFileKind(extension);
  const bytes = content ? new Uint8Array(content) : null;

  const fileType = (() => {
    switch (fileKind) {
      case "image":
      case "pdf":
      case "audio":
      case "video":
        return fileKind;
      case "text":
      case null:
      default:
        return "text";
    }
  })();

  const setFileUrlEvent = useEffectEvent((url: string | null) => {
    setFileUrl(url);
  });

  useEffect(() => {
    if (!content || !fileKind || fileKind === "text") {
      setFileUrlEvent(null);
      return;
    }

    const bytes = new Uint8Array(content);
    const type = getMimeType(fileKind, extension);

    const url = URL.createObjectURL(new Blob([bytes], { type }));
    setFileUrlEvent(url);

    return () => URL.revokeObjectURL(url);
  }, [content, fileKind, extension]);

  let text: string | null = null;

  if (fileKind === "text" && bytes) {
    text = safeUint8ArrayParse(bytes) ?? "Binary content cannot be displayed.";
  }

  return {
    fileType,
    isImage: fileKind === "image",
    isPdf: fileKind === "pdf",
    isAudio: fileKind === "audio",
    isVideo: fileKind === "video",
    fileUrl,
    text,
  };
};
