import { Button } from "@/components/ui/shadcn/button";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInput,
} from "@/components/ui/shadcn/sidebar";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/shadcn/tooltip";
import { type MessageContact } from "@/types/messages";
import { motion } from "framer-motion";
import { Plus } from "lucide-react";
import { UserList } from "./UserList";
import { UserSidebarHeader } from "./UserSidebarHeader";

type UserSidebarProps = {
  searchQuery: string;
  filteredUsers: MessageContact[];
  selectedUserId: string;
  importError: string | null;
  isLoading: boolean;
  credentials: {
    name: string;
    signingPubKey: number[];
    ephemeralPubKey: number[];
    expiresAt: string | null;
  } | null;
  openImportPicker: () => void;
  setSearchQuery: (query: string) => void;
  setSelectedUserId: (id: string) => void;
  exportCurrentUserProfile: () => void;
  renameContact: (id: string, newName: string) => Promise<void>;
  removeContact: (id: string) => Promise<void>;
  renewCredentials: () => Promise<void>;
  resetCredentials: () => Promise<void>;
  updateProfile: (data: { name: string; rotationMonths: number }) => Promise<void>;
};

export const UserSidebar = ({
  searchQuery,
  filteredUsers,
  selectedUserId,
  importError,
  isLoading,
  credentials,
  openImportPicker,
  setSearchQuery,
  setSelectedUserId,
  exportCurrentUserProfile,
  renameContact,
  removeContact,
  renewCredentials,
  resetCredentials,
  updateProfile,
}: UserSidebarProps) => (
  <Sidebar variant="floating" collapsible="none" className="w-full border-r-0 bg-background">
    <SidebarHeader className="gap-2 px-4 pt-6 pb-2">
      <UserSidebarHeader
        credentials={credentials}
        onRenew={renewCredentials}
        onReset={resetCredentials}
        onUpdate={updateProfile}
        onExport={exportCurrentUserProfile}
      />

      <div className="px-1 group-data-[collapsible=icon]:hidden"></div>

      {importError != null && (
        <motion.div
          initial={{ opacity: 0, y: -5 }}
          animate={{ opacity: 1, y: 0 }}
          className="mx-1 mt-2 rounded-md border border-destructive/20 bg-destructive/5 p-3 text-xs leading-tight text-destructive">
          {importError}
        </motion.div>
      )}
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup className="h-full p-0">
        <div className="flex items-center justify-between">
          <SidebarGroupLabel className="h-auto font-semibold tracking-tight text-foreground/80">
            Imported Users
          </SidebarGroupLabel>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="size-7 hover:bg-sidebar-accent"
                  onClick={openImportPicker}>
                  <Plus className="size-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Import Profile</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>

        <SidebarGroupContent className="h-full">
          <SidebarInput
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            placeholder="Search contacts..."
            className="h-9 bg-muted/40 text-sm focus-visible:ring-1"
          />
          <UserList
            users={filteredUsers}
            selectedUserId={selectedUserId}
            setSelectedUserId={setSelectedUserId}
            isLoading={isLoading}
            onRename={renameContact}
            onDelete={removeContact}
          />
          {/* <UserListEmpty /> */}
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
);
