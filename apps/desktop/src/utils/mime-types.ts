export type FileType = "image" | "pdf" | "audio" | "video" | "text" | "code" | null;

const IMAGE_EXTENSIONS = new Set(["png", "jpg", "jpeg", "gif", "webp", "svg", "ico"]);
const AUDIO_EXTENSIONS = new Set(["mp3", "wav", "ogg", "flac", "m4a"]);
const VIDEO_EXTENSIONS = new Set(["mp4", "mov", "avi", "webm", "mkv"]);
const CODE_EXTENSIONS = new Set(["js", "ts", "tsx", "html", "css", "json", "md"]);
const TEXT_EXTENSIONS = new Set(["pdf", "doc", "docx", "txt"]);

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

const DEFAULT_MIME_BY_TYPE: Record<
  Exclude<FileType, "pdf" | "text" | "code" | null>,
  string
> = {
  image: "image",
  audio: "audio",
  video: "video",
};

export const getFileType = (extension?: string): FileType => {
  if (extension == null) return null;

  const ext = extension.toLowerCase();

  if (ext === "pdf") return "pdf";
  if (IMAGE_EXTENSIONS.has(ext)) return "image";
  if (AUDIO_EXTENSIONS.has(ext)) return "audio";
  if (VIDEO_EXTENSIONS.has(ext)) return "video";
  if (CODE_EXTENSIONS.has(ext)) return "code";
  if (TEXT_EXTENSIONS.has(ext)) return "text";

  return null;
};

export const getFileTypeOrDefault = (extension?: string): NonNullable<FileType> => {
  return getFileType(extension) ?? "text";
};

export const getMimeType = (kind: FileType, extension?: string): string => {
  if (!kind || extension == null) return "";

  const ext = extension.toLowerCase();

  return (
    MIME_TYPES[ext] ??
    (Boolean(DEFAULT_MIME_BY_TYPE[kind as keyof typeof DEFAULT_MIME_BY_TYPE])
      ? `${DEFAULT_MIME_BY_TYPE[kind as keyof typeof DEFAULT_MIME_BY_TYPE]}/${ext}`
      : "")
  );
};
