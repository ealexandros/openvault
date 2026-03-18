import { tauriApi } from "@/libraries/tauri-api";
import { open } from "@tauri-apps/plugin-dialog";
import { toast } from "sonner";

type UseFileUploaderProps = {
  folderId?: string;
  refresh: () => Promise<void>;
};

export const useFileUploader = ({ folderId, refresh }: UseFileUploaderProps) => {
  const uploadPath = async (path: string) => {
    const isFile = await tauriApi.isFile({ path });

    if (isFile.success && !isFile.data) {
      return await tauriApi.uploadFolder({ parentId: folderId, sourcePath: path });
    }

    return await tauriApi.uploadFile({ parentId: folderId, sourcePath: path });
  };

  const uploadPaths = async (paths: string[]) => {
    if (paths.length === 0) return;

    const toastId = toast.loading("Uploading files");
    const results = await Promise.all(paths.map(path => uploadPath(path)));
    toast.dismiss(toastId);

    if (results.some(result => result.success)) {
      await refresh();
    }
  };

  const uploadFile = async () => {
    const selected = await open({
      multiple: true,
      directory: false,
    });

    if (selected != null) await uploadPaths(selected);
  };

  const uploadFolder = async () => {
    const selected = await open({
      multiple: true,
      directory: true,
    });

    if (selected != null) await uploadPaths(selected);
  };

  return {
    uploadPaths,
    uploadFile,
    uploadFolder,
  };
};
