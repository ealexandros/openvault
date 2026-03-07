import { logger } from "@/libraries/logger";
import { ItemType, type BrowseResult } from "@/types/filesystem";
import { invoke } from "@tauri-apps/api/core";

export type Result<T> = { success: true; data: T } | { success: false; error: unknown };

export const safeInvokeTauri = async <T>(
  command: string,
  args: Record<string, unknown>,
  errorMessage?: string,
): Promise<Result<T>> => {
  try {
    const data = await invoke<T>(command, args);
    return { success: true, data };
  } catch (error) {
    logger.error(errorMessage ?? `Failed to execute ${command}`, error);
    return { success: false, error };
  }
};

export const tauriApi = {
  createVault: (params: {
    path: string;
    name: string;
    password: number[];
    encryption: string;
    compression: string;
  }) => {
    return safeInvokeTauri<void>("create_vault", { params });
  },

  openVault: (params: { path: string; password: number[] }) => {
    return safeInvokeTauri<void>("open_vault", { params });
  },

  lockVault: () => {
    return safeInvokeTauri<void>("lock_vault", {});
  },

  browseFs: (params: { parentId: string }) => {
    return safeInvokeTauri<BrowseResult>("browse_fs", { params });
  },

  createFolder: (params: { parentId: string; name: string }) => {
    return safeInvokeTauri<string>("create_folder", { params });
  },

  deleteItem: (params: { id: string; itemType: ItemType }) => {
    return safeInvokeTauri<void>("delete_item", { params });
  },

  renameItem: (params: { id: string; itemType: ItemType; newName: string }) => {
    return safeInvokeTauri<void>("rename_item", { params });
  },

  uploadFile: (params: { parentId: string; sourcePath: string }) => {
    return safeInvokeTauri<void>("upload_file", { params });
  },

  uploadFolder: (params: { parentId: string; sourcePath: string }) => {
    return safeInvokeTauri<void>("upload_folder", { params });
  },

  readFileBytes: (params: { id: string }) => {
    return safeInvokeTauri<number[]>("read_file_bytes", { params });
  },

  isFile: (params: { path: string }) => {
    return safeInvokeTauri<boolean>("path_is_file", { params });
  },

  setFolderIcon: (params: { id: string; icon: string }) => {
    return safeInvokeTauri<void>("set_folder_icon", { params });
  },

  setFavourtieItem: (params: { id: string; itemType: ItemType; isFavourite: boolean }) => {
    return safeInvokeTauri<void>("set_favorite_item", { params });
  },

  exportFile: (params: { id: string; destinationPath: string }) => {
    return safeInvokeTauri<void>("export_file", { params });
  },

  exportFolder: (params: { id: string; destinationPath: string }) => {
    return safeInvokeTauri<void>("export_folder", { params });
  },
};
