"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { ScrollArea } from "@/components/ui/shadcn/scroll-area";
import { type MessageContact } from "@/types/messages";
import { AnimatePresence, motion } from "framer-motion";
import { Download, Search, UserPlus } from "lucide-react";
import { SelectedUserDetails } from "./SelectedUserDetails";
import { UserList } from "./UserList";
import { UserListEmpty } from "./UserListEmpty";
import { UserSidebarHeader } from "./UserSidebarHeader";

type UserSidebarProps = {
  searchQuery: string;
  filteredUsers: MessageContact[];
  selectedUserId: string;
  selectedUser: MessageContact | null;
  importError: string | null;
  isLoading: boolean;
  openImportPicker: () => void;
  setSearchQuery: (query: string) => void;
  setSelectedUserId: (id: string) => void;
  exportSelectedUserProfile: () => void;
  exportCurrentUserProfile: () => void;
  renameContact: (id: string, newName: string) => Promise<void>;
  removeContact: (id: string) => Promise<void>;
  renewCredentials: () => Promise<void>;
  resetCredentials: () => Promise<void>;
  updateProfile: (data: { name: string; rotationMonths: number }) => Promise<void>;
  credentials: {
    name: string;
    signingPubKey: number[];
    ephemeralPubKey: number[];
    expiresAt: string | null;
  } | null;
};

export const UserSidebar = ({
  searchQuery,
  filteredUsers,
  selectedUserId,
  selectedUser,
  importError,
  isLoading,
  openImportPicker,
  setSearchQuery,
  setSelectedUserId,
  exportSelectedUserProfile,
  exportCurrentUserProfile,
  renameContact,
  removeContact,
  renewCredentials,
  resetCredentials,
  updateProfile,
  credentials,
}: UserSidebarProps) => (
  <aside className="flex h-full w-full flex-col overflow-hidden bg-background">
    <UserSidebarHeader
      credentials={credentials}
      onRenew={renewCredentials}
      onReset={resetCredentials}
      onUpdate={updateProfile}
    />

    <div className="grid grid-cols-1 gap-2 px-6 pb-2">
      <Button
        variant="outline"
        size="sm"
        className="h-9 justify-start gap-2 rounded-lg"
        onClick={openImportPicker}>
        <UserPlus className="h-4 w-4 text-primary" />
        Import Profile
      </Button>

      <div className="flex gap-2">
        <Button
          variant="secondary"
          size="sm"
          className="flex-1 gap-2"
          onClick={exportCurrentUserProfile}>
          <Download className="h-4 w-4" />
          Mine
        </Button>

        <Button
          variant="secondary"
          size="sm"
          className="flex-1 gap-2"
          disabled={!Boolean(selectedUser)}
          onClick={exportSelectedUserProfile}>
          <Download className="h-4 w-4" />
          Selected
        </Button>
      </div>

      {importError != null && (
        <motion.div
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          className="rounded-lg border border-destructive/20 bg-destructive/10 p-2.5 text-[11px] text-destructive">
          {importError}
        </motion.div>
      )}
    </div>

    <div className="flex min-h-0 flex-1 flex-col">
      <div className="px-6 pt-4 pb-2">
        <div className="relative">
          <Search className="absolute top-1/2 left-3 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
          <Input
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            placeholder="Search by name..."
            className="h-9 rounded-lg pl-9 text-xs"
          />
        </div>
      </div>

      <ScrollArea className="flex-1 px-4">
        <div className="space-y-1.5 py-4">
          <AnimatePresence mode="popLayout">
            <UserList
              users={filteredUsers}
              selectedUserId={selectedUserId}
              setSelectedUserId={setSelectedUserId}
              isLoading={isLoading}
              onRename={renameContact}
              onDelete={removeContact}
            />
          </AnimatePresence>
          {!isLoading && filteredUsers.length === 0 && <UserListEmpty />}
        </div>
      </ScrollArea>
    </div>

    <SelectedUserDetails selectedUser={selectedUser} />
  </aside>
);
