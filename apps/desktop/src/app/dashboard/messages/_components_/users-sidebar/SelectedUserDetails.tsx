"use client";

import { Badge } from "@/components/ui/shadcn/badge";
import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";
import { Clock, ShieldCheck } from "lucide-react";
import { MessageUserProfile } from "../../useMessagesPage";

type Props = {
  selectedUser: MessageUserProfile | null;
};

export const SelectedUserDetails = ({ selectedUser }: Props) => {
  if (selectedUser == null) return null;

  const isTrusted = selectedUser.trusted && !selectedUser.isExpired;

  return (
    <div className="border-t border-border bg-muted/10 p-6">
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-start justify-between">
          <div className="space-y-1">
            <h4 className="text-sm font-semibold">{selectedUser.displayName}</h4>

            <div className="flex flex-col gap-1">
              <div className="flex items-center gap-2 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
                <Clock className="size-2.5" />
                <span>Imported: {new Date(selectedUser.importedAt).toLocaleDateString()}</span>
              </div>

              <div className="flex items-center gap-2 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
                <ShieldCheck className="h-2.5 w-2.5" />
                <span>Expires: {new Date(selectedUser.expiresAt).toLocaleDateString()}</span>
              </div>
            </div>
          </div>

          {/* Trust Badge */}
          <Badge
            variant={isTrusted ? "secondary" : "outline"}
            className={cn(
              "h-5 px-1.5 text-[10px] font-bold tracking-wider uppercase",
              isTrusted
                ? "border-primary/20 bg-primary/10 text-primary"
                : "border-destructive/20 bg-destructive/5 text-destructive",
            )}>
            {selectedUser.isExpired
              ? "Expired"
              : Boolean(selectedUser.trusted)
                ? "Trusted"
                : "Unverified"}
          </Badge>
        </div>

        {/* Fingerprint */}
        <div className="space-y-1">
          <Label className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
            Public Key Fingerprint
          </Label>

          <div className="rounded-lg border border-border/50 bg-muted/50 p-2 font-mono text-[10px] break-all text-muted-foreground select-all">
            {selectedUser.publicKey.substring(0, 32)}...
          </div>
        </div>
      </div>
    </div>
  );
};
