import { logger } from "@/libraries/logger";
import { invoke } from "@tauri-apps/api/core";

export type FolderItem = {
  id: string;
  name: string;
  item_count: number;
};

export type FileItem = {
  id: string;
  name: string;
  size: number;
  extension: string;
};

export type BrowseResponse = {
  folders: FolderItem[];
  files: FileItem[];
};

type TauriCommands = {
  create_vault: {
    args: { path: string; name: string; password: string };
    return: void;
  };
  open_vault: {
    args: { path: string; password: string };
    return: void;
  };
  browse_vault: {
    args: { parentId: string };
    return: BrowseResponse;
  };
  create_folder: {
    args: { parentId: string; name: string };
    return: string;
  };
  delete_item: {
    args: { id: string; itemType: "file" | "folder" };
    return: void;
  };
  rename_item: {
    args: { id: string; itemType: "file" | "folder"; newName: string };
    return: void;
  };
  upload_file: {
    args: { parentId: string; sourcePath: string };
    return: void;
  };
  get_file_content: {
    args: { id: string };
    return: number[] | null;
  };
};

export const tauriApi = {
  invoke: async <K extends keyof TauriCommands>(
    command: K,
    args: TauriCommands[K]["args"],
  ): Promise<TauriCommands[K]["return"]> => {
    return invoke(command, args);
  },

  safeInvoke: async <K extends keyof TauriCommands>(
    command: K,
    args: TauriCommands[K]["args"],
    errorMessage?: string,
  ): Promise<{
    data: TauriCommands[K]["return"] | null;
    error: unknown;
  }> => {
    try {
      const data: TauriCommands[K]["return"] | null = await invoke(command, args);
      return { data, error: null };
    } catch (err) {
      logger.error(errorMessage ?? `Failed to execute ${command}`, err);
      return { data: null, error: err };
    }
  },
};
