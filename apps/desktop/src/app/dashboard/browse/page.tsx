"use client";

import { FileDropListener } from "@/components/functional/FileDropListener";
import { Button } from "@/components/ui/shadcn/button";
import { FileDropOverlayView } from "@/components/views/FileDropOverlayView";
import {
  BrowseHeader,
  BrowseSection,
  BrowseSkeleton,
  BrowseViewState,
  ChangeFolderIconDialog,
  DeleteConfirmationDialog,
  EmptyState,
  FileCard,
  FilePropertiesDialog,
  FileViewerDialog,
  FolderBackButton,
  FolderCard,
  FolderPropertiesDialog,
  RenameItemDialog,
  useBrowse,
} from "@/features/dashboard/browse";
import { type FileItemResult } from "@/types/filesystem";
import { cn } from "@/utils/cn";
import { FileIcon, FolderIcon } from "lucide-react";
import { useState } from "react";

// @todo-soon refactor useBrowse to use useFolder and useFile

const BrowsePage = () => {
  const {
    currentPath,
    folders,
    files,
    folderCount,
    fileCount,
    searchQuery,
    viewState,
    isNavigating,
    renamingItem,
    viewingItem,
    canGoBack,
    folderIdForIconChange,
    setSearchQuery,
    clearSearch,
    handleDropPaths,
    handleFolderClick,
    handleBreadcrumbClick,
    handleCreateFolder,
    handleUploadFile,
    handleUploadFolder,
    handleDeleteFolder,
    handleDeleteFile,
    handleRequestFolderRename,
    handleRequestFileRename,
    handleRenameDialogOpenChange,
    handleRenameFromDialog,
    handleFileClick,
    handleFileViewerOpenChange,
    handleIconDialogOpenChange,
    handleIconSelect,
    setFolderIdForIconChange,
    handleToggleFileFavourite,
    handleToggleFolderFavourite,
  } = useBrowse();

  const [fileForProperties, setFileForProperties] = useState<FileItemResult | null>(null);
  const [folderForProperties, setFolderForProperties] = useState<FileItemResult | null>(null);

  const [itemForDeletion, setItemForDeletion] = useState<{
    id: string;
    name: string;
    type: "file" | "folder";
  } | null>(null);

  const handleFilePropertiesOpenChange = (open: boolean) => {
    if (!open) {
      setFileForProperties(null);
    }
  };

  const handleFolderPropertiesOpenChange = (open: boolean) => {
    if (!open) {
      setFolderForProperties(null);
    }
  };

  const handleDeleteDialogOpenChange = (open: boolean) => {
    if (!open) {
      setItemForDeletion(null);
    }
  };

  const handleDeleteConfirm = async () => {
    if (itemForDeletion == null) {
      return;
    }

    if (itemForDeletion.type === "folder") {
      await handleDeleteFolder(itemForDeletion.id);
    } else {
      await handleDeleteFile(itemForDeletion.id);
    }
  };

  return (
    <FileDropListener onDropPaths={handleDropPaths}>
      {({ isDragging }) => (
        <main className="relative mx-auto h-full max-w-7xl space-y-16 px-4 py-8">
          <FileDropOverlayView isVisible={isDragging} />

          <BrowseHeader
            currentPath={currentPath}
            folderCount={folderCount}
            fileCount={fileCount}
            searchQuery={searchQuery}
            onSearchQueryChange={setSearchQuery}
            onBreadcrumbClick={handleBreadcrumbClick}
            onUploadFile={handleUploadFile}
            onUploadFolder={handleUploadFolder}
            onCreateFolder={handleCreateFolder}
          />

          <section className="h-4/5">
            {viewState === BrowseViewState.Loading && <BrowseSkeleton />}

            {viewState === BrowseViewState.Empty && (
              <div
                className={cn(
                  "transition-opacity duration-200",
                  isNavigating ? "opacity-50" : "opacity-100",
                )}>
                <EmptyState
                  canGoBack={canGoBack}
                  onGoBack={() => handleBreadcrumbClick(currentPath.length - 2)}
                />
              </div>
            )}

            {viewState === BrowseViewState.NoResults && (
              <div className="flex animate-in flex-col items-center gap-3 rounded-2xl border border-dashed px-8 py-16 text-center duration-300 fade-in slide-in-from-bottom-2">
                <p className="text-base font-medium">No matches found</p>
                <p className="max-w-md text-sm text-muted-foreground">
                  Nothing matches <span>&ldquo;{searchQuery}&rdquo;</span>. Try another
                  keyword.
                </p>
                <Button variant="outline" onClick={clearSearch}>
                  Clear search
                </Button>
              </div>
            )}

            {viewState === BrowseViewState.Results && (
              <div
                className={cn(
                  "space-y-10 transition-opacity duration-200",
                  isNavigating ? "opacity-50" : "opacity-100",
                )}>
                {(folders.length > 0 || canGoBack) && (
                  <BrowseSection title="Folders" count={folders.length} icon={FolderIcon}>
                    <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                      {canGoBack && (
                        <FolderBackButton
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
                            setItemForDeletion({
                              id: item.id,
                              name: item.name,
                              type: "folder",
                            });
                          }}
                          onRename={() => handleRequestFolderRename(item)}
                          onChangeIcon={() => {
                            setFolderIdForIconChange(item.id);
                          }}
                          onToggleFavourite={() => {
                            void handleToggleFolderFavourite(item.id, !item.isFavourite);
                          }}
                          onProperties={() => {
                            setFolderForProperties(item);
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
                          file={item}
                          onClick={() => {
                            void handleFileClick(item);
                          }}
                          onDelete={() => {
                            setItemForDeletion({
                              id: item.id,
                              name: item.name,
                              type: "file",
                            });
                          }}
                          onRename={() => handleRequestFileRename(item)}
                          onToggleFavourite={() => {
                            void handleToggleFileFavourite(item.id, !item.isFavourite);
                          }}
                          onProperties={() => {
                            setFileForProperties(item);
                          }}
                        />
                      ))}
                    </div>
                  </BrowseSection>
                )}
              </div>
            )}
          </section>

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
          <FilePropertiesDialog
            isOpen={fileForProperties !== null}
            onOpenChange={handleFilePropertiesOpenChange}
            item={fileForProperties}
          />
          <FolderPropertiesDialog
            isOpen={folderForProperties !== null}
            onOpenChange={handleFolderPropertiesOpenChange}
            item={folderForProperties}
          />
          <DeleteConfirmationDialog
            isOpen={itemForDeletion !== null}
            onOpenChange={handleDeleteDialogOpenChange}
            itemName={itemForDeletion?.name ?? ""}
            itemType={itemForDeletion?.type ?? "file"}
            onConfirm={handleDeleteConfirm}
          />
        </main>
      )}
    </FileDropListener>
  );
};

export default BrowsePage;
