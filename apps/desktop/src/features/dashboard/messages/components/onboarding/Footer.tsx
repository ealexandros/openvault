import { Button } from "@/components/ui/shadcn/button";
import { ArrowRight } from "lucide-react";
import { Step } from "./MessageOnboarding";

type Props = {
  step: Step;
  next: () => void;
  back: () => void;
  displayName: string;
};

export const Footer = ({ step, next, back, displayName }: Props) => {
  if (step === "intro")
    return (
      <Button onClick={next} className="h-12 w-full gap-2">
        Start Setup
        <ArrowRight className="size-4" />
      </Button>
    );

  if (step === "identity")
    return (
      <>
        <Button onClick={next} disabled={!displayName.trim()} className="h-12 w-full">
          Next Step
        </Button>

        <Button variant="ghost" onClick={back} className="w-full">
          Back
        </Button>
      </>
    );

  if (step === "peers")
    return (
      <>
        <Button onClick={next} className="h-12 w-full">
          Skip for now
        </Button>

        <Button variant="ghost" onClick={back} className="w-full">
          Back
        </Button>
      </>
    );

  return (
    <Button onClick={next} className="h-12 w-full gap-2">
      Enter Workspace
      <ArrowRight className="size-4" />
    </Button>
  );
};
