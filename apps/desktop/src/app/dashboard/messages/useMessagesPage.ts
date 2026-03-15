"use client";

import { tauriApi } from "@/libraries/tauri-api";
import { type MessageContact } from "@/types/messages";
import { useEffect, useState } from "react";
import { toast } from "sonner";

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

const ALGORITHM_OPTIONS: {
  value: MessageAlgorithm;
  label: string;
}[] = [{ value: "xchacha20-poly1305", label: "XChaCha20-Poly1305" }];

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

export const useMessagesPage = () => {
  const [algorithm, setAlgorithm] = useState<MessageAlgorithm>("xchacha20-poly1305");
  const [mode, setMode] = useState<MessageMode>("encrypt");
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

  async function refreshContacts() {
    const result = await tauriApi.listContacts();
    if (result.success) setUsers(result.data);
  }

  async function checkSetup() {
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
  }

  useEffect(() => {
    void checkSetup();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function transformMessage() {
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
  }

  function clearMessageFields() {
    setMessageInput("");
    setMessageOutput("");
    setTransformError(null);
  }

  function swapMessageFields() {
    setMessageInput(messageOutput);
    setMessageOutput(messageInput);
    setTransformError(null);
  }

  const filteredUsers = searchQuery.trim()
    ? users.filter(u => u.name.toLowerCase().includes(searchQuery.toLowerCase()))
    : users;

  const selectedUser = users.find(u => u.id === selectedUserId) ?? null;

  async function importUserProfile(file: File) {
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
  }

  function exportSelectedUserProfile() {
    if (!selectedUser) return;

    const filename = selectedUser.name.toLowerCase().replace(/\s+/g, "-") + "-profile.ovp";

    downloadJson(filename, {
      name: selectedUser.name,
      signingPubKey: selectedUser.signingPubKey,
      ephemeralPubKey: selectedUser.ephemeralPubKey,
      expiresAt: selectedUser.expiresAt,
    });

    toast.success(`Profile for ${selectedUser.name} exported`);
  }

  function exportCurrentUserProfile() {
    if (!credentials) return;

    const filename = credentials.name.toLowerCase().replace(/\s+/g, "-") + "-profile.ovp";

    downloadJson(filename, {
      name: credentials.name,
      signingPubKey: credentials.signingPubKey,
      ephemeralPubKey: credentials.ephemeralPubKey,
      expiresAt: credentials.expiresAt,
    });

    toast.success("Your profile exported");
  }

  async function completeOnboarding({
    name,
    rotationMonths,
  }: {
    name: string;
    rotationMonths: number;
  }) {
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
  }

  async function renameContact(id: string, newName: string) {
    const result = await tauriApi.renameContact({ id, newName });

    if (!result.success) {
      toast.error("Failed to rename contact");
      return;
    }

    await refreshContacts();
    toast.success("Contact renamed");
  }

  async function removeContact(id: string) {
    const result = await tauriApi.removeContact({ id });

    if (!result.success) {
      toast.error("Failed to remove contact");
      return;
    }

    if (selectedUserId === id) setSelectedUserId("");

    await refreshContacts();
    toast.success("Contact removed");
  }

  async function renewCredentials() {
    const result = await tauriApi.renewMessageCredentials();

    if (!result.success) {
      toast.error("Failed to renew credentials");
      return;
    }

    await checkSetup();
    toast.success("Credentials renewed");
  }

  async function resetCredentials() {
    const result = await tauriApi.resetMessageCredentials();

    if (!result.success) {
      toast.error("Failed to reset credentials");
      return;
    }

    await checkSetup();
    toast.success("Credentials reset");
  }

  return {
    algorithm,
    mode,
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
    algorithmOptions: ALGORITHM_OPTIONS,
    setAlgorithm,
    setMode,
    setMessageInput,
    setMessageOutput,
    setSelectedUserId,
    setSearchQuery,
    transformMessage,
    clearMessageFields,
    swapMessageFields,
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
