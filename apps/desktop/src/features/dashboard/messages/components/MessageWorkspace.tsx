"use client";

import { Badge } from "@/components/ui/shadcn/badge";
import { Button } from "@/components/ui/shadcn/button";
import { Label } from "@/components/ui/shadcn/label";
import { Textarea } from "@/components/ui/shadcn/textarea";
import {
  type FileInfo,
  type FileSource,
  type MessageMode,
  type WorkMode,
} from "@/features/dashboard/messages/hooks/useMessages";
import { type MessageContact } from "@/types/messages";
import { cn } from "@/utils/cn";
import { AnimatePresence, LayoutGroup, motion } from "framer-motion";
import {
  Check,
  Copy,
  Eraser,
  File,
  FileText,
  HardDriveUpload,
  Lock,
  RefreshCw,
  ShieldCheck,
  Terminal,
  Unlock,
  Upload,
} from "lucide-react";
import { useState } from "react";

type MessageWorkspaceProps = {
  mode: MessageMode;
  setMode: (mode: MessageMode) => void;
  workMode: WorkMode;
  setWorkMode: (mode: WorkMode) => void;
  selectedFile: FileInfo | null;
  setSelectedFile: (file: FileInfo | null) => void;
  fileDestination: FileSource;
  setFileDestination: (dest: FileSource) => void;
  messageInput: string;
  setMessageInput: (value: string) => void;
  messageOutput: string;
  transformError: string | null;
  clearMessageFields: () => void;
  swapMessageFields: () => void;
  handlePrimaryAction: () => void;
  selectComputerFile: () => Promise<void>;
  openVaultPicker: () => void;
  selectedUser: MessageContact | null;
  users: MessageContact[];
  setSelectedUserId: (id: string) => void;
};

export const MessageWorkspace = ({
  mode,
  setMode,
  workMode,
  setWorkMode,
  selectedFile,
  setSelectedFile,
  fileDestination,
  setFileDestination,
  messageInput,
  setMessageInput,
  messageOutput,
  transformError,
  clearMessageFields,
  swapMessageFields,
  handlePrimaryAction,
  selectComputerFile,
  openVaultPicker,
  selectedUser,
  users,
  setSelectedUserId,
}: MessageWorkspaceProps) => {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = async () => {
    if (!messageOutput) return;
    await navigator.clipboard.writeText(messageOutput);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="mx-auto flex h-full max-w-5xl flex-col rounded-3xl border border-border/20 bg-background/50 p-6">
      <div className="mb-10 flex flex-col gap-6">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <div className="space-y-1">
            <h2 className="bg-linear-to-br from-foreground to-foreground/70 bg-clip-text text-2xl font-bold tracking-tight text-transparent">
              {mode === "encrypt" ? "Secure Transmission" : "Access Decryption"}
            </h2>
            <div className="flex items-center gap-2">
              <div
                className={cn(
                  "h-1.5 w-1.5 animate-pulse rounded-full",
                  mode === "encrypt" ? "bg-primary" : "bg-indigo-500",
                )}
              />
              <p className="text-xs font-medium tracking-widest text-muted-foreground uppercase">
                {mode === "encrypt"
                  ? "End-to-end encryption active"
                  : "Automated decryption engine"}
              </p>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div className="inline-flex items-center rounded-xl border border-border/40 bg-muted/30 p-1">
              <button
                onClick={() => {
                  if (mode !== "encrypt") {
                    setMode("encrypt");
                    swapMessageFields();
                  }
                }}
                className={cn(
                  "relative flex items-center gap-2 px-3 py-1.5 text-xs font-bold transition-all duration-200",
                  mode === "encrypt"
                    ? "text-primary-foreground"
                    : "text-muted-foreground hover:text-foreground",
                )}>
                {mode === "encrypt" && (
                  <motion.div
                    layoutId="active-mode"
                    className="absolute inset-0 rounded-lg bg-primary"
                  />
                )}
                <Lock className="relative z-10 h-3 w-3" />
                <span className="relative z-10">ENCRYPT</span>
              </button>
              <button
                onClick={() => {
                  if (mode !== "decrypt") {
                    setMode("decrypt");
                    swapMessageFields();
                  }
                }}
                className={cn(
                  "relative flex items-center gap-2 px-3 py-1.5 text-xs font-bold transition-all duration-200",
                  mode === "decrypt"
                    ? "text-primary-foreground"
                    : "text-muted-foreground hover:text-foreground",
                )}>
                {mode === "decrypt" && (
                  <motion.div
                    layoutId="active-mode"
                    className="absolute inset-0 rounded-lg bg-primary"
                  />
                )}
                <Unlock className="relative z-10 h-3 w-3" />
                <span className="relative z-10">DECRYPT</span>
              </button>
            </div>

            <div className="h-4 w-px bg-border/40" />

            <div className="inline-flex items-center rounded-xl border border-border/40 bg-muted/30 p-1">
              <button
                onClick={() => setWorkMode("text")}
                className={cn(
                  "relative flex items-center gap-2 px-3 py-1.5 text-xs font-bold transition-all duration-200",
                  workMode === "text"
                    ? "text-foreground"
                    : "text-muted-foreground hover:text-foreground",
                )}>
                {workMode === "text" && (
                  <motion.div
                    layoutId="active-workmode"
                    className="absolute inset-0 rounded-lg border border-border/10 bg-background"
                  />
                )}
                <FileText className="relative z-10 h-3 w-3" />
                <span className="relative z-10">TEXT</span>
              </button>
              <button
                onClick={() => setWorkMode("file")}
                className={cn(
                  "relative flex items-center gap-2 px-3 py-1.5 text-xs font-bold transition-all duration-200",
                  workMode === "file"
                    ? "text-foreground"
                    : "text-muted-foreground hover:text-foreground",
                )}>
                {workMode === "file" && (
                  <motion.div
                    layoutId="active-workmode"
                    className="absolute inset-0 rounded-lg border border-border/10 bg-background"
                  />
                )}
                <File className="relative z-10 h-3 w-3" />
                <span className="relative z-10">FILE</span>
              </button>
            </div>

            <Button
              variant="ghost"
              size="icon"
              onClick={clearMessageFields}
              className="h-9 w-9 rounded-xl border border-transparent bg-muted/30 text-muted-foreground hover:border-destructive/20 hover:text-destructive">
              <Eraser className="h-3.5 w-3.5" />
            </Button>
          </div>
        </div>
      </div>

      <LayoutGroup>
        <div className="mb-8 grid flex-1 grid-cols-1 gap-6 lg:grid-cols-2">
          <motion.div layout className="flex flex-col gap-3">
            <div className="flex items-center justify-between px-1">
              <Label className="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
                {workMode === "text"
                  ? mode === "encrypt"
                    ? "Input Plaintext"
                    : "Input Ciphertext"
                  : mode === "encrypt"
                    ? "Input Source File"
                    : "Input Encrypted File"}
              </Label>
              {workMode === "text" && (
                <Badge
                  variant="secondary"
                  className="h-5 rounded-md border-none bg-muted/50 px-1.5 text-[10px] font-bold tracking-tight">
                  {messageInput.length.toLocaleString()}{" "}
                  <span className="ml-0.5 text-muted-foreground/60">CHARS</span>
                </Badge>
              )}
            </div>

            {selectedUser ? (
              workMode === "text" ? (
                <div className="group relative flex-1">
                  <Textarea
                    placeholder={
                      mode === "encrypt"
                        ? "Enter the content you wish to protect..."
                        : "Paste your encrypted sequence here..."
                    }
                    className="h-full min-h-[350px] resize-none rounded-2xl border-2 border-dashed border-border/40 bg-background p-5 font-mono text-sm leading-relaxed shadow-none transition-all duration-300 hover:border-primary/20 focus:border-primary/50 focus-visible:ring-0 focus-visible:ring-offset-0"
                    value={messageInput}
                    onChange={e => setMessageInput(e.target.value)}
                  />
                  <div className="absolute right-4 bottom-4 opacity-10 transition-opacity group-focus-within:opacity-40">
                    <Terminal className="h-4 w-4 text-primary" />
                  </div>
                </div>
              ) : (
                <div className="flex flex-1 flex-col items-center justify-center gap-4 rounded-2xl border border-dashed border-border/80 bg-muted/10 p-8 text-center transition-colors duration-300 hover:bg-muted/20">
                  {selectedFile ? (
                    <motion.div
                      initial={{ opacity: 0, scale: 0.95 }}
                      animate={{ opacity: 1, scale: 1 }}
                      className="w-full space-y-5">
                      <div className="relative mx-auto flex h-20 w-20 items-center justify-center rounded-2xl border border-primary/10 bg-primary/5 text-primary">
                        <File className="h-10 w-10" />
                        <div className="absolute -right-1 -bottom-1 flex h-6 w-6 items-center justify-center rounded-full border-2 border-background bg-primary">
                          <Check className="h-3 w-3 text-primary-foreground" />
                        </div>
                      </div>
                      <div>
                        <p className="truncate px-4 text-lg font-bold tracking-tight">
                          {selectedFile.name}
                        </p>
                        <p className="mt-1 text-xs font-medium tracking-widest text-muted-foreground uppercase">
                          {(selectedFile.size / 1024 / 1024).toFixed(2)} MB • Source:{" "}
                          {selectedFile.source}
                        </p>
                      </div>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => setSelectedFile(null)}
                        className="h-8 rounded-lg px-4 text-[10px] font-bold tracking-wider uppercase">
                        Change File
                      </Button>
                    </motion.div>
                  ) : (
                    <>
                      <div className="space-y-3">
                        <div className="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl border bg-muted/50 text-muted-foreground">
                          <Upload className="h-6 w-6" />
                        </div>
                        <div className="space-y-1">
                          <h3 className="font-bold tracking-tight">Select Input File</h3>
                          <p className="mx-auto max-w-[220px] text-xs font-medium text-muted-foreground">
                            Choose a file from your computer or current vault session.
                          </p>
                        </div>
                      </div>
                      <div className="mt-4 flex gap-3">
                        <Button
                          variant="outline"
                          size="sm"
                          className="h-9 gap-2 rounded-xl border border-border/40 px-4 hover:bg-background"
                          onClick={selectComputerFile}>
                          <HardDriveUpload className="h-3.5 w-3.5 opacity-70" />
                          Computer
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          className="h-9 gap-2 rounded-xl border border-border/40 px-4 hover:bg-background"
                          onClick={openVaultPicker}>
                          <ShieldCheck className="h-3.5 w-3.5 opacity-70" />
                          Vault
                        </Button>
                      </div>
                    </>
                  )}
                </div>
              )
            ) : (
              <div className="flex min-h-[350px] flex-1 flex-col items-center justify-center rounded-2xl border border-dashed border-border/40 bg-muted/5 p-8 text-center">
                <div className="w-full max-w-[320px] space-y-6">
                  <div className="space-y-2">
                    <h3 className="text-lg font-bold tracking-tight">Select Recipient</h3>
                    <p className="px-4 text-xs font-medium text-muted-foreground">
                      Choose a trusted contact from your list to begin secure transmission.
                    </p>
                  </div>
                  <div className="scrollbar-thin grid max-h-[220px] grid-cols-1 gap-2 overflow-y-auto pr-1">
                    {users.length > 0 ? (
                      users.map(user => (
                        <button
                          key={user.id}
                          onClick={() => setSelectedUserId(user.id)}
                          className="group flex items-center gap-3 rounded-xl border border-transparent p-3 transition-all duration-200 hover:border-border/40 hover:bg-background active:scale-[0.98]">
                          <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-primary/10 bg-primary/10 text-[10px] font-bold text-primary transition-colors group-hover:bg-primary group-hover:text-primary-foreground">
                            {user.name.charAt(0).toUpperCase()}
                          </div>
                          <div className="overflow-hidden text-left">
                            <p className="truncate text-sm font-semibold">{user.name}</p>
                            {user.secure && (
                              <span className="mt-0.5 block text-[9px] font-bold tracking-widest text-primary uppercase">
                                Secure Link
                              </span>
                            )}
                          </div>
                          <div className="ml-auto opacity-0 transition-opacity group-hover:opacity-100">
                            <Check className="h-3.5 w-3.5 text-primary" />
                          </div>
                        </button>
                      ))
                    ) : (
                      <div className="py-8 opacity-40">
                        <p className="text-xs font-medium italic">No contacts found</p>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            )}
          </motion.div>

          {/* Transfer Animation / Middle Action */}
          <div className="flex items-center justify-center py-2 lg:hidden">
            <div className="h-10 w-px bg-linear-to-b from-transparent via-border to-transparent" />
          </div>

          <motion.div layout className="flex flex-col gap-3">
            <div className="flex items-center justify-between px-1">
              <Label className="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
                {workMode === "text"
                  ? mode === "encrypt"
                    ? "Cryptographic Result"
                    : "Recovered Content"
                  : mode === "encrypt"
                    ? "Encrypted Destination"
                    : "Decrypted Destination"}
              </Label>
              {workMode === "text" && messageOutput && (
                <Badge
                  variant="outline"
                  className="h-5 rounded-md border-primary/20 bg-primary/5 px-1.5 text-[10px] font-bold tracking-tight text-primary">
                  {messageOutput.length.toLocaleString()}{" "}
                  <span className="ml-0.5 opacity-60">CHARS</span>
                </Badge>
              )}
            </div>

            {workMode === "text" ? (
              <div className="group relative flex-1 overflow-hidden rounded-2xl border border-border/40 bg-muted/5">
                <AnimatePresence mode="wait">
                  {transformError !== null && transformError !== "" ? (
                    <motion.div
                      key="error"
                      initial={{ opacity: 0, scale: 0.98 }}
                      animate={{ opacity: 1, scale: 1 }}
                      exit={{ opacity: 0, scale: 0.98 }}
                      className="flex h-full flex-col items-center justify-center p-8 text-center">
                      <div className="mb-4 rounded-2xl border border-destructive/10 bg-destructive/10 p-4 text-destructive">
                        <RefreshCw className="h-6 w-6" />
                      </div>
                      <h4 className="text-sm font-bold tracking-tight">Operation Failed</h4>
                      <p className="mt-2 max-w-[200px] text-xs leading-relaxed font-medium text-muted-foreground">
                        {transformError}
                      </p>
                    </motion.div>
                  ) : messageOutput ? (
                    <motion.div
                      key="output"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      exit={{ opacity: 0 }}
                      className="flex h-full flex-col">
                      <div className="scrollbar-none flex-1 overflow-y-auto p-5 font-mono text-sm leading-relaxed break-all whitespace-pre-wrap select-all">
                        {messageOutput}
                      </div>
                      <div className="border-t border-border/10 bg-muted/10 p-4">
                        <Button
                          variant={copied ? "default" : "secondary"}
                          className={cn(
                            "h-11 w-full gap-2 rounded-xl text-xs font-bold tracking-widest uppercase transition-all duration-300",
                            copied ? "bg-primary" : "border border-border/40 bg-background",
                          )}
                          onClick={copyToClipboard}>
                          {copied ? (
                            <>
                              <Check className="h-4 w-4" />
                              <span>Copied successfully</span>
                            </>
                          ) : (
                            <>
                              <Copy className="h-4 w-4 opacity-70" />
                              <span>
                                {mode === "encrypt" ? "Copy Encryption" : "Copy Content"}
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
                      className="flex h-full flex-col items-center justify-center p-8 text-center">
                      <div className="mb-4 rounded-2xl border border-border/50 bg-muted/30 p-4 opacity-40">
                        <RefreshCw className="h-6 w-6 rotate-12 text-muted-foreground" />
                      </div>
                      <p className="text-sm font-bold tracking-tight opacity-40">
                        Awaiting Action
                      </p>
                      <p className="mt-1 max-w-[200px] text-xs font-medium text-muted-foreground opacity-30">
                        The cryptographic result will be processed and displayed here.
                      </p>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            ) : (
              <div className="flex flex-1 flex-col justify-center rounded-2xl border border-border/40 bg-muted/5 p-8">
                <div className="mx-auto flex w-full max-w-[300px] flex-col gap-4">
                  <Label className="mb-1 text-center text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
                    Target Storage
                  </Label>
                  <button
                    onClick={() => setFileDestination("computer")}
                    className={cn(
                      "group flex transform items-center gap-4 rounded-2xl border p-4 transition-all duration-300",
                      fileDestination === "computer"
                        ? "-translate-y-0.5 border-primary bg-primary/5"
                        : "border-border/40 bg-background hover:bg-muted/40",
                    )}>
                    <div
                      className={cn(
                        "flex h-11 w-11 shrink-0 items-center justify-center rounded-xl transition-colors duration-300",
                        fileDestination === "computer"
                          ? "bg-primary text-primary-foreground"
                          : "bg-muted text-muted-foreground",
                      )}>
                      <HardDriveUpload className="h-5 w-5" />
                    </div>
                    <div className="text-left">
                      <p className="text-sm font-bold tracking-tight">Save to Local</p>
                      <p className="text-[10px] font-medium tracking-widest text-muted-foreground uppercase">
                        Internal Storage
                      </p>
                    </div>
                    {fileDestination === "computer" && (
                      <Check className="ml-auto h-4 w-4 text-primary" />
                    )}
                  </button>

                  <button
                    onClick={() => setFileDestination("vault")}
                    className={cn(
                      "group flex transform items-center gap-4 rounded-2xl border p-4 transition-all duration-300",
                      fileDestination === "vault"
                        ? "-translate-y-0.5 border-primary bg-primary/5"
                        : "border-border/40 bg-background hover:bg-muted/40",
                    )}>
                    <div
                      className={cn(
                        "flex h-11 w-11 shrink-0 items-center justify-center rounded-xl transition-colors duration-300",
                        fileDestination === "vault"
                          ? "bg-primary text-primary-foreground"
                          : "bg-muted text-muted-foreground",
                      )}>
                      <ShieldCheck className="h-5 w-5" />
                    </div>
                    <div className="text-left">
                      <p className="text-sm font-bold tracking-tight">Save to Vault</p>
                      <p className="text-[10px] font-medium tracking-widest text-muted-foreground uppercase">
                        Encrypted Cloud
                      </p>
                    </div>
                    {fileDestination === "vault" && (
                      <Check className="ml-auto h-4 w-4 text-primary" />
                    )}
                  </button>
                </div>
              </div>
            )}
          </motion.div>
        </div>

        {/* Primary Action Button - Minimal and Only for Encryption */}
        <div className="flex justify-center">
          {mode === "encrypt" && (
            <Button
              size="lg"
              onClick={handlePrimaryAction}
              disabled={
                workMode === "text"
                  ? !messageInput ||
                    (selectedUser?.expiresAt !== null &&
                      new Date(selectedUser!.expiresAt) < new Date())
                  : !selectedFile ||
                    (selectedUser?.expiresAt !== null &&
                      new Date(selectedUser!.expiresAt) < new Date())
              }
              variant="default"
              className={cn(
                "h-14 min-w-[280px] gap-3 rounded-2xl text-sm font-bold tracking-widest uppercase transition-all duration-500",
                "bg-primary hover:bg-primary/90",
              )}>
              <Lock className="h-5 w-5" />
              <span>Process encryption</span>
            </Button>
          )}
        </div>
      </LayoutGroup>
    </div>
  );
};
