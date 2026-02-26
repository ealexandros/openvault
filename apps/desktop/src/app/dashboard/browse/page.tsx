"use client";

import { Button } from "@/components/ui/shadcn/button";
import { AnimatePresence, motion } from "framer-motion";
import { PlusIcon, UploadCloudIcon } from "lucide-react";
import { useState } from "react";
import { Breadcrumbs } from "./_components_/Breadcrumbs";
import { EmptyState } from "./_components_/EmptyState";
import { FileCard } from "./_components_/FileCard";
import { FileViewerDialog } from "./_components_/FileViewerDialog";
import { FolderCard } from "./_components_/FolderCard";
import { NewFolderDialog } from "./_components_/NewFolderDialog";
import { RenameItemDialog } from "./_components_/RenameItemDialog";
import { useBrowse } from "./useBrowse";

const BrowsePage = () => {
  const {
    currentPath,
    currentFiles,
    isDragging,
    handleFolderClick,
    handleBreadcrumbClick,
    handleResetPath,
    handleCreateFolder,
    handleDeleteItem,
    handleRenameItem,
    handleUploadFile,
    getFileContent,
  } = useBrowse();

  const [renamingItem, setRenamingItem] = useState<{
    id: string;
    name: string;
    type: "file" | "folder";
  } | null>(null);

  const [viewingItem, setViewingItem] = useState<{
    id: string;
    name: string;
    mimeType?: string;
    content: number[] | null;
  } | null>(null);

  const handleFileClick = async (item: { id: string; name: string; mimeType?: string }) => {
    setViewingItem({
      id: item.id,
      name: item.name,
      mimeType: item.mimeType,
      content: null,
    });
    const content = await getFileContent(item.id);
    setViewingItem({
      id: item.id,
      name: item.name,
      mimeType: item.mimeType,
      content,
    });
  };

  return (
    <div className="relative mx-auto max-w-5xl space-y-8">
      <AnimatePresence>
        {isDragging && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-50 flex items-center justify-center bg-background/60 backdrop-blur-sm">
            <motion.div
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.9, opacity: 0 }}
              className="flex flex-col items-center gap-4 rounded-3xl border-2 border-dashed border-primary bg-primary/5 p-12 text-center shadow-2xl shadow-primary/20">
              <div className="rounded-full bg-primary/10 p-4 text-primary">
                <UploadCloudIcon className="size-12 animate-bounce" />
              </div>
              <div className="space-y-1">
                <h2 className="text-2xl font-bold tracking-tight text-primary">
                  Drop files to upload
                </h2>
                <p className="text-muted-foreground">
                  Release your files to securely add them
                </p>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      <div className="space-y-6">
        <div className="sticky top-0 z-10 flex flex-col gap-4 bg-background/95 py-2 backdrop-blur md:flex-row md:items-center md:justify-between">
          <div className="space-y-1">
            <h3 className="text-lg font-semibold tracking-tight">Files</h3>
            <Breadcrumbs
              currentPath={currentPath}
              onReset={handleResetPath}
              onClick={handleBreadcrumbClick}
            />
          </div>
          <div className="flex gap-2">
            <NewFolderDialog onCreate={handleCreateFolder} />
            <Button
              onClick={() => {
                void handleUploadFile();
              }}
              size="sm"
              className="h-9 rounded-xl px-4 text-xs font-semibold shadow-lg shadow-primary/20 transition-all hover:scale-105 active:scale-95">
              <PlusIcon className="mr-2 size-3.5" />
              Upload file
            </Button>
          </div>
        </div>

        <div className="grid grid-cols-1 gap-4 pb-12 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {currentFiles.map(item =>
            item.type === "folder" ? (
              <FolderCard
                key={item.id}
                item={item}
                onClick={() => handleFolderClick(item)}
                onDelete={() => {
                  void handleDeleteItem(item.id, item.type);
                }}
                onRename={() => {
                  setRenamingItem({ id: item.id, name: item.name, type: item.type });
                }}
              />
            ) : (
              <FileCard
                key={item.id}
                item={item}
                onClick={() => {
                  void handleFileClick(item);
                }}
                onDelete={() => {
                  void handleDeleteItem(item.id, item.type);
                }}
                onRename={() => {
                  setRenamingItem({ id: item.id, name: item.name, type: item.type });
                }}
              />
            ),
          )}

          {currentFiles.length === 0 && <EmptyState />}
        </div>
      </div>

      <RenameItemDialog
        isOpen={renamingItem !== null}
        onOpenChange={open => {
          if (!open) setRenamingItem(null);
        }}
        initialName={renamingItem?.name ?? ""}
        itemType={renamingItem?.type ?? "file"}
        onRename={async newName => {
          if (renamingItem) {
            await handleRenameItem(renamingItem.id, renamingItem.type, newName);
          }
        }}
      />

      <FileViewerDialog
        isOpen={viewingItem !== null}
        onOpenChange={open => {
          if (!open) setViewingItem(null);
        }}
        fileName={viewingItem?.name ?? ""}
        mimeType={viewingItem?.mimeType}
        content={viewingItem?.content ?? null}
      />
    </div>
  );
};

export default BrowsePage;
