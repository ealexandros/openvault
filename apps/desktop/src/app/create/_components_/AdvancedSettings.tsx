"use client";

import { Button } from "@/components/ui/shadcn/button";
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@/components/ui/shadcn/drawer";
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
import { Settings2Icon } from "lucide-react";

type AdvancedSettingsProps = {
  algorithm: string;
  setFieldValue: (field: string, value: string) => void;
};

export const AdvancedSettings = ({ algorithm, setFieldValue }: AdvancedSettingsProps) => (
  <Drawer direction="right">
    <DrawerTrigger asChild>
      <Button
        type="button"
        variant="ghost"
        size="sm"
        className="h-auto gap-2 px-1 py-2 text-xs font-bold tracking-widest text-muted-foreground uppercase transition-colors hover:bg-transparent hover:text-foreground">
        <Settings2Icon className="size-3.5" />
        Advanced Settings
      </Button>
    </DrawerTrigger>
    <DrawerContent className="max-w-md! p-0 before:inset-0 before:rounded-none">
      <div className="mx-auto flex h-full w-full flex-col gap-8 px-6 py-8">
        <DrawerHeader className="p-0 px-1">
          <DrawerTitle className="text-3xl font-bold tracking-tight">
            Advanced Settings
          </DrawerTitle>
          <DrawerDescription className="text-base text-muted-foreground">
            Configure technical encryption parameters for your vault
          </DrawerDescription>
        </DrawerHeader>

        <div className="flex-1 space-y-8 pt-4">
          <div className="space-y-4">
            <Label className="ml-1 text-sm font-bold tracking-wider text-foreground uppercase">
              Encryption Algorithm
            </Label>
            <Select value={algorithm} onValueChange={val => setFieldValue("algorithm", val)}>
              <SelectTrigger className="h-16 w-full border-border/50 bg-muted/30 px-4 text-sm font-medium focus:ring-primary/20">
                <SelectValue placeholder="Select algorithm" />
              </SelectTrigger>
              <SelectContent className="border-border bg-card">
                <SelectGroup>
                  <SelectLabel className="text-xs font-bold tracking-widest text-muted-foreground uppercase">
                    Recommended
                  </SelectLabel>
                  <SelectItem value="xchacha" className="rounded-xl">
                    <div className="flex flex-col items-start gap-1 py-1">
                      <span className="font-semibold text-foreground">XChaCha20-Poly1305</span>
                      <span className="text-xs text-muted-foreground">
                        Extended nonce for extra safety
                      </span>
                    </div>
                  </SelectItem>
                  <SelectItem value="aes-256-gcm" className="rounded-xl" disabled>
                    <div className="flex flex-col items-start gap-1 py-1">
                      <span className="font-semibold text-foreground">AES-256-GCM</span>
                      <span className="text-xs text-muted-foreground">
                        Standard, hardware-accelerated
                      </span>
                    </div>
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <p className="ml-1 text-sm leading-relaxed text-muted-foreground/80">
              XChaCha20-Poly1305 is set as the default, offering an extended nonce and robust
              security across all devices.
            </p>
          </div>
        </div>

        <div className="pt-6">
          <DrawerClose asChild>
            <Button variant="outline" className="h-12 w-full rounded-xl text-sm font-semibold">
              Close Settings
            </Button>
          </DrawerClose>
        </div>
      </div>
    </DrawerContent>
  </Drawer>
);
