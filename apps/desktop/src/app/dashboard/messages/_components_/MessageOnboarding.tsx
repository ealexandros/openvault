"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/shadcn/select";
import { AnimatePresence, motion } from "framer-motion";
import { ArrowRight, ChevronLeft, ShieldCheck, User, UserPlus, Users } from "lucide-react";
import { useState } from "react";

type MessageOnboardingProps = {
  currentUserName: string;
  onComplete: (data: { name: string; rotationMonths: number }) => void;
  openImportPicker: () => void;
};

type Step = "intro" | "identity" | "peers" | "ready";

export const MessageOnboarding = ({
  currentUserName,
  onComplete,
  openImportPicker,
}: MessageOnboardingProps) => {
  const [step, setStep] = useState<Step>("intro");
  const [displayName, setDisplayName] = useState(currentUserName);
  const [rotationMonths, setRotationMonths] = useState(12);

  const handleNext = () => {
    if (step === "intro") setStep("identity");
    else if (step === "identity") setStep("peers");
    else if (step === "peers") setStep("ready");
    else onComplete({ name: displayName, rotationMonths });
  };

  const handleBack = () => {
    if (step === "identity") setStep("intro");
    else if (step === "peers") setStep("identity");
    else if (step === "ready") setStep("peers");
  };

  return (
    <div className="flex h-full w-full items-center justify-center bg-linear-to-b from-background to-muted/20 p-6">
      <div className="w-full max-w-lg">
        <AnimatePresence mode="wait">
          {step === "intro" && (
            <motion.div
              key="intro"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              className="space-y-8 text-center">
              <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
                <ShieldCheck className="h-10 w-10" />
              </div>
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tight">Secure Messaging</h2>
                <p className="text-muted-foreground">
                  Experience end-to-end encrypted communication. Share credentials, messages,
                  and digital signatures with ultimate privacy.
                </p>
              </div>
              <Button
                size="lg"
                onClick={handleNext}
                className="h-12 w-full gap-2 text-sm font-semibold">
                Start Setup
                <ArrowRight className="h-4 w-4" />
              </Button>
            </motion.div>
          )}

          {step === "identity" && (
            <motion.div
              key="identity"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              className="space-y-8">
              <div className="space-y-2">
                <h3 className="text-2xl font-bold tracking-tight">Set Your Identity</h3>
                <p className="text-sm text-muted-foreground">
                  This identity will be visible to your trusted peers.
                </p>
              </div>

              <div className="space-y-4 rounded-2xl border border-border/50 bg-muted/30 p-6">
                <div className="space-y-2">
                  <Label
                    htmlFor="displayName"
                    className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
                    Display Name
                  </Label>
                  <Input
                    id="displayName"
                    value={displayName}
                    onChange={e => setDisplayName(e.target.value)}
                    placeholder="Alice..."
                    className="h-11 bg-background"
                  />
                </div>

                <div className="space-y-2">
                  <Label className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
                    Key Rotation Policy
                  </Label>
                  <Select
                    value={rotationMonths.toString()}
                    onValueChange={v => setRotationMonths(parseInt(v))}>
                    <SelectTrigger className="h-11 bg-background">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="3">Every 3 months (Highly Secure)</SelectItem>
                      <SelectItem value="6">Every 6 months (Balanced)</SelectItem>
                      <SelectItem value="12">Every 12 months (Standard)</SelectItem>
                      <SelectItem value="24">Every 24 months (Long Term)</SelectItem>
                    </SelectContent>
                  </Select>
                  <p className="text-[10px] text-muted-foreground">
                    * Automated key rotation helps minimize the impact of long-term credential
                    leakage.
                  </p>
                </div>
              </div>

              <div className="flex flex-col gap-2">
                <Button
                  onClick={handleNext}
                  disabled={!displayName.trim()}
                  className="h-12 w-full gap-2 text-sm font-semibold">
                  Next Step
                  <ArrowRight className="size-4" />
                </Button>
                <Button
                  variant="ghost"
                  onClick={handleBack}
                  className="h-12 w-full gap-2 text-sm text-muted-foreground">
                  Back
                </Button>
              </div>
            </motion.div>
          )}

          {step === "peers" && (
            <motion.div
              key="peers"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              className="space-y-8">
              <div className="space-y-2 text-center">
                <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-secondary/50 text-secondary-foreground">
                  <Users className="h-8 w-8" />
                </div>
                <h3 className="text-2xl font-bold tracking-tight">Connect with Peers</h3>
                <p className="mx-auto max-w-[320px] text-sm text-muted-foreground">
                  Import public keys from your friends to start exchange messages securely.
                </p>
              </div>

              <div className="flex flex-col gap-3">
                <Button
                  variant="outline"
                  onClick={openImportPicker}
                  className="h-16 justify-start gap-4 rounded-2xl border-dashed px-6 hover:border-primary/50 hover:bg-primary/5">
                  <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 text-primary">
                    <UserPlus className="h-5 w-5" />
                  </div>
                  <div className="text-left">
                    <p className="text-sm font-semibold">Import peer identities</p>
                    <p className="text-xs text-muted-foreground">Choose .ovp profiles</p>
                  </div>
                </Button>

                <div className="py-4 text-center">
                  <button
                    onClick={handleNext}
                    className="text-xs font-semibold text-muted-foreground underline underline-offset-4 hover:text-primary">
                    Skip for now
                  </button>
                </div>
              </div>

              <Button
                variant="ghost"
                onClick={handleBack}
                className="mx-auto w-fit gap-2 text-muted-foreground">
                <ChevronLeft className="h-4 w-4" />
                Back to Identity
              </Button>
            </motion.div>
          )}

          {step === "ready" && (
            <motion.div
              key="ready"
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0, scale: 0.9 }}
              className="space-y-8 text-center">
              <div className="relative mx-auto size-20">
                <div className="absolute inset-0 animate-ping rounded-full bg-primary/20" />
                <div className="relative flex h-full w-full items-center justify-center rounded-full bg-primary text-white">
                  <User className="size-8" />
                </div>
              </div>

              <div className="space-y-2">
                <h3 className="text-2xl font-bold tracking-tight">Everything is Ready</h3>
                <p className="text-base text-muted-foreground">
                  Your cryptographic workspace has been initialized. All communication at this
                  point will be end-to-end encrypted.
                </p>
              </div>

              <div className="space-y-4">
                <div className="divide-y divide-border/50 rounded-lg border border-border/50 bg-muted/20 text-sm">
                  <div className="flex items-center justify-between p-5">
                    <span className="font-medium text-muted-foreground">Identity</span>
                    <span className="font-semibold">{displayName}</span>
                  </div>
                  <div className="flex items-center justify-between p-5">
                    <span className="font-medium text-muted-foreground">Rotation</span>
                    <span className="font-semibold">Every {rotationMonths} months</span>
                  </div>
                </div>

                <Button
                  size="lg"
                  onClick={() => onComplete({ name: displayName, rotationMonths })}
                  className="group h-12 w-full gap-1 text-sm">
                  Enter Workspace
                  <ArrowRight className="size-4 transition-transform group-hover:translate-x-0.5" />
                </Button>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};
