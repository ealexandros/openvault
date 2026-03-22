"use client";

import { tauriApi } from "@/libraries/tauri-api";
import { type MessageContact } from "@/types/messages";
import { join, tempDir } from "@tauri-apps/api/path";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useEffect, useState } from "react";
import { toast } from "sonner";

export type MessageMode = "encrypt" | "decrypt";
export type WorkMode = "text" | "file";
export type FileSource = "computer" | "vault";

export type FileInfo = {
  name: string;
  size: number;
  source: FileSource;
  id?: string;
  path?: string;
};

type UserPresence = "online" | "offline";

export type MessageUserProfile = {
  id: string;
  displayName: string;
  email: string;
  publicKey: string;
  status: UserPresence;
  trusted: boolean;
  importedAt: string;
  expiresAt: string;
  isExpired: boolean;
};

type MessageUserProfileImport = {
  name?: string;
  signingPubKey?: number[];
  ephemeralPubKey?: number[];
  expiresAt?: string | null;
};

const ROOT_FOLDER_ID = "00000000-0000-0000-0000-000000000000";

const toSafeFilename = (value: string) => {
  return value.trim().replace(/[\\/:*?"<>|]+/g, "_") || "openvault-file";
};

const createTempFilePath = async (fileName: string) => {
  const dir = await tempDir();

  const unique =
    // eslint-disable-next-line @typescript-eslint/strict-boolean-expressions, @typescript-eslint/no-unnecessary-condition
    typeof crypto !== "undefined" && crypto.randomUUID
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(16).slice(2)}`;

  return await join(dir, `openvault-${unique}-${toSafeFilename(fileName)}`);
};

const downloadJson = (filename: string, payload: unknown) => {
  const blob = new Blob([JSON.stringify(payload, null)], {
    type: "application/json",
  });

  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");

  a.href = url;
  a.download = filename;
  a.click();

  URL.revokeObjectURL(url);
};

export const useMessages = () => {
  // Variables
  const [mode, setMode] = useState<MessageMode>("encrypt");
  const [workMode, setWorkMode] = useState<WorkMode>("text");
  const [selectedFile, setSelectedFile] = useState<FileInfo | null>(null);
  const [fileDestination, setFileDestination] = useState<FileSource>("computer");
  const [messageInput, setMessageInput] = useState("");
  const [messageOutput, setMessageOutput] = useState("");
  const [transformError, setTransformError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSetup, setIsSetup] = useState(false);
  const [credentials, setCredentials] = useState<{
    name: string;
    signingPubKey: number[];
    ephemeralPubKey: number[];
    expiresAt: string | null;
  } | null>(null);

  const [users, setUsers] = useState<MessageContact[]>([]);
  const [selectedUserId, setSelectedUserId] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [importError, setImportError] = useState<string | null>(null);
  const [isVaultPickerOpen, setIsVaultPickerOpen] = useState(false);

  const filteredUsers = searchQuery.trim()
    ? users.filter(u => u.name.toLowerCase().includes(searchQuery.toLowerCase()))
    : users;

  const selectedUser = users.find(u => u.id === selectedUserId) ?? null;

  // Functions
  const refreshContacts = async () => {
    const result = await tauriApi.listContacts();
    if (result.success) setUsers(result.data);
  };

  const checkSetup = async () => {
    setIsLoading(true);

    const result = await tauriApi.getMessageCredentials();

    if (result.success && result.data) {
      setCredentials(result.data);
      setIsSetup(true);
      await refreshContacts();
    } else {
      setIsSetup(false);
    }

    setIsLoading(false);
  };

  useEffect(() => {
    void checkSetup();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const transformMessage = async () => {
    if (workMode === "file") {
      if (!selectedFile) {
        setTransformError("Please select a file first.");
        return;
      }

      if (!selectedUserId) {
        setTransformError("Please select a user first.");
        return;
      }

      const isEncrypt = mode === "encrypt";
      const outputFileName = isEncrypt
        ? `${selectedFile.name}.encrypted`
        : selectedFile.name.replace(".encrypted", "");

      const destinationPath =
        fileDestination === "computer"
          ? await save({
              title: isEncrypt ? "Save Encrypted File" : "Save Decrypted File",
              defaultPath: outputFileName,
            })
          : await createTempFilePath(outputFileName);

      if (fileDestination === "computer" && typeof destinationPath !== "string") {
        return; // User cancelled
      }

      const resolveSourcePath = async () => {
        if (selectedFile.source === "computer") {
          if (selectedFile.path === undefined) {
            toast.error("Source file must be from computer for this operation.");
            return null;
          }

          return selectedFile.path;
        }

        if (selectedFile.id == null) {
          toast.error("Vault file reference is missing.");
          return null;
        }

        const exportPath = await createTempFilePath(selectedFile.name);
        const exportResult = await tauriApi.exportFile({
          id: selectedFile.id,
          destinationPath: exportPath,
        });

        if (!exportResult.success) {
          toast.error("Failed to load file from vault.");
          return null;
        }

        return exportPath;
      };

      const finalDestinationPath =
        typeof destinationPath === "string" ? destinationPath : null;

      if (finalDestinationPath == null) {
        toast.error("Failed to resolve destination path.");
        return;
      }

      setIsLoading(true);

      try {
        const sourcePath = await resolveSourcePath();

        if (sourcePath == null) {
          return;
        }

        const result = isEncrypt
          ? await tauriApi.encryptFile({
              contactId: selectedUserId,
              sourcePath,
              destinationPath: finalDestinationPath,
            })
          : await tauriApi.decryptFile({
              contactId: selectedUserId,
              sourcePath,
              destinationPath: finalDestinationPath,
            });

        if (!result.success) {
          setTransformError(
            `Failed to ${isEncrypt ? "encrypt" : "decrypt"} file. ${
              isEncrypt ? "Ensure it is under 10MB." : "Invalid format or wrong key."
            }`,
          );
          return;
        }

        if (fileDestination === "vault") {
          const uploadResult = await tauriApi.uploadFile({
            parentId: ROOT_FOLDER_ID,
            sourcePath: finalDestinationPath,
          });

          if (!uploadResult.success) {
            toast.error("Failed to save file to vault.");
            return;
          }

          toast.success(`File ${isEncrypt ? "encrypted" : "decrypted"} and saved to vault`);
        } else {
          toast.success(`File ${isEncrypt ? "encrypted" : "decrypted"} successfully`);
        }

        setSelectedFile(null);
        setTransformError(null);
      } catch (_) {
        toast.error("Failed to process file");
      } finally {
        setIsLoading(false);
      }

      return;
    }

    if (!messageInput.trim()) {
      setMessageOutput("");
      setTransformError(null);
      return;
    }

    if (!selectedUserId) {
      setTransformError("Please select a user first.");
      return;
    }

    if (mode === "encrypt") {
      const result = await tauriApi.encryptMessage({
        id: selectedUserId,
        payload: messageInput,
      });

      if (!result.success) {
        setTransformError("Encryption failed.");
        return;
      }

      setMessageOutput(result.data.toString());
      setTransformError(null);
      return;
    }

    const payload = messageInput.trim();
    const result = await tauriApi.decryptMessage({ id: selectedUserId, payload });

    if (!result.success) {
      setTransformError("Decryption failed. Invalid payload or wrong key.");
      return;
    }

    setMessageOutput(result.data);
    setTransformError(null);
  };

  // Auto-decryption effect after transformMessage is defined
  useEffect(() => {
    if (mode === "decrypt") {
      if (workMode === "text" && messageInput.trim().length > 10) {
        const timeout = setTimeout(() => {
          void transformMessage();
        }, 300);
        return () => clearTimeout(timeout);
      } else if (workMode === "file" && selectedFile) {
        void transformMessage();
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [mode, workMode, messageInput, selectedFile]);

  const clearMessageFields = () => {
    setMessageInput("");
    setMessageOutput("");
    setTransformError(null);
  };

  const swapMessageFields = () => {
    setMessageInput(messageOutput);
    setMessageOutput(messageInput);
    setTransformError(null);
  };

  const selectComputerFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: mode === "encrypt" ? "Select File to Encrypt" : "Select File to Decrypt",
      });

      if (typeof selected === "string") {
        const fileName = selected.split(/[\/\\]/).pop();

        setSelectedFile({
          name: fileName ?? selected,
          size: 0, // We'll enforce the 10MB limit on the backend and return an error if too large
          source: "computer",
          path: selected,
        });
      }
    } catch (_) {
      toast.error("Failed to open file dialog");
    }
  };

  const importUserProfile = async (file: File) => {
    setImportError(null);

    const text = await file.text();

    let parsed: MessageUserProfileImport | null = null;

    try {
      parsed = JSON.parse(text) as MessageUserProfileImport;
    } catch {
      setImportError("Invalid JSON format.");
      return;
    }

    const name = parsed.name?.trim();
    const signingPubKey = parsed.signingPubKey;
    const ephemeralPubKey = parsed.ephemeralPubKey;

    if (name == null || signingPubKey == null || ephemeralPubKey == null) {
      setImportError("Profile missing required fields.");
      return;
    }

    const result = await tauriApi.addContact({
      name,
      signingPubKey,
      ephemeralPubKey,
    });

    if (!result.success) {
      setImportError("Failed to add contact.");
      return;
    }

    await refreshContacts();
    setSelectedUserId(result.data);

    toast.success(`Contact ${name} imported successfully`);
  };

  const exportSelectedUserProfile = () => {
    if (!selectedUser) return;

    const filename = selectedUser.name.toLowerCase().replace(/\s+/g, "-") + "-profile.ovp";

    downloadJson(filename, {
      name: selectedUser.name,
      signingPubKey: selectedUser.signingPubKey,
      ephemeralPubKey: selectedUser.ephemeralPubKey,
      expiresAt: selectedUser.expiresAt,
    });

    toast.success(`Profile for ${selectedUser.name} exported`);
  };

  const exportCurrentUserProfile = () => {
    if (!credentials) return;

    const filename = credentials.name.toLowerCase().replace(/\s+/g, "-") + "-profile.ovp";

    downloadJson(filename, {
      name: credentials.name,
      signingPubKey: credentials.signingPubKey,
      ephemeralPubKey: credentials.ephemeralPubKey,
      expiresAt: credentials.expiresAt,
    });

    toast.success("Your profile exported");
  };

  const completeOnboarding = async ({
    name,
    rotationMonths,
  }: {
    name: string;
    rotationMonths: number;
  }) => {
    const expiresAt = new Date();
    expiresAt.setMonth(expiresAt.getMonth() + rotationMonths);

    const result = await tauriApi.createMessageCredentials({
      name,
      expiresAt: expiresAt.toISOString(),
    });

    if (!result.success) {
      toast.error("Failed to create profile");
      return;
    }

    await checkSetup();
    toast.success("Profile setup complete");
  };

  const renameContact = async (id: string, newName: string) => {
    const result = await tauriApi.renameContact({ id, newName });

    if (!result.success) {
      toast.error("Failed to rename contact");
      return;
    }

    await refreshContacts();
    toast.success("Contact renamed");
  };

  const removeContact = async (id: string) => {
    const result = await tauriApi.removeContact({ id });

    if (!result.success) {
      toast.error("Failed to remove contact");
      return;
    }

    if (selectedUserId === id) setSelectedUserId("");

    await refreshContacts();
    toast.success("Contact removed");
  };

  const renewCredentials = async () => {
    const result = await tauriApi.renewMessageCredentials();

    if (!result.success) {
      toast.error("Failed to renew credentials");
      return;
    }

    await checkSetup();
    toast.success("Credentials renewed");
  };

  const resetCredentials = async () => {
    const result = await tauriApi.resetMessageCredentials();

    if (!result.success) {
      toast.error("Failed to reset credentials");
      return;
    }

    await checkSetup();
    toast.success("Credentials reset");
  };

  return {
    mode,
    workMode,
    selectedFile,
    fileDestination,
    messageInput,
    messageOutput,
    transformError,
    isLoading,
    isSetup,
    credentials,
    users,
    selectedUser,
    filteredUsers,
    selectedUserId,
    searchQuery,
    importError,
    isVaultPickerOpen,
    setMode,
    setWorkMode,
    setSelectedFile,
    setFileDestination,
    setMessageInput,
    setMessageOutput,
    setSelectedUserId,
    setSearchQuery,
    setIsVaultPickerOpen,
    transformMessage,
    clearMessageFields,
    swapMessageFields,
    selectComputerFile,
    importUserProfile,
    exportSelectedUserProfile,
    exportCurrentUserProfile,
    completeOnboarding,
    renameContact,
    removeContact,
    renewCredentials,
    resetCredentials,
  };
};
