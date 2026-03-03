"use client";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { type ReactNode, useEffect, useRef, useState } from "react";

type FileDropListenerProps = {
  onDropPaths: (paths: string[]) => void | Promise<void>;
  onDragEnter?: () => void;
  onDragLeave?: () => void;
  children: (state: { isDragging: boolean }) => ReactNode;
};

type DragDropEventPayload = {
  payload?: {
    paths?: string[];
  };
};

export const FileDropListener = ({
  onDropPaths,
  onDragEnter,
  onDragLeave,
  children,
}: FileDropListenerProps) => {
  const [isDragging, setIsDragging] = useState(false);

  const onDropPathsRef = useRef(onDropPaths);
  const onDragEnterRef = useRef(onDragEnter);
  const onDragLeaveRef = useRef(onDragLeave);

  useEffect(() => {
    onDropPathsRef.current = onDropPaths;
    onDragEnterRef.current = onDragEnter;
    onDragLeaveRef.current = onDragLeave;
  }, [onDropPaths, onDragEnter, onDragLeave]);

  useEffect(() => {
    const appWindow = getCurrentWindow();

    const unlistenPromises = Promise.all([
      appWindow.listen("tauri://drag-enter", () => {
        setIsDragging(true);
        onDragEnterRef.current?.();
      }),
      appWindow.listen("tauri://drag-leave", () => {
        setIsDragging(false);
        onDragLeaveRef.current?.();
      }),
      appWindow.listen("tauri://drag-drop", event => {
        setIsDragging(false);
        onDragLeaveRef.current?.();

        const paths = (event as DragDropEventPayload).payload?.paths ?? [];
        void onDropPathsRef.current(paths);
      }),
    ]);

    return () => {
      void unlistenPromises.then(unlisteners => unlisteners.forEach(unlisten => unlisten()));
    };
  }, []);

  return <>{children({ isDragging })}</>;
};
