"use client";

import { AnimatePresence } from "framer-motion";
import { useState } from "react";
import { Footer } from "./Footer";
import { IdentityStep } from "./IdentityStep";
import { IntroStep } from "./IntroStep";
import { PeersStep } from "./PeersStep";
import { Progress } from "./Progress";
import { ReadyStep } from "./ReadyStep";
import { StepContainer } from "./StepContainer";

export type Step = "intro" | "identity" | "peers" | "ready";

const STEPS: Step[] = ["intro", "identity", "peers", "ready"];

type Props = {
  currentUserName: string;
  onComplete: (data: { name: string; rotationMonths: number }) => void;
  openImportPicker: () => void;
};

export function MessageOnboarding({ currentUserName, onComplete, openImportPicker }: Props) {
  const [stepIndex, setStepIndex] = useState(0);
  const [displayName, setDisplayName] = useState(currentUserName);
  const [rotationMonths, setRotationMonths] = useState(12);

  const step = STEPS[stepIndex];

  const next = () => {
    if (step === "ready") {
      onComplete({ name: displayName, rotationMonths });
      return;
    }

    setStepIndex(s => s + 1);
  };

  const back = () => setStepIndex(s => Math.max(0, s - 1));

  return (
    <div className="flex h-full w-full items-center justify-center bg-linear-to-b from-background to-muted/20 p-6">
      <div className="w-full max-w-lg space-y-8">
        <AnimatePresence mode="popLayout" initial={false}>
          {step === "intro" && (
            <StepContainer key="intro">
              <IntroStep />
            </StepContainer>
          )}

          {step === "identity" && (
            <StepContainer key="identity">
              <IdentityStep
                displayName={displayName}
                setDisplayName={setDisplayName}
                rotationMonths={rotationMonths}
                setRotationMonths={setRotationMonths}
              />
            </StepContainer>
          )}

          {step === "peers" && (
            <StepContainer key="peers">
              <PeersStep openImportPicker={openImportPicker} />
            </StepContainer>
          )}

          {step === "ready" && (
            <StepContainer key="ready">
              <ReadyStep displayName={displayName} rotationMonths={rotationMonths} />
            </StepContainer>
          )}
        </AnimatePresence>

        <Progress stepIndex={stepIndex} />

        <Footer step={step} next={next} back={back} displayName={displayName} />
      </div>
    </div>
  );
}
