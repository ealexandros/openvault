import { User } from "lucide-react";

export function ReadyStep({
  displayName,
  rotationMonths,
}: {
  displayName: string;
  rotationMonths: number;
}) {
  return (
    <div className="space-y-8 text-center">
      <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary text-primary-foreground">
        <User className="h-10 w-10" />
      </div>

      <h3 className="text-3xl font-bold tracking-tight">Everything is Ready</h3>

      <div className="divide-y rounded-lg border bg-muted/20 text-sm">
        <div className="flex justify-between p-5">
          <span>Identity</span>
          <span className="font-semibold">{displayName}</span>
        </div>

        <div className="flex justify-between p-5">
          <span>Rotation</span>
          <span className="font-semibold">Every {rotationMonths} months</span>
        </div>
      </div>
    </div>
  );
}
