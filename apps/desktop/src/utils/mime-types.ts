export type FileKind = "image" | "pdf" | "audio" | "video" | "text" | null;

const IMAGE_EXTENSIONS = new Set(["png", "jpg", "jpeg", "gif", "webp", "svg", "ico"]);

const AUDIO_EXTENSIONS = new Set(["mp3", "wav", "ogg", "flac", "m4a"]);

const VIDEO_EXTENSIONS = new Set(["mp4", "mov", "avi", "webm", "mkv"]);

const MIME_TYPES: Record<string, string> = {
  mp3: "audio/mpeg",
  wav: "audio/wav",
  ogg: "audio/ogg",
  flac: "audio/flac",
  m4a: "audio/mp4",

  mp4: "video/mp4",
  mov: "video/quicktime",
  avi: "video/x-msvideo",
  webm: "video/webm",
  mkv: "video/x-matroska",

  svg: "image/svg+xml",

  pdf: "application/pdf",
};

const DEFAULT_MIME_BY_KIND: Record<Exclude<FileKind, "pdf" | "text" | null>, string> = {
  image: "image",
  audio: "audio",
  video: "video",
};

export function getFileKind(extension?: string): FileKind {
  if (extension == null) return null;

  const ext = extension.toLowerCase();

  if (ext === "pdf") return "pdf";
  if (IMAGE_EXTENSIONS.has(ext)) return "image";
  if (AUDIO_EXTENSIONS.has(ext)) return "audio";
  if (VIDEO_EXTENSIONS.has(ext)) return "video";

  return "text";
}

export function getMimeType(kind: FileKind, extension?: string): string {
  if (!kind || extension == null) return "";

  const ext = extension.toLowerCase();

  return (
    MIME_TYPES[ext] ??
    (Boolean(DEFAULT_MIME_BY_KIND[kind as keyof typeof DEFAULT_MIME_BY_KIND])
      ? `${DEFAULT_MIME_BY_KIND[kind as keyof typeof DEFAULT_MIME_BY_KIND]}/${ext}`
      : "")
  );
}
