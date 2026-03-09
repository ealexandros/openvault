"use client";

import { Input } from "@/components/ui/shadcn/input";
import { ScrollArea } from "@/components/ui/shadcn/scroll-area";
import { AnimatePresence } from "framer-motion";
import { Search } from "lucide-react";
import { MessageUserProfile } from "../../useMessagesPage";
import { SelectedUserDetails } from "./SelectedUserDetails";
import { UserList } from "./UserList";
import { UserListEmpty } from "./UserListEmpty";
import { UserSidebarActions } from "./UserSidebarActions";
import { UserSidebarHeader } from "./UserSidebarHeader";

type UserSidebarProps = {
  searchQuery: string;
  filteredUsers: MessageUserProfile[];
  selectedUserId: string;
  selectedUser: MessageUserProfile | null;
  importError: string | null;
  keyExpiresAt: string;
  openImportPicker: () => void;
  setSearchQuery: (query: string) => void;
  setSelectedUserId: (id: string) => void;
  exportSelectedUserProfile: () => void;
  exportCurrentUserProfile: () => void;
};

export const UserSidebar = ({
  searchQuery,
  filteredUsers,
  selectedUserId,
  selectedUser,
  importError,
  keyExpiresAt,
  openImportPicker,
  setSearchQuery,
  setSelectedUserId,
  exportSelectedUserProfile,
  exportCurrentUserProfile,
}: UserSidebarProps) => (
  <aside className="flex h-full w-full flex-col overflow-hidden bg-background">
    <UserSidebarHeader keyExpiresAt={keyExpiresAt} />

    <UserSidebarActions
      openImportPicker={openImportPicker}
      exportSelectedUserProfile={exportSelectedUserProfile}
      exportCurrentUserProfile={exportCurrentUserProfile}
      selectedUser={selectedUser}
      importError={importError}
    />

    <div className="flex min-h-0 flex-1 flex-col">
      <div className="px-6 pt-6 pb-2">
        <div className="relative">
          <Search className="absolute top-1/2 left-3 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
          <Input
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            placeholder="Search by name or email..."
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
            />
          </AnimatePresence>
          {filteredUsers.length === 0 && <UserListEmpty />}
        </div>
      </ScrollArea>
    </div>

    <SelectedUserDetails selectedUser={selectedUser} />
  </aside>
);
