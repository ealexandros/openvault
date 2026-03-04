"use client";

import { debounce } from "lodash-es";
import { ChangeEvent, useEffect, useMemo, useRef } from "react";
import { MessageWorkspace } from "./_components_/MessageWorkspace";
import { UserSidebar } from "./_components_/UserSidebar";
import { useMessagesPage } from "./useMessagesPage";

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
    algorithmOptions,
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

  useEffect(() => {
    if (mode !== "decrypt") {
      debouncedDecrypt.cancel();
      return;
    }

    debouncedDecrypt();

    return () => {
      debouncedDecrypt.cancel();
    };
  }, [debouncedDecrypt, mode, messageInput]);

  const openImportPicker = () => {
    fileInputRef.current?.click();
  };

  const handleImportChange = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];

    if (file == null) {
      return;
    }

    await importUserProfile(file);
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
    <div className="flex h-full w-full gap-8 overflow-hidden bg-background">
      <input
        ref={fileInputRef}
        type="file"
        accept="application/json"
        className="hidden"
        onChange={handleImportChange}
      />

      <main className="scrollbar-none flex-1 overflow-y-auto p-10">
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

      <div className="w-sm shrink-0 border-l border-border bg-muted/30">
        <UserSidebar
          searchQuery={searchQuery}
          setSearchQuery={setSearchQuery}
          filteredUsers={filteredUsers}
          selectedUserId={selectedUserId}
          setSelectedUserId={setSelectedUserId}
          selectedUser={selectedUser}
          openImportPicker={openImportPicker}
          exportSelectedUserProfile={exportSelectedUserProfile}
          exportCurrentUserProfile={exportCurrentUserProfile}
          importError={importError}
        />
      </div>
    </div>
  );
};

export default MessagesPage;
