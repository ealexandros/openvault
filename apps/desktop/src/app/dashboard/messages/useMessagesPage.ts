"use client";

import { useVault } from "@/context/VaultContext";
import { useCallback, useEffect, useMemo, useState } from "react";

export type MessageAlgorithm = "rsa-4096" | "x25519-aes-gcm" | "xchacha20-poly1305";
export type MessageMode = "encrypt" | "decrypt";

type UserPresence = "online" | "offline";

export type MessageUserProfile = {
  id: string;
  displayName: string;
  email: string;
  publicKey: string;
  status: UserPresence;
  trusted: boolean;
  lastSeen: string;
};

type ImportUserPayload = Partial<
  Pick<MessageUserProfile, "displayName" | "email" | "publicKey" | "status" | "trusted">
>;

const KEY_SIZE_BYTES = 512;

const ALGORITHM_OPTIONS: { value: MessageAlgorithm; label: string }[] = [
  { value: "rsa-4096", label: "RSA-4096" },
  { value: "x25519-aes-gcm", label: "X25519 + AES-256-GCM" },
  { value: "xchacha20-poly1305", label: "XChaCha20-Poly1305" },
];

const MOCK_USERS: MessageUserProfile[] = [
  {
    id: "user-1",
    displayName: "Maya Chen",
    email: "maya.chen@openvault.app",
    publicKey: "cHVibGljX2tleV9tYXlhX2NoZW5fMDE1ZV9hNGZiX2IxMGE5YjFhNjJjM2I3ZDQ5YzAy",
    status: "online",
    trusted: true,
    lastSeen: "Active now",
  },
  {
    id: "user-2",
    displayName: "Liam Carter",
    email: "liam.carter@openvault.app",
    publicKey: "cHVibGljX2tleV9saWFtX2NhcnRlcl8wN2FlX2UyNGQ4NmQxYTZjN2JiYjVlMTkxNmQ5",
    status: "offline",
    trusted: true,
    lastSeen: "Last seen 2h ago",
  },
  {
    id: "user-3",
    displayName: "Sofia Rivera",
    email: "sofia.rivera@openvault.app",
    publicKey: "cHVibGljX2tleV9zb2ZpYV9yaXZlcmFfMGMxYV9hZDE4N2Y2YjVhYjQ4Y2I1MjQxMjAw",
    status: "offline",
    trusted: false,
    lastSeen: "Last seen yesterday",
  },
];

const bytesToBase64 = (bytes: Uint8Array) => {
  let binary = "";
  const chunkSize = 0x8000;

  for (let index = 0; index < bytes.length; index += chunkSize) {
    const chunk = bytes.subarray(index, index + chunkSize);
    binary += String.fromCharCode(...chunk);
  }

  return btoa(binary);
};

const encodeTextToBase64 = (value: string) => {
  const bytes = new TextEncoder().encode(value);
  return bytesToBase64(bytes);
};

const decodeBase64ToText = (base64: string) => {
  const binary = atob(base64);
  const bytes = Uint8Array.from(binary, char => char.charCodeAt(0));

  return new TextDecoder().decode(bytes);
};

const randomBytes = (size: number) => {
  const bytes = new Uint8Array(size);
  crypto.getRandomValues(bytes);

  return bytes;
};

const downloadJson = (filename: string, payload: unknown) => {
  const blob = new Blob([JSON.stringify(payload, null, 2)], { type: "application/json" });
  const objectUrl = URL.createObjectURL(blob);
  const anchor = document.createElement("a");

  anchor.href = objectUrl;
  anchor.download = filename;
  anchor.click();

  URL.revokeObjectURL(objectUrl);
};

export const useMessagesPage = () => {
  const { vaultName } = useVault();

  const [algorithm, setAlgorithm] = useState<MessageAlgorithm>("rsa-4096");
  const [mode, setMode] = useState<MessageMode>("encrypt");
  const [messageInput, setMessageInput] = useState("");
  const [messageOutput, setMessageOutput] = useState("");
  const [transformError, setTransformError] = useState<string | null>(null);

  const [publicKey, setPublicKey] = useState("");
  const [privateKey, setPrivateKey] = useState("");
  const [isGeneratingKeys, setIsGeneratingKeys] = useState(true);

  const [users, setUsers] = useState<MessageUserProfile[]>(MOCK_USERS);
  const [selectedUserId, setSelectedUserId] = useState<string>(MOCK_USERS[0]?.id ?? "");
  const [searchQuery, setSearchQuery] = useState("");
  const [importError, setImportError] = useState<string | null>(null);

  const currentUserName = useMemo(() => vaultName ?? "Current User", [vaultName]);

  const generateKeyPair = useCallback(() => {
    setIsGeneratingKeys(true);

    const generatedPublicKey = bytesToBase64(randomBytes(KEY_SIZE_BYTES));
    const generatedPrivateKey = bytesToBase64(randomBytes(KEY_SIZE_BYTES));

    setPublicKey(generatedPublicKey);
    setPrivateKey(generatedPrivateKey);
    setIsGeneratingKeys(false);
  }, []);

  useEffect(() => {
    generateKeyPair();
  }, [generateKeyPair]);

  const transformMessage = useCallback(() => {
    if (messageInput.trim().length === 0) {
      setMessageOutput("");
      setTransformError(null);
      return;
    }

    try {
      const result =
        mode === "encrypt"
          ? encodeTextToBase64(messageInput)
          : decodeBase64ToText(messageInput.trim());
      setMessageOutput(result);
      setTransformError(null);
    } catch {
      setMessageOutput("");
      setTransformError("Input is not valid Base64.");
    }
  }, [messageInput, mode]);

  const clearMessageFields = useCallback(() => {
    setMessageInput("");
    setMessageOutput("");
    setTransformError(null);
  }, []);

  const filteredUsers = useMemo(() => {
    if (!searchQuery.trim()) {
      return users;
    }

    const query = searchQuery.toLowerCase();

    return users.filter(user => {
      if (user.displayName.toLowerCase().includes(query)) return true;
      return user.email.toLowerCase().includes(query);
    });
  }, [searchQuery, users]);

  const selectedUser = useMemo(
    () => users.find(user => user.id === selectedUserId) ?? null,
    [selectedUserId, users],
  );

  const importUserProfile = useCallback(async (file: File) => {
    setImportError(null);

    try {
      const raw = await file.text();
      const parsed = JSON.parse(raw) as ImportUserPayload;

      const displayName = parsed.displayName?.trim();
      const email = parsed.email?.trim();
      const importedPublicKey = parsed.publicKey?.trim();

      if (
        displayName == null ||
        displayName.length === 0 ||
        email == null ||
        email.length === 0 ||
        importedPublicKey == null ||
        importedPublicKey.length === 0
      ) {
        throw new Error("Missing required fields");
      }

      const importedUser: MessageUserProfile = {
        id: `user-${Date.now()}`,
        displayName,
        email,
        publicKey: importedPublicKey,
        status: parsed.status === "online" ? "online" : "offline",
        trusted: parsed.trusted ?? false,
        lastSeen: "Imported just now",
      };

      setUsers(previousUsers => [importedUser, ...previousUsers]);
      setSelectedUserId(importedUser.id);
    } catch {
      setImportError(
        "Unable to import profile. Expected a JSON file with displayName, email and publicKey.",
      );
    }
  }, []);

  const exportSelectedUserProfile = useCallback(() => {
    if (selectedUser == null) {
      return;
    }

    const filename = `${selectedUser.displayName.toLowerCase().replace(/\s+/g, "-")}-profile.json`;

    downloadJson(filename, {
      ...selectedUser,
      exportedAt: new Date().toISOString(),
      algorithm,
    });
  }, [algorithm, selectedUser]);

  const exportCurrentUserProfile = useCallback(() => {
    const filename = `${currentUserName.toLowerCase().replace(/\s+/g, "-")}-profile.json`;

    downloadJson(filename, {
      displayName: currentUserName,
      keySizeBytes: KEY_SIZE_BYTES,
      publicKey,
      privateKey,
      algorithm,
      exportedAt: new Date().toISOString(),
    });
  }, [algorithm, currentUserName, privateKey, publicKey]);

  return {
    algorithm,
    mode,
    messageInput,
    messageOutput,
    transformError,
    publicKey,
    privateKey,
    isGeneratingKeys,
    users,
    selectedUser,
    filteredUsers,
    selectedUserId,
    searchQuery,
    importError,
    currentUserName,
    keySizeBytes: KEY_SIZE_BYTES,
    algorithmOptions: ALGORITHM_OPTIONS,
    setAlgorithm,
    setMode,
    setMessageInput,
    setSelectedUserId,
    setSearchQuery,
    generateKeyPair,
    transformMessage,
    clearMessageFields,
    importUserProfile,
    exportSelectedUserProfile,
    exportCurrentUserProfile,
  };
};
