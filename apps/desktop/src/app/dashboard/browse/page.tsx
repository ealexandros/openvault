"use client";

import { FileDropListener } from "@/components/functional/FileDropListener";
import { FileDropOverlayView } from "@/components/views/FileDropOverlayView";
import {
  BrowseHeader,
  BrowseSection,
  BrowseSkeleton,
  BrowseViewState,
  ChangeFolderIconDialog,
  DeleteConfirmationDialog,
  EmptySearchResult,
  EmptyState,
  FilePropertiesDialog,
  FileViewerDialog,
  FolderBackButton,
  FolderCard,
  FolderPropertiesDialog,
  RenameItemDialog,
  useBrowse,
} from "@/features/dashboard/browse";
import { FileItem } from "@/features/dashboard/browse/components/FileItem";
import { FolderItemResult, type FileItemResult } from "@/types/filesystem";
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
  const [folderForProperties, setFolderForProperties] = useState<FolderItemResult | null>(
    null,
  );

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
        <main className="relative mx-auto h-full max-w-7xl space-y-16 py-8">
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

          {viewState === BrowseViewState.Loading && <BrowseSkeleton />}

          {viewState === BrowseViewState.Empty && (
            <EmptyState
              canGoBack={canGoBack}
              onGoBack={() => handleBreadcrumbClick(currentPath.length - 2)}
            />
          )}

          {viewState === BrowseViewState.NoResults && (
            <EmptySearchResult searchQuery={searchQuery} onClearSearch={clearSearch} />
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
                    {canGoBack && !isNavigating && (
                      <FolderBackButton
                        handleBreadcrumbClick={handleBreadcrumbClick}
                        currentPath={currentPath}
                      />
                    )}
                    {folders.map(item => (
                      <FolderCard
                        key={item.id}
                        folder={item}
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
                    {files.map(file => (
                      <FileItem
                        key={file.id}
                        file={file}
                        onClick={() => handleFileClick(file)}
                        onDelete={() => {
                          setItemForDeletion({
                            id: file.id,
                            name: file.name,
                            type: "file",
                          });
                        }}
                        onRename={() => handleRequestFileRename(file)}
                        onToggleFavourite={() =>
                          handleToggleFileFavourite(file.id, !file.isFavourite)
                        }
                        onProperties={() => setFileForProperties(file)}
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
