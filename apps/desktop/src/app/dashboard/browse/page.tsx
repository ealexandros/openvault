"use client";

import { FileDropListener } from "@/components/functional/FileDropListener";
import { FileDropOverlayView } from "@/components/views/FileDropOverlayView";
import {
  BrowseHeader,
  BrowseSkeleton,
  BrowseViewState,
  ChangeFolderIconDialog,
  DeleteConfirmationDialog,
  EmptyFolder,
  EmptySearchResult,
  FilePropertiesDialog,
  FileViewerDialog,
  FolderPropertiesDialog,
  RenameItemDialog,
  useBrowse,
} from "@/features/dashboard/browse";
import { FolderItemResult, type FileItemResult } from "@/types/filesystem";
import { useState } from "react";
import { FilesSection } from "./_components_/FilesSection";
import { FoldersSection } from "./_components_/FoldersSection";

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
    handleBackClick,
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

          {viewState === BrowseViewState.Results && (
            <section className="space-y-10 transition-opacity duration-200">
              <FoldersSection
                folders={folders}
                canGoBack={canGoBack}
                isNavigating={isNavigating}
                onBackClick={handleBackClick}
                onFolderClick={handleFolderClick}
                onFolderRename={handleRequestFolderRename}
                onFolderToggleFavourite={handleToggleFolderFavourite}
                onFolderProperties={setFolderForProperties}
                onFolderDelete={folder =>
                  setItemForDeletion({ id: folder.id, name: folder.name, type: "folder" })
                }
                onFolderChangeIcon={folder => setFolderIdForIconChange(folder.id)}
              />
              <FilesSection
                files={files}
                onFileClick={handleFileClick}
                onFileRename={handleRequestFileRename}
                onFileToggleFavourite={handleToggleFileFavourite}
                onFileProperties={setFileForProperties}
                onFileDelete={file =>
                  setItemForDeletion({ id: file.id, name: file.name, type: "file" })
                }
              />
            </section>
          )}

          {viewState === BrowseViewState.Empty && (
            <EmptyFolder canGoBack={canGoBack} onGoBack={handleBackClick} />
          )}

          {viewState === BrowseViewState.NoResults && (
            <EmptySearchResult searchQuery={searchQuery} onClearSearch={clearSearch} />
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
