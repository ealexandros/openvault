"use client";

import { Badge } from "@/components/ui/shadcn/badge";
import { Button } from "@/components/ui/shadcn/button";
import { Label } from "@/components/ui/shadcn/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/shadcn/select";
import { Textarea } from "@/components/ui/shadcn/textarea";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/shadcn/tooltip";
import { type MessageContact } from "@/types/messages";
import { cn } from "@/utils/cn";
import { AnimatePresence, motion } from "framer-motion";
import {
  ArrowRightLeft,
  Check,
  Copy,
  Eraser,
  Lock,
  RefreshCw,
  ShieldCheck,
  Terminal,
  Unlock,
} from "lucide-react";
import { useState } from "react";
import { MessageAlgorithm, MessageMode } from "../useMessagesPage";

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
  swapMessageFields: () => void;
  handlePrimaryAction: () => void;
  selectedUser: MessageContact | null;
};

export const MessageWorkspace = ({
  algorithm,
  algorithmOptions,
  setAlgorithm,
  mode,
  setMode,
  messageInput,
  setMessageInput,
  messageOutput,
  transformError,
  clearMessageFields,
  swapMessageFields,
  handlePrimaryAction,
  selectedUser,
}: MessageWorkspaceProps) => {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = async () => {
    if (!messageOutput) return;
    await navigator.clipboard.writeText(messageOutput);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const toggleMode = () => {
    setMode(mode === "encrypt" ? "decrypt" : "encrypt");
    swapMessageFields();
  };

  return (
    <div className="mx-auto flex h-full max-w-4xl flex-col">
      <div className="mb-8 flex flex-col gap-6">
        <div className="flex items-center justify-between">
          <div className="space-y-1">
            <h2 className="text-2xl font-semibold tracking-tight">Message Workspace</h2>
            <p className="text-sm text-muted-foreground">
              {mode === "encrypt"
                ? "Securely encrypt a message for a trusted recipient."
                : "Decrypt a message using your private key."}
            </p>
          </div>

          <div className="flex items-center gap-2">
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={clearMessageFields}
                    className="h-9 w-9 text-muted-foreground hover:text-destructive">
                    <Eraser className="h-4 w-4" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Clear everything from memory</TooltipContent>
              </Tooltip>
            </TooltipProvider>

            <div className="h-4 w-px bg-border" />

            <Button
              variant="outline"
              size="sm"
              onClick={toggleMode}
              className="h-9 gap-2 font-medium">
              <ArrowRightLeft className="h-3.5 w-3.5" />
              <span>Switch to {mode === "encrypt" ? "Decrypt" : "Encrypt"}</span>
            </Button>
          </div>
        </div>

        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <div className="space-y-2">
            <Label className="text-sm font-bold tracking-wider text-muted-foreground uppercase">
              Selected Recipient
            </Label>
            <div
              className={cn(
                "flex h-12 items-center gap-3 rounded-lg border bg-muted/30 px-4",
                selectedUser ? "border-primary/20" : "border-border/50 opacity-60",
              )}>
              {selectedUser ? (
                <>
                  <div className="flex h-6 w-6 items-center justify-center rounded bg-primary/10 text-xs font-bold text-primary">
                    {selectedUser.name.charAt(0)}
                  </div>
                  <span className="text-sm font-medium">{selectedUser.name}</span>
                  {selectedUser.secure && (
                    <ShieldCheck className="ml-auto h-4 w-4 text-primary" />
                  )}
                </>
              ) : (
                <span className="text-sm text-muted-foreground italic">
                  No recipient selected
                </span>
              )}
            </div>
          </div>

          <div className="space-y-2">
            <Label className="text-sm font-bold tracking-wider text-muted-foreground uppercase">
              Encryption Protocol
            </Label>
            <Select
              value={algorithm}
              onValueChange={value => setAlgorithm(value as MessageAlgorithm)}>
              <SelectTrigger className="h-12 w-full bg-muted/30 px-4 text-sm">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {algorithmOptions.map(option => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>
      </div>

      <div className="grid flex-1 grid-cols-1 gap-8 lg:grid-cols-2">
        <div className="flex flex-col gap-3">
          <div className="flex items-center justify-between">
            <Label className="text-sm font-bold tracking-wider text-muted-foreground uppercase">
              {mode === "encrypt" ? "Plaintext Message" : "Encoded Ciphertext"}
            </Label>
            <Badge variant="outline" className="h-6 px-2 text-xs font-medium tracking-tight">
              {messageInput.length} chars
            </Badge>
          </div>
          <div className="group relative flex-1">
            <Textarea
              placeholder={
                !selectedUser
                  ? "Select a recipient first..."
                  : mode === "encrypt"
                    ? "Type your sensitive message here..."
                    : "Paste the base64 encoded sequence..."
              }
              disabled={!selectedUser}
              className="h-full min-h-[300px] resize-none rounded-xl border-border/50 bg-muted/10 p-4 font-mono text-sm leading-relaxed transition-all focus:border-primary/30 focus-visible:ring-0 focus-visible:ring-offset-0 disabled:cursor-not-allowed disabled:opacity-50"
              value={messageInput}
              onChange={e => setMessageInput(e.target.value)}
            />
            <div className="absolute right-3 bottom-3 opacity-0 transition-opacity group-focus-within:opacity-100">
              <Terminal className="h-4 w-4 text-muted-foreground" />
            </div>
          </div>
          <Button
            size="lg"
            onClick={handlePrimaryAction}
            disabled={
              !messageInput ||
              (mode === "encrypt" &&
                (!selectedUser ||
                  (selectedUser.expiresAt !== null &&
                    new Date(selectedUser.expiresAt) < new Date())))
            }
            variant="default"
            className="h-12 gap-2 rounded-lg text-sm font-semibold transition-all hover:shadow-lg hover:shadow-primary/20">
            {mode === "encrypt" ? (
              <>
                <Lock className="h-4 w-4" />
                <span>Encrypt Message</span>
              </>
            ) : (
              <>
                <Unlock className="h-4 w-4" />
                <span>Decrypt Sequence</span>
              </>
            )}
          </Button>
        </div>

        <div className="flex flex-col gap-3">
          <div className="flex items-center justify-between">
            <Label className="text-sm font-bold tracking-wider text-muted-foreground uppercase">
              {mode === "encrypt" ? "Cryptographic Output" : "Revealed Message"}
            </Label>
            {messageOutput && (
              <Badge variant="outline" className="h-6 px-2 text-xs font-medium tracking-tight">
                {messageOutput.length} chars
              </Badge>
            )}
          </div>
          <div className="group relative flex-1 overflow-hidden rounded-xl border border-border/50 bg-muted/5 p-1">
            <AnimatePresence mode="wait">
              {transformError !== null && transformError !== "" ? (
                <motion.div
                  key="error"
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  className="flex h-full flex-col items-center justify-center p-6 text-center">
                  <div className="mb-3 rounded-full bg-destructive/10 p-3 text-destructive">
                    <RefreshCw className="h-5 w-5" />
                  </div>
                  <h4 className="text-sm font-semibold">Transformation Failed</h4>
                  <p className="mt-1 text-xs text-muted-foreground">{transformError}</p>
                </motion.div>
              ) : messageOutput ? (
                <motion.div
                  key="output"
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="flex h-full flex-col">
                  <div className="scrollbar-none flex-1 overflow-y-auto p-3 font-mono text-sm leading-relaxed break-all whitespace-pre-wrap select-all">
                    {messageOutput}
                  </div>
                  <div className="border-t border-border/10 bg-muted/20 p-3">
                    <Button
                      variant="secondary"
                      className="h-10 w-full gap-2 rounded-lg text-xs font-semibold"
                      onClick={copyToClipboard}>
                      {copied ? (
                        <>
                          <Check className="h-3.5 w-3.5 text-primary" />
                          <span>Copied to Clipboard</span>
                        </>
                      ) : (
                        <>
                          <Copy className="h-3.5 w-3.5" />
                          <span>
                            {mode === "encrypt"
                              ? "Copy Encrypted Output"
                              : "Copy Revealed Text"}
                          </span>
                        </>
                      )}
                    </Button>
                  </div>
                </motion.div>
              ) : (
                <motion.div
                  key="empty"
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="flex h-full flex-col items-center justify-center p-6 text-center opacity-40">
                  <div className="mb-3 rounded-full bg-muted p-3">
                    <Lock className="h-5 w-5 text-muted-foreground" />
                  </div>
                  <p className="text-sm font-medium">Output will appear here</p>
                  <p className="mt-1 max-w-[180px] text-xs text-muted-foreground">
                    Waiting for input and action triggers...
                  </p>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      </div>
    </div>
  );
};
