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
  encryption: string;
  compression: string;
  setFieldValue: (field: string, value: string) => void;
};

export const AdvancedSettings = ({
  encryption,
  compression,
  setFieldValue,
}: AdvancedSettingsProps) => (
  <Drawer direction="right">
    <DrawerTrigger asChild>
      <Button
        type="button"
        variant="ghost"
        size="sm"
        className="h-auto gap-2 px-1 py-2 text-xs font-bold tracking-widest text-muted-foreground uppercase transition-colors hover:bg-transparent hover:text-foreground">
        <Settings2Icon className="size-3.5" />
        <span>Advanced Settings</span>
      </Button>
    </DrawerTrigger>
    <DrawerContent className="max-w-md! p-0 before:inset-0 before:rounded-none">
      <div className="mx-auto flex h-full w-full flex-col gap-8 px-6 py-8">
        <DrawerHeader className="p-0">
          <DrawerTitle className="text-3xl font-bold tracking-tight">
            Advanced Settings
          </DrawerTitle>
          <DrawerDescription className="text-base text-muted-foreground">
            Configure technical encryption parameters for your vault
          </DrawerDescription>
        </DrawerHeader>

        <div className="flex-1 space-y-8 pt-4">
          <div className="space-y-4">
            <Label className="text-sm font-bold tracking-wider text-foreground uppercase">
              Encryption Algorithm
            </Label>
            <Select value={encryption} onValueChange={val => setFieldValue("encryption", val)}>
              <SelectTrigger className="-ml-1 h-16 w-full border-border/50 bg-muted/30 px-4 text-sm font-medium focus:ring-primary/20">
                <SelectValue placeholder="Select algorithm" />
              </SelectTrigger>
              <SelectContent className="border-border bg-card">
                <SelectGroup>
                  <SelectLabel className="text-xs font-bold tracking-widest text-muted-foreground uppercase">
                    Recommended
                  </SelectLabel>
                  <SelectItem value="xchacha">
                    <div className="flex flex-col items-start gap-1 py-1">
                      <span className="font-semibold text-foreground">XChaCha20-Poly1305</span>
                      <span className="text-xs text-muted-foreground">
                        Extended nonce for extra safety
                      </span>
                    </div>
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <p className="text-sm leading-relaxed text-muted-foreground/80">
              XChaCha20-Poly1305 is set as the default, offering an extended nonce and robust
              security across all devices.
            </p>
          </div>

          <div className="space-y-4">
            <Label className="text-sm font-bold tracking-wider text-foreground uppercase">
              Compression Algorithm
            </Label>
            <Select
              value={compression}
              onValueChange={val => setFieldValue("compression", val)}>
              <SelectTrigger className="-ml-1 h-16 w-full border-border/50 bg-muted/30 px-4 text-sm font-medium focus:ring-primary/20">
                <SelectValue placeholder="Select algorithm" />
              </SelectTrigger>
              <SelectContent className="border-border bg-card">
                <SelectGroup>
                  <SelectLabel className="text-xs font-bold tracking-widest text-muted-foreground uppercase">
                    Recommended
                  </SelectLabel>
                  <SelectItem value="zstd">
                    <div className="flex flex-col items-start gap-1 py-1">
                      <span className="font-semibold text-foreground">Zstd</span>
                      <span className="text-xs text-muted-foreground">
                        Fast compression with good ratios
                      </span>
                    </div>
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <p className="text-sm leading-relaxed text-muted-foreground/80">
              Zstd is set as the default, offering fast compression with good ratios.
            </p>
          </div>
        </div>

        <DrawerClose asChild>
          <Button variant="outline" className="h-12 w-full text-sm font-semibold">
            Close Settings
          </Button>
        </DrawerClose>
      </div>
    </DrawerContent>
  </Drawer>
);
