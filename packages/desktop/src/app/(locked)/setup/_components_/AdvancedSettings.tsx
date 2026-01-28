"use client";

import { Label } from "@/components/ui/shadcn/label";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/shadcn/select";
import { ChevronDownIcon, ChevronUpIcon } from "lucide-react";
import { useState } from "react";

type AdvancedSettingsProps = {
  algorithm: string;
  setFieldValue: (field: string, value: any) => void;
};

export const AdvancedSettings = ({ algorithm, setFieldValue }: AdvancedSettingsProps) => {
  const [showAdvanced, setShowAdvanced] = useState(false);

  return (
    <div className="pt-2">
      <button
        type="button"
        onClick={() => setShowAdvanced(!showAdvanced)}
        className="flex items-center gap-2 text-[11px] font-bold tracking-widest text-muted-foreground uppercase transition-colors hover:text-foreground">
        Advanced Settings
        {showAdvanced ? (
          <ChevronUpIcon className="h-3 w-3" />
        ) : (
          <ChevronDownIcon className="h-3 w-3" />
        )}
      </button>

      {showAdvanced && (
        <div className="mt-4 animate-in space-y-4 duration-300 fade-in slide-in-from-top-2">
          <div className="space-y-2">
            <Label className="ml-1 text-[11px] font-bold tracking-widest text-muted-foreground uppercase">
              Encryption Algorithm
            </Label>
            <Select value={algorithm} onValueChange={val => setFieldValue("algorithm", val)}>
              <SelectTrigger className="h-16! w-full rounded-2xl border-border bg-muted/30 px-4 text-sm focus:ring-primary/20">
                <SelectValue placeholder="Select algorithm" />
              </SelectTrigger>
              <SelectContent className="rounded-xl border-border bg-card">
                <SelectGroup>
                  <SelectLabel className="text-[10px] tracking-widest text-muted-foreground/60 uppercase">
                    Recommended
                  </SelectLabel>
                  <SelectItem value="aes-256-gcm" className="rounded-lg">
                    <div className="flex flex-col items-start gap-0.5">
                      <span className="font-medium">AES-256-GCM</span>
                      <span className="text-[10px] text-muted-foreground">
                        Standard, hardware-accelerated
                      </span>
                    </div>
                  </SelectItem>
                  <SelectItem value="chacha20-poly1305" className="rounded-lg" disabled>
                    <div className="flex flex-col items-start gap-0.5">
                      <span className="font-medium">ChaCha20-Poly1305</span>
                      <span className="text-[10px] text-muted-foreground">
                        Fast on devices without AES-NI
                      </span>
                    </div>
                  </SelectItem>
                  <SelectItem value="xchacha20-poly1305" className="rounded-lg" disabled>
                    <div className="flex flex-col items-start gap-0.5">
                      <span className="font-medium">XChaCha20-Poly1305</span>
                      <span className="text-[10px] text-muted-foreground">
                        Extended nonce for extra safety
                      </span>
                    </div>
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </div>
      )}
    </div>
  );
};
