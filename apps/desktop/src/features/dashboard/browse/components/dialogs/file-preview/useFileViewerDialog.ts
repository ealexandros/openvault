import { fetchApi } from "@/libraries/fetch";
import { tauriApi } from "@/libraries/tauri-api";
import { FileItemResult } from "@/types/filesystem";
import { getFileTypeOrDefault } from "@/utils/mime-types";
import { safeUint8ArrayParse } from "@/utils/safe-parse";
import { useEffect, useEffectEvent, useRef, useState } from "react";
import { toast } from "sonner";

type FileViewProps = {
  item: FileItemResult | null;
};

export const useFileViewerDialog = ({ item }: FileViewProps) => {
  const [contentUri, setContentUri] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const codeRef = useRef<HTMLElement | null>(null);

  const fileType = item ? getFileTypeOrDefault(item.extension) : "text";

  const showErrorToast = () => {
    toast.error("Failed to open file", {
      description: "The file may be corrupted or access is denied.",
    });
  };

  const clearContent = () => {
    if (codeRef.current) codeRef.current.textContent = "";
    setContentUri(null);
  };

  const fetchContent = useEffectEvent(async () => {
    if (item == null) return clearContent();

    setIsLoading(true);

    const result = await tauriApi.exposeFileUrl({ id: item.id });

    if (!result.success) {
      showErrorToast();
      setIsLoading(false);
      clearContent();
      return;
    }

    const uri = result.data;

    clearContent();

    if (fileType !== "text") {
      setContentUri(uri);
      setIsLoading(false);
      return;
    }

    const controller = new AbortController();
    const response = await fetchApi.get(uri, { signal: controller.signal });

    if (response == null || !response.ok) {
      showErrorToast();
      setIsLoading(false);
      return clearContent();
    }

    const buffer = await response.arrayBuffer();
    const uint8 = new Uint8Array(buffer);

    if (codeRef.current) {
      codeRef.current.textContent = safeUint8ArrayParse(uint8) ?? "Content not available.";
    }

    uint8.fill(0);
    setIsLoading(false);

    return () => controller.abort();
  });

  useEffect(() => {
    void fetchContent();
    return () => clearContent();
  }, [item]);

  return {
    fileType,
    contentUri,
    codeRef,
    isLoading,
  };
};
