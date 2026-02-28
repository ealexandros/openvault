import { cn } from "@/utils/cn";

type PasswordStrengthProps = {
  password: string;
  maxStrength?: number;
};

const getPasswordStrength = (pw: string) => {
  if (!pw) return 0;
  let s = 0;
  if (pw.length >= 8) s++;
  if (/[A-Z]/.test(pw)) s++;
  if (/[0-9]/.test(pw)) s++;
  if (/[^a-zA-Z0-9]/.test(pw)) s++;
  return s;
};

const getStrengthLabel = (s: number) => {
  switch (s) {
    case 1:
      return "Weak";
    case 2:
      return "Fair";
    case 3:
      return "Good";
    case 4:
      return "Strong";
    default:
      return "";
  }
};

const getStrengthColor = (s: number) => {
  switch (s) {
    case 1:
      return "bg-destructive";
    case 2:
      return "bg-orange-500";
    case 3:
      return "bg-yellow-500";
    case 4:
      return "bg-emerald-500";
    default:
      return "bg-muted/30";
  }
};

const getTextColor = (s: number) => {
  switch (s) {
    case 1:
      return "text-destructive";
    case 2:
      return "text-orange-500";
    case 3:
      return "text-yellow-500";
    case 4:
      return "text-emerald-500";
    default:
      return "";
  }
};

export const PasswordStrength = ({ password, maxStrength = 4 }: PasswordStrengthProps) => {
  const strength = getPasswordStrength(password);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between px-1">
        <span
          className={cn(
            "text-xs font-bold tracking-widest uppercase",
            getTextColor(strength),
          )}>
          {getStrengthLabel(strength)}
        </span>
      </div>
      <div className="flex h-1 gap-1.5 px-1">
        {Array.from({ length: maxStrength }).map((_, i) => (
          <div
            key={i}
            className={cn(
              "flex-1 rounded-full transition-all duration-500",
              i < strength ? getStrengthColor(strength) : "bg-muted/30",
            )}
          />
        ))}
      </div>
    </div>
  );
};
