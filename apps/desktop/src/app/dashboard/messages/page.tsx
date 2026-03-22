"use client";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/shadcn/resizable";
import { SidebarInset } from "@/components/ui/shadcn/sidebar";
import { useMessages } from "@/features/dashboard/messages";
import { MessageOnboarding } from "@/features/dashboard/messages/components/MessageOnboarding";
import { MessageWorkspace } from "@/features/dashboard/messages/components/MessageWorkspace";
import { UserSidebar } from "@/features/dashboard/messages/components/users-sidebar";
import { AnimatePresence, motion } from "framer-motion";
import { debounce } from "lodash-es";
import { ChangeEvent, useEffect, useMemo, useRef } from "react";

// @todo-now continue this..

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
    mode,
    workMode,
    selectedFile,
    fileDestination,
    messageInput,
    messageOutput,
    transformError,
    filteredUsers,
    selectedUser,
    selectedUserId,
    searchQuery,
    importError,
    isLoading,
    isSetup,
    credentials,
    setMode,
    setWorkMode,
    setSelectedFile,
    setFileDestination,
    setMessageInput,
    setSelectedUserId,
    setSearchQuery,
    transformMessage,
    clearMessageFields,
    swapMessageFields,
    selectComputerFile,
    importUserProfile,
    exportCurrentUserProfile,
    completeOnboarding,
    setIsVaultPickerOpen,
    renameContact,
    removeContact,
    renewCredentials,
    resetCredentials,
  } = useMessages();

  const transformMessageRef = useRef(transformMessage);

  useEffect(() => {
    transformMessageRef.current = transformMessage;
  }, [transformMessage]);

  const debouncedDecrypt = useMemo(
    () =>
      // eslint-disable-next-line react-hooks/refs
      typedDebounce(() => {
        const fn = transformMessageRef.current;
        void fn();
      }, 450),
    [],
  );

  const debouncedEncrypt = useMemo(
    () =>
      // eslint-disable-next-line react-hooks/refs
      typedDebounce(() => {
        const fn = transformMessageRef.current;
        void fn();
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
      void transformMessage();
      return;
    }

    debouncedDecrypt.flush();
  };

  // const handleSelectVaultFile = (file: FileItemResult) => {
  //   const displayName = file.extension ? `${file.name}.${file.extension}` : file.name;
  //   setSelectedFile({
  //     name: displayName,
  //     size: file.size,
  //     source: "vault",
  //     id: file.id,
  //   });
  // };

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
              currentUserName={credentials?.name ?? ""}
              onComplete={completeOnboarding}
              openImportPicker={openImportPicker}
            />
          </motion.div>
        ) : (
          <div key="workspace" className="flex h-full w-full">
            <ResizablePanelGroup orientation="horizontal" className="h-full">
              <ResizablePanel defaultSize="20%" minSize={300}>
                <UserSidebar
                  searchQuery={searchQuery}
                  filteredUsers={filteredUsers}
                  selectedUserId={selectedUserId}
                  importError={importError}
                  isLoading={isLoading}
                  openImportPicker={openImportPicker}
                  setSearchQuery={setSearchQuery}
                  setSelectedUserId={setSelectedUserId}
                  exportCurrentUserProfile={exportCurrentUserProfile}
                  renameContact={renameContact}
                  removeContact={removeContact}
                  renewCredentials={renewCredentials}
                  resetCredentials={resetCredentials}
                  updateProfile={completeOnboarding}
                  credentials={credentials}
                />
              </ResizablePanel>

              <ResizableHandle className="w-px bg-gray-100" />

              <ResizablePanel defaultSize="80%">
                <SidebarInset className="h-full w-full border-none">
                  <main className="scrollbar-none h-full overflow-y-auto">
                    <MessageWorkspace
                      mode={mode}
                      setMode={setMode}
                      workMode={workMode}
                      setWorkMode={setWorkMode}
                      selectedFile={selectedFile}
                      setSelectedFile={setSelectedFile}
                      fileDestination={fileDestination}
                      setFileDestination={setFileDestination}
                      messageInput={messageInput}
                      setMessageInput={setMessageInput}
                      messageOutput={messageOutput}
                      transformError={transformError}
                      clearMessageFields={clearMessageFields}
                      swapMessageFields={swapMessageFields}
                      handlePrimaryAction={handlePrimaryAction}
                      selectComputerFile={selectComputerFile}
                      openVaultPicker={() => setIsVaultPickerOpen(true)}
                      selectedUser={selectedUser}
                      users={filteredUsers}
                      setSelectedUserId={setSelectedUserId}
                    />
                  </main>
                </SidebarInset>
              </ResizablePanel>
            </ResizablePanelGroup>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
};

export default MessagesPage;
