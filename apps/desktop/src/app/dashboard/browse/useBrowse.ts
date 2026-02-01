"use client";

import { useState } from "react";

export type FileItem = {
  id: string;
  name: string;
  type: "file" | "folder";
  details?: string;
  children?: FileItem[];
};

const MOCK_FILES_HIERARCHY: FileItem[] = [
  {
    id: "f1",
    name: "Work",
    type: "folder",
    children: [
      {
        id: "f1-1",
        name: "Projects",
        type: "folder",
        children: [
          { id: "f1-1-1", name: "roadmap.pdf", type: "file", details: "324 KB" },
          { id: "f1-1-2", name: "specs.docx", type: "file", details: "1.2 MB" },
        ],
      },
      { id: "f1-2", name: "budget.xlsx", type: "file", details: "45 KB" },
    ],
  },
  {
    id: "f2",
    name: "Personal",
    type: "folder",
    children: [
      {
        id: "f2-1",
        name: "Photos",
        type: "folder",
        children: [
          { id: "f2-1-1", name: "vacation.jpg", type: "file", details: "2.4 MB" },
          { id: "f2-1-2", name: "family.png", type: "file", details: "1.8 MB" },
        ],
      },
      { id: "f2-2", name: "notes.txt", type: "file", details: "2 KB" },
    ],
  },
  { id: "f3", name: "credentials.env", type: "file", details: "1 KB" },
];

export const useBrowse = () => {
  const [currentPath, setCurrentPath] = useState<string[]>([]);

  const getCurrentFiles = () => {
    let files = MOCK_FILES_HIERARCHY;
    for (const segment of currentPath) {
      const folder = files.find(f => f.name === segment && f.type === "folder");
      if (folder && folder.children) {
        files = folder.children;
      } else {
        return [];
      }
    }
    return files;
  };

  const handleFolderClick = (name: string) => {
    setCurrentPath(prev => [...prev, name]);
  };

  const handleBreadcrumbClick = (index: number) => {
    setCurrentPath(prev => prev.slice(0, index + 1));
  };

  const handleResetPath = () => {
    setCurrentPath([]);
  };

  return {
    currentPath,
    currentFiles: getCurrentFiles(),
    handleFolderClick,
    handleBreadcrumbClick,
    handleResetPath,
  };
};
