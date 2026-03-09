"use client";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/shadcn/resizable";
import { AnimatePresence, motion } from "framer-motion";
import { debounce } from "lodash-es";
import { ChangeEvent, useEffect, useMemo, useRef } from "react";
import { MessageOnboarding } from "./_components_/MessageOnboarding";
import { MessageWorkspace } from "./_components_/MessageWorkspace";
import { UserSidebar } from "./_components_/users-sidebar";
import { useMessagesPage } from "./useMessagesPage";

// @todo-soon continue this..

type DebouncedAction = {
  (): void;
  cancel: () => void;
  flush: () => void;
};

const typedDebounce = debounce as unknown as (
  callback: () => void,
  waitMs: number,
) => DebouncedAction;

const MessagesPage = () => {
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const {
    algorithm,
    mode,
    messageInput,
    messageOutput,
    transformError,
    filteredUsers,
    selectedUser,
    selectedUserId,
    searchQuery,
    importError,
    keyExpiresAt,
    algorithmOptions,
    isSetup,
    currentUserName,
    setAlgorithm,
    setMode,
    setMessageInput,
    setSelectedUserId,
    setSearchQuery,
    transformMessage,
    clearMessageFields,
    importUserProfile,
    exportSelectedUserProfile,
    exportCurrentUserProfile,
    completeOnboarding,
  } = useMessagesPage();

  const transformMessageRef = useRef(transformMessage);

  useEffect(() => {
    transformMessageRef.current = transformMessage;
  }, [transformMessage]);

  const debouncedDecrypt = useMemo(
    () =>
      // eslint-disable-next-line react-hooks/refs
      typedDebounce(() => {
        const fn = transformMessageRef.current;
        fn();
      }, 450),
    [],
  );

  const debouncedEncrypt = useMemo(
    () =>
      // eslint-disable-next-line react-hooks/refs
      typedDebounce(() => {
        const fn = transformMessageRef.current;
        fn();
      }, 450),
    [],
  );

  useEffect(() => {
    if (mode === "decrypt") {
      debouncedDecrypt();
    } else {
      debouncedEncrypt();
    }

    return () => {
      debouncedDecrypt.cancel();
      debouncedEncrypt.cancel();
    };
  }, [debouncedDecrypt, debouncedEncrypt, mode, messageInput]);

  const openImportPicker = () => {
    fileInputRef.current?.click();
  };

  const handleImportChange = async (event: ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;

    if (files == null || files.length === 0) {
      return;
    }

    for (const file of Array.from(files)) {
      await importUserProfile(file);
    }

    event.target.value = "";
  };

  const handlePrimaryAction = () => {
    if (mode === "encrypt") {
      transformMessage();
      return;
    }

    debouncedDecrypt.flush();
  };

  return (
    <div className="flex h-full w-full overflow-hidden bg-background">
      <input
        ref={fileInputRef}
        type="file"
        accept=".ovp"
        multiple
        className="hidden"
        onChange={handleImportChange}
      />
      <AnimatePresence mode="wait">
        {!isSetup ? (
          <motion.div
            key="onboarding"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="h-full w-full">
            <MessageOnboarding
              currentUserName={currentUserName}
              onComplete={completeOnboarding}
              openImportPicker={openImportPicker}
            />
          </motion.div>
        ) : (
          <motion.div
            key="workspace"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="flex h-full w-full">
            <ResizablePanelGroup orientation="horizontal" className="h-full">
              <ResizablePanel defaultSize="75%">
                <main className="scrollbar-none h-full overflow-y-auto p-10">
                  <MessageWorkspace
                    algorithm={algorithm}
                    algorithmOptions={algorithmOptions}
                    setAlgorithm={setAlgorithm}
                    mode={mode}
                    setMode={setMode}
                    messageInput={messageInput}
                    setMessageInput={setMessageInput}
                    messageOutput={messageOutput}
                    transformError={transformError}
                    clearMessageFields={clearMessageFields}
                    handlePrimaryAction={handlePrimaryAction}
                    selectedUser={selectedUser}
                  />
                </main>
              </ResizablePanel>

              <ResizableHandle withHandle />

              <ResizablePanel defaultSize="25%">
                <UserSidebar
                  searchQuery={searchQuery}
                  filteredUsers={filteredUsers}
                  selectedUserId={selectedUserId}
                  selectedUser={selectedUser}
                  importError={importError}
                  keyExpiresAt={keyExpiresAt}
                  openImportPicker={openImportPicker}
                  setSearchQuery={setSearchQuery}
                  setSelectedUserId={setSelectedUserId}
                  exportSelectedUserProfile={exportSelectedUserProfile}
                  exportCurrentUserProfile={exportCurrentUserProfile}
                />
              </ResizablePanel>
            </ResizablePanelGroup>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

export default MessagesPage;
