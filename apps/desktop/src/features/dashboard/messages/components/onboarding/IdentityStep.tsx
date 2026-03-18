import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/shadcn/select";

type Props = {
  displayName: string;
  setDisplayName: (v: string) => void;
  rotationMonths: number;
  setRotationMonths: (v: number) => void;
};

export const IdentityStep = ({
  displayName,
  setDisplayName,
  rotationMonths,
  setRotationMonths,
}: Props) => (
  <div className="space-y-6">
    <div>
      <h3 className="text-3xl font-bold tracking-tight">Set Your Identity</h3>
      <p className="text-muted-foreground">This identity will be visible to peers.</p>
    </div>

    <div className="space-y-2">
      <Label className="text-xs font-bold tracking-widest text-muted-foreground/60 uppercase">
        Display Name
      </Label>
      <Input
        value={displayName}
        onChange={e => setDisplayName(e.target.value)}
        className="h-12"
      />
    </div>

    <div className="space-y-2">
      <Label className="text-xs font-bold tracking-widest text-muted-foreground/60 uppercase">
        Key Rotation Policy
      </Label>

      <Select
        value={rotationMonths.toString()}
        onValueChange={v => setRotationMonths(parseInt(v))}>
        <SelectTrigger className="h-12">
          <SelectValue />
        </SelectTrigger>

        <SelectContent>
          <SelectItem value="3">Every 3 months</SelectItem>
          <SelectItem value="6">Every 6 months</SelectItem>
          <SelectItem value="12">Every 12 months</SelectItem>
          <SelectItem value="24">Every 24 months</SelectItem>
        </SelectContent>
      </Select>
    </div>
  </div>
);
