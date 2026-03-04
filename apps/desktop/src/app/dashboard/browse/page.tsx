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
import { FilesSection } from "./_components_/FilesSection";
import { FoldersSection } from "./_components_/FoldersSection";

const BrowsePage = () => {
  const { browseState, folderState, fileState, dialogState } = useBrowse();

  return (
    <FileDropListener onDropPaths={fileState.handleDropPaths}>
      {({ isDragging }) => (
        <main className="relative mx-auto h-full max-w-7xl space-y-16 py-16">
          <FileDropOverlayView isVisible={isDragging} />

          <BrowseHeader
            currentPath={browseState.currentPath}
            folderCount={browseState.folderCount}
            fileCount={browseState.fileCount}
            searchQuery={browseState.searchQuery}
            onSearchQueryChange={browseState.setSearchQuery}
            onBreadcrumbClick={folderState.openBreadcrumb}
            onUploadFile={fileState.uploadFile}
            onUploadFolder={fileState.uploadFolder}
            onCreateFolder={folderState.createFolder}
          />

          {browseState.viewState === BrowseViewState.Loading && <BrowseSkeleton />}

          {browseState.viewState === BrowseViewState.Results && (
            <section className="space-y-10 pb-20">
              <FoldersSection
                folders={folderState.folders}
                canGoBack={browseState.canGoBack}
                isNavigating={browseState.isNavigating}
                onBackClick={folderState.goBack}
                onFolderClick={folderState.openFolder}
                onFolderRename={folderState.requestRename}
                onFolderToggleFavourite={folderState.toggleFavourite}
                onFolderProperties={folderState.requestProperties}
                onFolderDelete={folderState.requestDelete}
                onFolderChangeIcon={folderState.requestIconChange}
              />
              <FilesSection
                files={fileState.files}
                onFileClick={fileState.openFile}
                onFileRename={fileState.requestRename}
                onFileToggleFavourite={fileState.toggleFavourite}
                onFileProperties={fileState.requestProperties}
                onFileDelete={fileState.requestDelete}
              />
            </section>
          )}

          {browseState.viewState === BrowseViewState.Empty && (
            <EmptyFolder canGoBack={browseState.canGoBack} onGoBack={folderState.goBack} />
          )}

          {browseState.viewState === BrowseViewState.NoResults && (
            <EmptySearchResult
              searchQuery={browseState.searchQuery}
              onClearSearch={browseState.clearSearch}
            />
          )}

          <FileViewerDialog
            isOpen={dialogState.isFileViewerVisible}
            onOpenChange={dialogState.toggleFileViewerVisibility}
            fileName={dialogState.viewingItem?.name ?? ""}
            extension={dialogState.viewingItem?.extension}
            content={dialogState.viewingItem?.content ?? null}
          />
          <RenameItemDialog
            isOpen={dialogState.isRenameVisible}
            onOpenChange={dialogState.toggleRenameVisibility}
            initialName={dialogState.renameInitialName}
            itemType={dialogState.renameItemType}
            onRename={dialogState.submitRename}
          />
          <ChangeFolderIconDialog
            isOpen={dialogState.isFolderIconPickerVisible}
            onOpenChange={dialogState.toggleFolderIconPickerVisibility}
            onSelectIcon={dialogState.selectFolderIcon}
          />
          <FilePropertiesDialog
            isOpen={dialogState.isFilePropertiesVisible}
            onOpenChange={dialogState.toggleFilePropertiesVisibility}
            item={dialogState.fileForProperties}
          />
          <FolderPropertiesDialog
            isOpen={dialogState.isFolderPropertiesVisible}
            onOpenChange={dialogState.toggleFolderPropertiesVisibility}
            item={dialogState.folderForProperties}
          />
          <DeleteConfirmationDialog
            isOpen={dialogState.isDeleteConfirmationVisible}
            onOpenChange={dialogState.toggleDeleteConfirmationVisibility}
            itemName={dialogState.deleteItemName}
            itemType={dialogState.deleteItemType}
            onConfirm={dialogState.confirmDeleteSelection}
          />
        </main>
      )}
    </FileDropListener>
  );
};

export default BrowsePage;
