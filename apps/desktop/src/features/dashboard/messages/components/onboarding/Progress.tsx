import { cn } from "@/utils/cn";

const STEPS = ["intro", "identity", "peers", "ready"];

export const Progress = ({ stepIndex }: { stepIndex: number }) => (
  <div className="flex justify-center gap-1.5 py-4">
    {STEPS.map((s, i) => (
      <div
        key={s}
        className={cn(
          "h-1 rounded-full transition-all duration-500",
          i <= stepIndex ? "w-8 bg-primary" : "w-1.5 bg-muted-foreground/20",
        )}
      />
    ))}
  </div>
);
