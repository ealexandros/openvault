import { tauriApi } from "@/libraries/tauri-api";
import { type MessageContact } from "@/types/messages";
import { useEffect, useEffectEvent, useMemo, useState } from "react";
import { toast } from "sonner";

export const useContacts = () => {
  const [users, setUsers] = useState<MessageContact[]>([]);
  const [selectedUserId, setSelectedUserId] = useState<string>("");
  const [searchQuery, setSearchQuery] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const selectedUser = users.find(u => u.id === selectedUserId) ?? null;

  const filteredUsers = useMemo(() => {
    const query = searchQuery.trim().toLowerCase();
    if (!query) return users;
    return users.filter(u => u.name.toLowerCase().includes(query));
  }, [users, searchQuery]);

  const refreshContacts = async () => {
    setIsLoading(true);
    const result = await tauriApi.listContacts();
    setIsLoading(false);

    if (!result.success) {
      toast.error("Failed to load contacts");
      return;
    }

    setUsers(result.data);
  };

  const addContact = async (
    name: string,
    signingPubKey: number[],
    ephemeralPubKey: number[],
  ) => {
    const result = await tauriApi.addContact({
      name,
      signingPubKey,
      ephemeralPubKey,
    });

    if (!result.success) {
      toast.error("Failed to add contact");
      return null;
    }

    await refreshContacts();
    setSelectedUserId(result.data);

    toast.success(`Contact ${name} added`);
    return result.data;
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

    setSelectedUserId(prev => (prev === id ? "" : prev));

    await refreshContacts();
    toast.success("Contact removed");
  };

  const refreshContactsEvent = useEffectEvent(refreshContacts);

  useEffect(() => {
    void refreshContactsEvent();
  }, []);

  return {
    users,
    filteredUsers,
    selectedUser,
    selectedUserId,
    searchQuery,
    isLoading,
    setSelectedUserId,
    setSearchQuery,
    refreshContacts,
    addContact,
    renameContact,
    removeContact,
  };
};
