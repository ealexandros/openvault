import { cn } from "@/utils/cn";

type PasswordStrengthProps = {
  password?: string;
  strengthScore?: number;
  maxStrength?: number;
};

const DEFAULT_STRENGTH = {
  label: "",
  color: "bg-muted/30",
  textColor: "",
};

const strengthMap = [
  DEFAULT_STRENGTH,
  { label: "Weak", color: "bg-destructive", textColor: "text-destructive" },
  { label: "Fair", color: "bg-orange-500", textColor: "text-orange-500" },
  { label: "Good", color: "bg-yellow-500", textColor: "text-yellow-500" },
  { label: "Strong", color: "bg-emerald-500", textColor: "text-emerald-500" },
];

export const getPasswordStrength = (password: string) => {
  if (!password) return 0;
  let score = 0;
  if (password.length >= 8) score++;
  if (/[A-Z]/.test(password)) score++;
  if (/[0-9]/.test(password)) score++;
  if (/[^a-zA-Z0-9]/.test(password)) score++;
  return score;
};

export const PasswordStrength = ({
  password,
  strengthScore,
  maxStrength = 4,
}: PasswordStrengthProps) => {
  const strength = strengthScore ?? (password != null ? getPasswordStrength(password) : 0);

  const { label, textColor, color } = strengthMap[strength] ?? DEFAULT_STRENGTH;

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between px-1">
        <span className={cn("text-xs font-bold tracking-widest uppercase", textColor)}>
          {label}
        </span>
      </div>
      <div className="flex h-1 gap-1.5 px-1">
        {Array.from({ length: maxStrength }).map((_, i) => (
          <div
            key={i}
            className={cn(
              "flex-1 rounded-full transition-all duration-500",
              i < strength ? color : "bg-muted/30",
            )}
          />
        ))}
      </div>
    </div>
  );
};
