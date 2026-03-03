import { useLayoutEffect, useMemo, useState } from "react";

const IMAGE_EXTENSIONS = ["png", "jpg", "jpeg", "gif", "webp", "svg", "ico"];
const AUDIO_EXTENSIONS = ["mp3", "wav", "ogg", "flac", "m4a"];
const VIDEO_EXTENSIONS = ["mp4", "mov", "avi", "webm", "mkv"];

const MIME_TYPES: Record<string, string> = {
  // Audio
  mp3: "audio/mpeg",
  wav: "audio/wav",
  ogg: "audio/ogg",
  flac: "audio/flac",
  m4a: "audio/mp4",
  // Video
  mp4: "video/mp4",
  mov: "video/quicktime",
  avi: "video/x-msvideo",
  webm: "video/webm",
  mkv: "video/x-matroska",
  // Image fallbacks
  svg: "image/svg+xml",
};

// @todo-now refactor this...

export const useFileViewerDialog = (content: number[] | null, extension?: string) => {
  const isImage =
    extension != null ? IMAGE_EXTENSIONS.includes(extension.toLowerCase()) : false;

  const isPdf = extension != null ? extension.toLowerCase() === "pdf" : false;

  const isAudio =
    extension != null ? AUDIO_EXTENSIONS.includes(extension.toLowerCase()) : false;

  const isVideo =
    extension != null ? VIDEO_EXTENSIONS.includes(extension.toLowerCase()) : false;

  const bytes = useMemo(() => (content ? new Uint8Array(content) : null), [content]);

  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  useLayoutEffect(() => {
    if (!bytes || (!isImage && !isPdf && !isAudio && !isVideo)) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setBlobUrl(null);
      return;
    }

    const lowerExt = extension?.toLowerCase() ?? "";
    const type = isPdf
      ? "application/pdf"
      : isAudio || isVideo
        ? (MIME_TYPES[lowerExt] ?? `${isAudio ? "audio" : "video"}/${lowerExt}`)
        : (MIME_TYPES[lowerExt] ?? `image/${lowerExt}`);
    const blob = new Blob([bytes], { type });
    const url = URL.createObjectURL(blob);
    setBlobUrl(url);

    return () => URL.revokeObjectURL(url);
  }, [isImage, isPdf, isAudio, isVideo, bytes, extension]);

  const text = useMemo(() => {
    if (isImage || isPdf || isAudio || isVideo || !bytes) return null;

    try {
      return new TextDecoder().decode(bytes);
    } catch {
      return "Binary content cannot be displayed.";
    }
  }, [isImage, isPdf, isAudio, isVideo, bytes]);

  return {
    isImage,
    isPdf,
    isAudio,
    isVideo,
    blobUrl,
    text,
  };
};
