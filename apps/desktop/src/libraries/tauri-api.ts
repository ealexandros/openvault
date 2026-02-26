import { logger } from "@/libraries/logger";
import { invoke } from "@tauri-apps/api/core";

export type FilesystemItem = {
  id: string;
  name: string;
  type: "file" | "folder";
  details?: string;
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
    args: { parentId: string | null };
    return: FilesystemItem[];
  };
  create_folder: {
    args: { parentId: string | null; name: string };
    return: string;
  };
  delete_item: {
    args: { id: string; itemType: "file" | "folder" };
    return: void;
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
