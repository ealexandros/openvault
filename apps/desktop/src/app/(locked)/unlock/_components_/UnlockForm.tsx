import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { EyeIcon, EyeOffIcon } from "lucide-react";

type UnlockFormProps = {
  password: string;
  setPassword: (value: string) => void;
  showPassword: boolean;
  toggleShowPassword: () => void;
  onSubmit: () => void;
};

export const UnlockForm = ({
  password,
  setPassword,
  showPassword,
  toggleShowPassword,
  onSubmit,
}: UnlockFormProps) => (
  <form
    className="space-y-4"
    onSubmit={e => {
      e.preventDefault();
      onSubmit();
    }}>
    <div className="relative">
      <Input
        type={showPassword ? "text" : "password"}
        placeholder="Vault password..."
        value={password}
        onChange={e => setPassword(e.target.value)}
        className="h-12 rounded-xl border-border bg-muted/30 pr-12 focus:ring-primary/20"
        autoFocus
      />
      <Button
        type="button"
        variant="ghost"
        size="icon"
        onClick={toggleShowPassword}
        className="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground">
        {showPassword ? <EyeOffIcon className="h-4 w-4" /> : <EyeIcon className="h-4 w-4" />}
      </Button>
    </div>

    <Button
      type="submit"
      disabled={!password}
      className="h-12 w-full rounded-xl bg-white font-medium text-black transition-all hover:bg-zinc-200">
      Access Vault
    </Button>
  </form>
);
