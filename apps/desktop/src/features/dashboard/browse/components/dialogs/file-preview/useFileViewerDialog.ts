import { tauriApi } from "@/libraries/tauri-api";
import { FileItemResult } from "@/types/filesystem";
import { getFileTypeOrDefault, getMimeType } from "@/utils/mime-types";
import { safeUint8ArrayParse } from "@/utils/safe-parse";
import { useEffect, useEffectEvent, useState } from "react";

type FileViewProps = {
  item: FileItemResult | null;
};

type ContentType = number[] | null;

export const useFileViewerDialog = ({ item }: FileViewProps) => {
  const [content, setContent] = useState<ContentType>(null);
  const [fileUrl, setFileUrl] = useState<string | null>(null);

  const fileType = item ? getFileTypeOrDefault(item.extension) : "text";

  const setFileUrlEvent = useEffectEvent((url: string | null) => {
    setFileUrl(url);
  });

  const setContentEvent = useEffectEvent((content: ContentType) => {
    setContent(content);
  });

  useEffect(() => {
    if (item == null) {
      setContentEvent(null);
      return;
    }

    const controller = new AbortController();

    const fetchContent = async () => {
      const result = await tauriApi.readFileBytes({ id: item.id });
      if (controller.signal.aborted) return;
      setContent(result.success ? result.data : null);
    };

    void fetchContent();

    return () => controller.abort();
  }, [item]);

  useEffect(() => {
    if (!content || !item || fileType === "text") {
      setFileUrlEvent(null);
      return;
    }

    const bytes = new Uint8Array(content);
    const mimeType = getMimeType(fileType, item.extension);
    const url = URL.createObjectURL(new Blob([bytes], { type: mimeType }));

    setFileUrlEvent(url);

    return () => URL.revokeObjectURL(url);
  }, [content, fileType, item]);

  let textContent: string | null = null;

  if (fileType === "text" && content) {
    const bytes = new Uint8Array(content);
    textContent = safeUint8ArrayParse(bytes) ?? "Binary content cannot be displayed.";
  }

  return {
    fileType,
    fileUrl,
    text: textContent,
    content,
  };
};
