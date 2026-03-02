"use client";

import { Button } from "@/components/ui/shadcn/button";
import { FileIcon, FolderIcon } from "lucide-react";
import { BrowseDropOverlay } from "./_components_/BrowseDropOverlay";
import { BrowseHeader } from "./_components_/BrowseHeader";
import { BrowseLoadingState } from "./_components_/BrowseLoadingState";
import { BrowseSection } from "./_components_/BrowseSection";
import { EmptyState } from "./_components_/EmptyState";
import { FileCard } from "./_components_/FileCard";
import { FileViewerDialog } from "./_components_/FileViewerDialog";
import { FolderCard } from "./_components_/FolderCard";
import { RenameItemDialog } from "./_components_/RenameItemDialog";
import { useBrowse } from "./useBrowse";

const BrowsePage = () => {
  const {
    currentPath,
    folders,
    files,
    folderCount,
    fileCount,
    searchQuery,
    setSearchQuery,
    clearSearch,
    viewState,
    isDragging,
    renamingItem,
    viewingItem,
    handleFolderClick,
    handleBreadcrumbClick,
    handleCreateFolder,
    handleUploadFile,
    handleDeleteFolder,
    handleDeleteFile,
    handleRequestFolderRename,
    handleRequestFileRename,
    handleRenameDialogOpenChange,
    handleRenameFromDialog,
    handleFileClick,
    handleFileViewerOpenChange,
  } = useBrowse();

  return (
    <div className="relative mx-auto max-w-7xl space-y-12 px-4 pb-12">
      <BrowseDropOverlay isVisible={isDragging} />

      <BrowseHeader
        currentPath={currentPath}
        folderCount={folderCount}
        fileCount={fileCount}
        searchQuery={searchQuery}
        onSearchQueryChange={setSearchQuery}
        onBreadcrumbClick={handleBreadcrumbClick}
        onUploadFile={() => {
          void handleUploadFile();
        }}
        onCreateFolder={handleCreateFolder}
      />

      {viewState === "loading" && <BrowseLoadingState />}

      {viewState === "empty" && <EmptyState />}

      {viewState === "no-results" && (
        <div className="flex animate-in flex-col items-center gap-3 rounded-2xl border border-dashed px-8 py-16 text-center duration-300 fade-in slide-in-from-bottom-2">
          <p className="text-base font-medium">No matches found</p>
          <p className="max-w-md text-sm text-muted-foreground">
            Nothing matches <span>&ldquo;{searchQuery}&rdquo;</span>. Try another keyword.
          </p>
          <Button variant="outline" onClick={clearSearch}>
            Clear search
          </Button>
        </div>
      )}

      {viewState === "results" && (
        <div className="space-y-10">
          {folders.length > 0 && (
            <BrowseSection title="Folders" count={folders.length} icon={FolderIcon}>
              <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                {folders.map(item => (
                  <FolderCard
                    key={item.id}
                    item={item}
                    onClick={() => handleFolderClick(item)}
                    onDelete={() => {
                      void handleDeleteFolder(item.id);
                    }}
                    onRename={() => handleRequestFolderRename(item)}
                  />
                ))}
              </div>
            </BrowseSection>
          )}

          {files.length > 0 && (
            <BrowseSection title="Files" count={files.length} icon={FileIcon}>
              <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                {files.map(item => (
                  <FileCard
                    key={item.id}
                    item={item}
                    onClick={() => {
                      void handleFileClick(item);
                    }}
                    onDelete={() => {
                      void handleDeleteFile(item.id);
                    }}
                    onRename={() => handleRequestFileRename(item)}
                  />
                ))}
              </div>
            </BrowseSection>
          )}
        </div>
      )}

      <RenameItemDialog
        isOpen={renamingItem !== null}
        onOpenChange={handleRenameDialogOpenChange}
        initialName={renamingItem?.name ?? ""}
        itemType={renamingItem?.type ?? "file"}
        onRename={handleRenameFromDialog}
      />
      <FileViewerDialog
        isOpen={viewingItem !== null}
        onOpenChange={handleFileViewerOpenChange}
        fileName={viewingItem?.name ?? ""}
        extension={viewingItem?.extension}
        content={viewingItem?.content ?? null}
      />
    </div>
  );
};

export default BrowsePage;
