"use client";

import { FileDropListener } from "@/components/file-drop/FileDropListener";
import { Button } from "@/components/ui/shadcn/button";
import { Spinner } from "@/components/ui/shadcn/spinner";
import { FileIcon, FolderIcon } from "lucide-react";
import { useState } from "react";
import { BrowseDropOverlay } from "./_components_/BrowseDropOverlay";
import { BrowseHeader } from "./_components_/BrowseHeader";
import { BrowseSection } from "./_components_/BrowseSection";
import { EmptyState } from "./_components_/EmptyState";
import { FileCard } from "./_components_/files/FileCard";
import { FileGridSkeleton } from "./_components_/files/FileGridSkeleton";
import { FileViewerDialog } from "./_components_/files/FileViewerDialog";
import { BackButton } from "./_components_/folders/BackButton";
import { ChangeFolderIconDialog } from "./_components_/folders/ChangeFolderIconDialog";
import { FolderCard } from "./_components_/folders/FolderCard";
import { FolderGridSkeleton } from "./_components_/folders/FolderGridSkeleton";
import { RenameItemDialog } from "./_components_/RenameItemDialog";
import { BrowseViewState, useBrowse } from "./useBrowse";

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
    isNavigating,
    handleDropPaths,
    renamingItem,
    viewingItem,
    handleFolderClick,
    handleBreadcrumbClick,
    handleCreateFolder,
    handleUploadFile,
    handleDeleteFolder,
    handleDeleteFile,
    handleRequestFolderRename,
    handleChangeFolderIcon,
    handleRequestFileRename,
    handleRenameDialogOpenChange,
    handleRenameFromDialog,
    handleFileClick,
    handleFileViewerOpenChange,
  } = useBrowse();

  const canGoBack = currentPath.length > 1;
  const [folderIdForIconChange, setFolderIdForIconChange] = useState<string | null>(null);

  const handleIconDialogOpenChange = (open: boolean) => {
    if (!open) {
      setFolderIdForIconChange(null);
    }
  };

  const handleIconSelect = async (iconName: string) => {
    const folderId = folderIdForIconChange;

    if (folderId == null) {
      return;
    }

    await handleChangeFolderIcon(folderId, iconName);
    setFolderIdForIconChange(null);
  };

  return (
    <FileDropListener onDropPaths={handleDropPaths}>
      {({ isDragging }) => (
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

          {isNavigating && (
            <div className="flex animate-in items-center gap-2 text-xs text-muted-foreground duration-200 fade-in">
              <Spinner className="size-3" />
              Loading folder...
            </div>
          )}

          {viewState === BrowseViewState.Loading && (
            <div className="space-y-10">
              <BrowseSection title="Folders" count={0} icon={FolderIcon}>
                <FolderGridSkeleton />
              </BrowseSection>
              <BrowseSection title="Files" count={0} icon={FileIcon}>
                <FileGridSkeleton />
              </BrowseSection>
            </div>
          )}

          {viewState === BrowseViewState.Empty && (
            <EmptyState
              canGoBack={canGoBack}
              onGoBack={() => handleBreadcrumbClick(currentPath.length - 2)}
            />
          )}

          {viewState === BrowseViewState.NoResults && (
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

          {viewState === BrowseViewState.Results && (
            <div className="space-y-10">
              {(folders.length > 0 || canGoBack) && (
                <BrowseSection title="Folders" count={folders.length} icon={FolderIcon}>
                  <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                    {canGoBack && (
                      <BackButton
                        handleBreadcrumbClick={handleBreadcrumbClick}
                        currentPath={currentPath}
                      />
                    )}
                    {folders.map(item => (
                      <FolderCard
                        key={item.id}
                        item={item}
                        onClick={() => handleFolderClick(item)}
                        onDelete={() => {
                          void handleDeleteFolder(item.id);
                        }}
                        onRename={() => handleRequestFolderRename(item)}
                        onChangeIcon={() => {
                          setFolderIdForIconChange(item.id);
                        }}
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
          <ChangeFolderIconDialog
            isOpen={folderIdForIconChange !== null}
            onOpenChange={handleIconDialogOpenChange}
            onSelectIcon={handleIconSelect}
          />
          <FileViewerDialog
            isOpen={viewingItem !== null}
            onOpenChange={handleFileViewerOpenChange}
            fileName={viewingItem?.name ?? ""}
            extension={viewingItem?.extension}
            content={viewingItem?.content ?? null}
          />
        </div>
      )}
    </FileDropListener>
  );
};

export default BrowsePage;
