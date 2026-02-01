import { Button } from "@/components/ui/shadcn/button";

type EncryptionProgressProps = {
  onCancel: () => void;
};

export const EncryptionProgress = ({ onCancel }: EncryptionProgressProps) => (
  <div
    className="fixed inset-0 z-50 flex animate-in flex-col items-center justify-center bg-background/80 backdrop-blur-2xl duration-500 fade-in"
    style={{ colorScheme: "dark" }}>
    <div className="w-full max-w-sm space-y-12 text-center">
      <div className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight text-foreground">
          Encrypting in process
        </h2>
        <p className="text-sm text-muted-foreground">
          This might take several minutes depending on the size of your vault.
        </p>
      </div>

      <div className="relative h-1 w-full overflow-hidden rounded-full bg-muted/30">
        <div className="animate-loader-slide absolute top-0 bottom-0 w-24 rounded-full bg-primary" />
      </div>

      <Button
        variant="ghost"
        size="sm"
        onClick={onCancel}
        className="text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground">
        Cancel
      </Button>
    </div>
  </div>
);
