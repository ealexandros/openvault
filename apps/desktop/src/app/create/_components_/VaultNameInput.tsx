import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";

type VaultNameInputProps = {
  value: string;
  error?: string;
  touched?: boolean;
  onChange: (e: React.ChangeEvent<unknown>) => void;
  onBlur: (e: unknown) => void;
};

export const VaultNameInput = ({
  value,
  error,
  touched,
  onChange,
  onBlur,
}: VaultNameInputProps) => (
  <div className="space-y-2">
    <Label className="ml-1 text-[11px] font-bold tracking-widest text-muted-foreground uppercase">
      Vault Name
    </Label>
    <Input
      name="name"
      placeholder="e.g. My Secure Projects"
      value={value}
      onChange={onChange}
      onBlur={onBlur}
      className={cn(
        "h-12 bg-muted/30 px-4 focus:ring-primary/20",
        touched === true && error != null ? "border-red-500/50" : "border-border",
      )}
    />
    {touched === true && error != null && (
      <p className="ml-1 text-[11px] text-destructive">{error}</p>
    )}
  </div>
);
