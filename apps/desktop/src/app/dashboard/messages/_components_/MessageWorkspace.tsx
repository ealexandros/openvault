"use client";

import { MessageAlgorithm, MessageMode, MessageUserProfile } from "../useMessagesPage";

type MessageWorkspaceProps = {
  algorithm: MessageAlgorithm;
  algorithmOptions: { value: MessageAlgorithm; label: string }[];
  setAlgorithm: (value: MessageAlgorithm) => void;
  mode: MessageMode;
  setMode: (mode: MessageMode) => void;
  messageInput: string;
  setMessageInput: (value: string) => void;
  messageOutput: string;
  transformError: string | null;
  clearMessageFields: () => void;
  handlePrimaryAction: () => void;
  selectedUser: MessageUserProfile | null;
};

export const MessageWorkspace = ({}: MessageWorkspaceProps) => {
  return <div>Messages</div>;
};
