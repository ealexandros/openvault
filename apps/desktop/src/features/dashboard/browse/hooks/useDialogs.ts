import { useDialogState } from "@/hooks/useDialogState";
import { ItemType, type FileItemResult, type FolderItemResult } from "@/types/filesystem";
import { DeletionTarget, ExportTarget, RenameTarget } from "../types";

export const useDialogs = () => {
  const deleteDialog = useDialogState<DeletionTarget>();
  const exportDialog = useDialogState<ExportTarget>();
  const renameDialog = useDialogState<RenameTarget>();
  const filePropertiesDialog = useDialogState<FileItemResult>();
  const folderPropertiesDialog = useDialogState<FolderItemResult>();
  const folderIconDialog = useDialogState<FolderItemResult>();
  const filePreviewDialog = useDialogState<FileItemResult>();

  const requestFolderIconChange = (folder: FolderItemResult) => {
    folderIconDialog.open(folder);
  };

  const requestFileDeletion = (file: FileItemResult) => {
    deleteDialog.open({ id: file.id, name: file.name, type: ItemType.FILE });
  };

  const requestFolderDeletion = (folder: FolderItemResult) => {
    deleteDialog.open({ id: folder.id, name: folder.name, type: ItemType.FOLDER });
  };

  const requestFileExport = (file: FileItemResult) => {
    exportDialog.open({ id: file.id, name: file.name, type: ItemType.FILE });
  };

  const requestFolderExport = (folder: FolderItemResult) => {
    exportDialog.open({ id: folder.id, name: folder.name, type: ItemType.FOLDER });
  };

  const requestFileProperties = (file: FileItemResult) => {
    filePropertiesDialog.open(file);
  };

  const requestFolderProperties = (folder: FolderItemResult) => {
    folderPropertiesDialog.open(folder);
  };

  const requestFolderRename = (folder: FolderItemResult) => {
    renameDialog.open({ id: folder.id, name: folder.name, type: ItemType.FOLDER });
  };

  const requestFileRename = (file: FileItemResult) => {
    renameDialog.open({ id: file.id, name: file.name, type: ItemType.FILE });
  };

  const requestFilePreview = (file: FileItemResult) => {
    filePreviewDialog.open(file);
  };

  return {
    delete: deleteDialog,
    export: exportDialog,
    rename: renameDialog,
    fileProperties: filePropertiesDialog,
    folderProperties: folderPropertiesDialog,
    folderIcon: folderIconDialog,
    filePreview: filePreviewDialog,
    requestFolderIconChange,
    requestFileDeletion,
    requestFolderDeletion,
    requestFileExport,
    requestFolderExport,
    requestFileProperties,
    requestFolderProperties,
    requestFilePreview,
    requestFolderRename,
    requestFileRename,
  };
};
