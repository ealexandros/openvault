"use client";

import { FileDropListener } from "@/components/functional/FileDropListener";
import { FileDropOverlayView } from "@/components/views/FileDropOverlayView";
import {
  BrowseHeader,
  BrowseViewState,
  DeleteItemDialog,
  EmptyFolder,
  EmptySearchResult,
  ExportItemDialog,
  FilePreviewDialog,
  FilePropertiesDialog,
  FolderPropertiesDialog,
  RenameItemDialog,
  useBrowse,
} from "@/features/dashboard/browse";
import { UpdateFolderIconDialog } from "@/features/dashboard/browse/components/dialogs/update-folder-icon";
import { FilesSection } from "./_components_/FilesSection";
import { FoldersSection } from "./_components_/FoldersSection";

// @todo-later implement the move functionality

const BrowsePage = () => {
  const { browseState, dialogs, upload, toggleFileFavourite, toggleFolderFavourite, refresh } =
    useBrowse();

  return (
    <FileDropListener onDropPaths={upload.paths}>
      {({ isDragging }) => (
        <main className="relative mx-auto h-full w-full space-y-16 p-8 xl:p-14 xl:py-8">
          <FileDropOverlayView isVisible={isDragging} />

          <BrowseHeader
            currentPath={browseState.currentPath}
            currentFolderId={browseState.currentFolderId}
            onFolderCreate={refresh}
            folderCount={browseState.folderCount}
            fileCount={browseState.fileCount}
            searchQuery={browseState.searchQuery}
            onSearchQueryChange={browseState.setSearchQuery}
            onBreadcrumbClick={browseState.navigateToBreadcrumb}
            onUploadFile={upload.files}
            onUploadFolder={upload.folders}
            canGoBack={browseState.canGoBack}
            canGoForward={browseState.canGoForward}
            onBack={browseState.goBack}
            onForward={browseState.goForward}
          />

          {browseState.viewState === BrowseViewState.Results && (
            <section className="space-y-10 pb-20">
              <FoldersSection
                folders={browseState.folders}
                onFolderClick={browseState.navigateToFolder}
                onFolderRename={dialogs.requestFolderRename}
                onFolderToggleFavourite={toggleFolderFavourite}
                onFolderProperties={dialogs.requestFolderProperties}
                onFolderDelete={dialogs.requestFolderDeletion}
                onFolderChangeIcon={dialogs.requestFolderIconChange}
                onFolderExport={dialogs.requestFolderExport}
                onUploadFolder={upload.folders}
              />
              <FilesSection
                files={browseState.files}
                onFileClick={dialogs.filePreview.open}
                onFileRename={dialogs.requestFileRename}
                onFileToggleFavourite={toggleFileFavourite}
                onFileProperties={dialogs.requestFileProperties}
                onFileDelete={dialogs.requestFileDeletion}
                onFileExport={dialogs.requestFileExport}
                onUploadFile={upload.files}
              />
            </section>
          )}

          {browseState.viewState === BrowseViewState.Empty && (
            <EmptyFolder
              canGoBack={browseState.canGoBack}
              onGoBack={browseState.goBack}
              onUploadFile={upload.files}
            />
          )}

          {browseState.viewState === BrowseViewState.NoResults && (
            <EmptySearchResult
              searchQuery={browseState.searchQuery}
              onClearSearch={browseState.clearSearch}
            />
          )}

          <FilePreviewDialog
            isOpen={dialogs.filePreview.isOpen}
            item={dialogs.filePreview.item}
            onOpenChange={dialogs.filePreview.toggle}
          />
          <FilePropertiesDialog
            isOpen={dialogs.fileProperties.isOpen}
            item={dialogs.fileProperties.item}
            onOpenChange={dialogs.fileProperties.toggle}
          />
          <FolderPropertiesDialog
            isOpen={dialogs.folderProperties.isOpen}
            item={dialogs.folderProperties.item}
            onOpenChange={dialogs.folderProperties.toggle}
          />
          <UpdateFolderIconDialog
            isOpen={dialogs.folderIcon.isOpen}
            item={dialogs.folderIcon.item}
            onOpenChange={dialogs.folderIcon.toggle}
            onUpdate={refresh}
          />
          <RenameItemDialog
            key={dialogs.rename.item?.id}
            isOpen={dialogs.rename.isOpen}
            item={dialogs.rename.item}
            onOpenChange={dialogs.rename.toggle}
            onRename={refresh}
          />
          <DeleteItemDialog
            isOpen={dialogs.delete.isOpen}
            item={dialogs.delete.item}
            onOpenChange={dialogs.delete.toggle}
            onDelete={refresh}
          />
          <ExportItemDialog
            isOpen={dialogs.export.isOpen}
            item={dialogs.export.item}
            onOpenChange={dialogs.export.toggle}
            onExport={refresh}
          />
        </main>
      )}
    </FileDropListener>
  );
};

export default BrowsePage;
