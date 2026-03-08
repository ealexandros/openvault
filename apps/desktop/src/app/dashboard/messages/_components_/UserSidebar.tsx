"use client";

import { Badge } from "@/components/ui/shadcn/badge";
import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { ScrollArea } from "@/components/ui/shadcn/scroll-area";
import { cn } from "@/utils/cn";
import { AnimatePresence, motion } from "framer-motion";
import { Clock, Download, Search, ShieldCheck, UserPlus } from "lucide-react";
import { MessageUserProfile } from "../useMessagesPage";

type UserSidebarProps = {
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  filteredUsers: MessageUserProfile[];
  selectedUserId: string;
  setSelectedUserId: (id: string) => void;
  selectedUser: MessageUserProfile | null;
  openImportPicker: () => void;
  exportSelectedUserProfile: () => void;
  exportCurrentUserProfile: () => void;
  importError: string | null;
  keyExpiresAt: string;
};

export const UserSidebar = ({
  searchQuery,
  setSearchQuery,
  filteredUsers,
  selectedUserId,
  setSelectedUserId,
  selectedUser,
  openImportPicker,
  exportSelectedUserProfile,
  exportCurrentUserProfile,
  importError,
  keyExpiresAt,
}: UserSidebarProps) => {
  return (
    <aside className="flex h-full w-full flex-col overflow-hidden bg-background">
      <div className="flex flex-col gap-4 border-b border-border bg-muted/20 p-6">
        <div className="space-y-1">
          <h3 className="text-lg font-semibold tracking-tight">User Profiles</h3>
          <p className="text-xs text-muted-foreground">
            Manage public keys and identities for secure messaging.
          </p>
          {keyExpiresAt && (
            <div className="flex items-center gap-1.5 text-[10px] font-bold text-amber-600/80 uppercase">
              <Clock className="h-2.5 w-2.5" />
              <span>Key expires: {new Date(keyExpiresAt).toLocaleDateString()}</span>
            </div>
          )}
        </div>

        <div className="grid grid-cols-1 gap-2">
          <Button
            variant="outline"
            size="sm"
            className="h-9 justify-start gap-2 rounded-lg"
            onClick={openImportPicker}>
            <UserPlus className="h-4 w-4 text-primary" />
            <span>Import Profile</span>
          </Button>
          <div className="flex gap-2">
            <Button
              variant="secondary"
              size="sm"
              className="h-9 flex-1 gap-2 rounded-lg"
              onClick={exportCurrentUserProfile}>
              <Download className="h-4 w-4" />
              <span>Mine</span>
            </Button>
            <Button
              variant="secondary"
              size="sm"
              className="h-9 flex-1 gap-2 rounded-lg"
              onClick={exportSelectedUserProfile}
              disabled={selectedUser == null}>
              <Download className="h-4 w-4" />
              <span>Selected</span>
            </Button>
          </div>
        </div>

        {importError != null && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            className="rounded-lg border border-destructive/20 bg-destructive/10 p-2.5 text-[11px] leading-relaxed text-destructive">
            {importError}
          </motion.div>
        )}
      </div>

      <div className="flex min-h-0 flex-1 flex-col">
        <div className="px-6 pt-6 pb-2">
          <div className="relative">
            <Search className="absolute top-1/2 left-3 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              value={searchQuery}
              onChange={event => setSearchQuery(event.target.value)}
              placeholder="Search by name or email..."
              className="h-9 rounded-lg border-border pl-9 text-xs transition-shadow focus-visible:ring-1"
            />
          </div>
        </div>

        <ScrollArea className="flex-1 px-4">
          <div className="space-y-1.5 py-4">
            <AnimatePresence mode="popLayout">
              {filteredUsers.map(user => (
                <motion.button
                  layout
                  key={user.id}
                  type="button"
                  onClick={() => setSelectedUserId(user.id)}
                  className={cn(
                    "group relative w-full rounded-xl border p-3 text-left transition-all duration-200",
                    selectedUserId === user.id
                      ? "border-primary/50 bg-primary/5 shadow-sm shadow-primary/10"
                      : "border-transparent hover:border-border hover:bg-muted/50",
                  )}>
                  <div className="flex items-start gap-3">
                    <div
                      className={cn(
                        "flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border text-xs font-semibold",
                        selectedUserId === user.id
                          ? "border-primary/20 bg-primary/10 text-primary"
                          : "border-border bg-muted/30 text-muted-foreground",
                      )}>
                      {user.displayName.charAt(0).toUpperCase()}
                    </div>
                    <div className="flex-1 overflow-hidden">
                      <div className="flex items-center justify-between gap-2">
                        <p className="truncate text-sm leading-none font-medium">
                          {user.displayName}
                        </p>
                        {user.trusted && !user.isExpired && (
                          <ShieldCheck className="h-3 w-3 shrink-0 text-primary" />
                        )}
                      </div>
                      <p className="mt-1 truncate text-xs text-muted-foreground">
                        {user.email}
                      </p>
                    </div>
                  </div>
                </motion.button>
              ))}
            </AnimatePresence>

            {filteredUsers.length === 0 && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex flex-col items-center justify-center rounded-xl border border-dashed border-border py-12 text-center">
                <div className="rounded-full bg-muted p-3">
                  <Search className="h-5 w-5 text-muted-foreground" />
                </div>
                <p className="mt-2 text-sm font-medium">No results found</p>
                <p className="text-xs text-muted-foreground">Try a different search term</p>
              </motion.div>
            )}
          </div>
        </ScrollArea>
      </div>

      {selectedUser != null && (
        <div className="border-t border-border bg-muted/10 p-6">
          <div className="space-y-4">
            <div className="flex items-start justify-between">
              <div className="space-y-1">
                <h4 className="text-sm font-semibold">{selectedUser.displayName}</h4>
                <div className="flex flex-col gap-1">
                  <div className="flex items-center gap-2 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
                    <Clock className="h-2.5 w-2.5" />
                    <span>
                      Imported: {new Date(selectedUser.importedAt).toLocaleDateString()}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
                    <ShieldCheck className="h-2.5 w-2.5" />
                    <span>
                      Expires: {new Date(selectedUser.expiresAt).toLocaleDateString()}
                    </span>
                  </div>
                </div>
              </div>
              <Badge
                variant={
                  selectedUser.trusted && !selectedUser.isExpired ? "secondary" : "outline"
                }
                className={cn(
                  "h-5 px-1.5 text-[10px] font-bold tracking-wider uppercase",
                  selectedUser.trusted && !selectedUser.isExpired
                    ? "border-primary/20 bg-primary/10 text-primary"
                    : "border-destructive/20 bg-destructive/5 text-destructive",
                )}>
                {selectedUser.isExpired
                  ? "Expired"
                  : selectedUser.trusted
                    ? "Trusted"
                    : "Unverified"}
              </Badge>
            </div>

            <div className="space-y-3">
              <div className="space-y-1">
                <Label className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
                  Public Key Fingerprint
                </Label>
                <div className="rounded-lg border border-border/50 bg-muted/50 p-2 font-mono text-[10px] break-all text-muted-foreground select-all">
                  {selectedUser.publicKey.substring(0, 32)}...
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </aside>
  );
};
