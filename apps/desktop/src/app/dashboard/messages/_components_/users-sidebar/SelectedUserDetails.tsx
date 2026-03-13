"use client";

import { Badge } from "@/components/ui/shadcn/badge";
import { Label } from "@/components/ui/shadcn/label";
import { type MessageContact } from "@/types/messages";
import { cn } from "@/utils/cn";
import { Clock, ShieldCheck } from "lucide-react";

type Props = {
  selectedUser: MessageContact | null;
};

export const SelectedUserDetails = ({ selectedUser }: Props) => {
  if (selectedUser == null) return null;

  const isExpired =
    selectedUser.expiresAt != null ? new Date(selectedUser.expiresAt) < new Date() : false;
  const isSecure = selectedUser.secure && !isExpired;

  const fingerprint = Array.from(selectedUser.signingPubKey)
    .map(b => b.toString(16).padStart(2, "0"))
    .join("")
    .substring(0, 32);

  return (
    <div className="border-t border-border bg-muted/10 p-6">
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-start justify-between">
          <div className="space-y-1">
            <h4 className="text-sm font-semibold">{selectedUser.name}</h4>

            <div className="flex flex-col gap-1">
              <div className="flex items-center gap-2 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
                <Clock className="size-2.5" />
                <span>Added: {new Date(selectedUser.createdAt).toLocaleDateString()}</span>
              </div>

              {selectedUser.expiresAt != null && (
                <div className="flex items-center gap-2 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
                  <ShieldCheck className="h-2.5 w-2.5" />
                  <span>Expires: {new Date(selectedUser.expiresAt).toLocaleDateString()}</span>
                </div>
              )}
            </div>
          </div>

          {/* Trust Badge */}
          <Badge
            variant={isSecure ? "secondary" : "outline"}
            className={cn(
              "h-5 px-1.5 text-[10px] font-bold tracking-wider uppercase",
              isSecure
                ? "border-primary/20 bg-primary/10 text-primary"
                : "border-destructive/20 bg-destructive/5 text-destructive",
            )}>
            {isExpired ? "Expired" : selectedUser.secure ? "Secure" : "Unverified"}
          </Badge>
        </div>

        {/* Fingerprint */}
        <div className="space-y-1">
          <Label className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
            Public Key Fingerprint
          </Label>

          <div className="rounded-lg border border-border/50 bg-muted/50 p-2 font-mono text-[10px] break-all text-muted-foreground select-all">
            {fingerprint}...
          </div>
        </div>
      </div>
    </div>
  );
};
