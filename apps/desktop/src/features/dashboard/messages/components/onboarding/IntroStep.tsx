import { ShieldCheck } from "lucide-react";

export const IntroStep = () => (
  <div className="space-y-8 text-center">
    <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
      <ShieldCheck className="h-10 w-10" />
    </div>

    <div className="space-y-2">
      <h2 className="text-3xl font-bold tracking-tight">Secure Messaging</h2>

      <p className="text-muted-foreground">Experience end-to-end encrypted communication.</p>
    </div>
  </div>
);
