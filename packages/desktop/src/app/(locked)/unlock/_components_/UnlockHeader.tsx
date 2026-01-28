import { Button } from "@/components/ui/shadcn/button";
import { ArrowLeftIcon, LockIcon } from "lucide-react";

type UnlockHeaderProps = {
  onBack: () => void;
  selectedPath: string;
};

export const UnlockHeader = ({ onBack, selectedPath }: UnlockHeaderProps) => (
  <div className="space-y-8">
    <Button
      variant="ghost"
      size="sm"
      onClick={onBack}
      className="-ml-2 text-muted-foreground hover:text-foreground">
      <ArrowLeftIcon className="mr-2 h-4 w-4" />
      Back
    </Button>

    <div className="space-y-6">
      <div className="space-y-2 text-center">
        <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-2xl border border-primary/20 bg-primary/10">
          <LockIcon className="h-8 w-8 text-primary" />
        </div>
        <h1 className="text-2xl font-semibold tracking-tight">Enter Password</h1>
        <p className="truncate px-4 text-sm text-muted-foreground">{selectedPath}</p>
      </div>
    </div>
  </div>
);
