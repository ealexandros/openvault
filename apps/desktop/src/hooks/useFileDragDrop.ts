import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useRef, useState } from "react";

type UseFileDragDropProps = {
  onDrop: (event: unknown) => void | Promise<void>;
};

export const useFileDragDrop = ({ onDrop }: UseFileDragDropProps) => {
  const [isDragging, setIsDragging] = useState(false);
  const onDropRef = useRef(onDrop);

  useEffect(() => {
    onDropRef.current = onDrop;
  }, [onDrop]);

  useEffect(() => {
    const window = getCurrentWindow();

    const unlistenPromises = Promise.all([
      window.listen("tauri://drag-enter", () => {
        setIsDragging(true);
      }),
      window.listen("tauri://drag-leave", () => {
        setIsDragging(false);
      }),
      window.listen("tauri://drag-drop", event => {
        setIsDragging(false);
        void onDropRef.current(event);
      }),
    ]);

    return () => {
      void unlistenPromises.then(unlisteners => unlisteners.forEach(unlisten => unlisten()));
    };
  }, []);

  return { isDragging };
};
