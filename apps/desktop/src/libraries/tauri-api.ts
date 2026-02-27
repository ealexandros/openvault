import { logger } from "@/libraries/logger";
import { type BrowseResult } from "@/types/filesystem";
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
  createVault: (params: { path: string; name: string; password: string }) => {
    return safeInvokeTauri<void>("create_vault", { params });
  },

  openVault: (params: { path: string; password: string }) => {
    return safeInvokeTauri<void>("open_vault", { params });
  },

  browseVault: (params: { parentId: string }) => {
    return safeInvokeTauri<BrowseResult>("browse_vault", { params });
  },

  createFolder: (params: { parentId: string; name: string }) => {
    return safeInvokeTauri<string>("create_folder", { params });
  },

  deleteItem: (params: { id: string; itemType: "file" | "folder" }) => {
    return safeInvokeTauri<void>("delete_item", { params });
  },

  renameItem: (params: { id: string; itemType: "file" | "folder"; newName: string }) => {
    return safeInvokeTauri<void>("rename_item", { params });
  },

  uploadFile: (params: { parentId: string; sourcePath: string }) => {
    return safeInvokeTauri<void>("upload_file", { params });
  },

  getFileContent: (params: { id: string }) => {
    return safeInvokeTauri<number[] | null>("get_file_content", { params });
  },

  checkPathIsFile: (params: { path: string }) => {
    return safeInvokeTauri<boolean>("path_is_file", { params });
  },
};
