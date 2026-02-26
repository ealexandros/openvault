"use client";

import { Button } from "@/components/ui/shadcn/button";
import { PlusIcon } from "lucide-react";
import { useState } from "react";
import { Breadcrumbs } from "./_components_/Breadcrumbs";
import { EmptyState } from "./_components_/EmptyState";
import { FileCard } from "./_components_/FileCard";
import { NewFolderDialog } from "./_components_/NewFolderDialog";
import { RenameItemDialog } from "./_components_/RenameItemDialog";
import { useBrowse } from "./useBrowse";

const BrowsePage = () => {
  const {
    currentPath,
    currentFiles,
    handleFolderClick,
    handleBreadcrumbClick,
    handleResetPath,
    handleCreateFolder,
    handleDeleteItem,
    handleRenameItem,
  } = useBrowse();

  const [renamingItem, setRenamingItem] = useState<{
    id: string;
    name: string;
    type: "file" | "folder";
  } | null>(null);

  return (
    <div className="mx-auto max-w-5xl space-y-8">
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
              size="sm"
              className="h-9 rounded-xl px-4 text-xs font-semibold shadow-lg shadow-primary/20 transition-all hover:scale-105 active:scale-95">
              <PlusIcon className="mr-2 size-3.5" />
              Upload
            </Button>
          </div>
        </div>

        <div className="grid grid-cols-1 gap-4 pb-12 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {currentFiles.map(item => (
            <FileCard
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
          ))}

          {currentFiles.length === 0 && <EmptyState />}
        </div>
      </div>

      <RenameItemDialog
        isOpen={renamingItem !== null}
        onOpenChange={open => !open && setRenamingItem(null)}
        initialName={renamingItem?.name ?? ""}
        itemType={renamingItem?.type ?? "file"}
        onRename={async newName => {
          if (renamingItem) {
            await handleRenameItem(renamingItem.id, renamingItem.type, newName);
          }
        }}
      />
    </div>
  );
};

export default BrowsePage;
