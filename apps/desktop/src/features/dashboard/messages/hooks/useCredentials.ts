"use client";

import { tauriApi } from "@/libraries/tauri-api";
import { useEffect, useEffectEvent, useState } from "react";
import { toast } from "sonner";

type Credentials = {
  name: string;
  signingPubKey: number[];
  ephemeralPubKey: number[];
  expiresAt: string | null;
};

export const useCredentials = () => {
  const [credentials, setCredentials] = useState<Credentials | null>(null);
  const [isSetup, setIsSetup] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  const checkSetup = async () => {
    setIsLoading(true);

    const result = await tauriApi.getMessageCredentials();

    if (result.success && result.data) {
      setCredentials(result.data);
      setIsSetup(true);
    } else {
      setCredentials(null);
      setIsSetup(false);
    }

    setIsLoading(false);
  };

  const completeOnboarding = async (name: string, rotationMonths: number) => {
    const expiresAt = new Date();
    expiresAt.setMonth(expiresAt.getMonth() + rotationMonths);

    const result = await tauriApi.createMessageCredentials({
      name,
      expiresAt: expiresAt.toISOString(),
    });

    if (!result.success) {
      toast.error("Failed to create profile");
      return false;
    }

    await checkSetup();
    toast.success("Profile setup complete");
    return true;
  };

  const renewCredentials = async () => {
    const result = await tauriApi.renewMessageCredentials();

    if (!result.success) {
      toast.error("Failed to renew credentials");
      return false;
    }

    await checkSetup();
    toast.success("Credentials renewed");
    return true;
  };

  const resetCredentials = async () => {
    const result = await tauriApi.resetMessageCredentials();

    if (!result.success) {
      toast.error("Failed to reset credentials");
      return false;
    }

    await checkSetup();
    toast.success("Credentials reset");
    return true;
  };

  const exportProfile = () => {
    if (!credentials) return;

    const filename = credentials.name.toLowerCase().replace(/\s+/g, "-") + "-profile.ovp";

    const blob = new Blob([JSON.stringify(credentials, null, 2)], {
      type: "application/json",
    });

    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");

    a.href = url;
    a.download = filename;
    a.click();

    URL.revokeObjectURL(url);

    toast.success("Your profile exported");
  };

  const checkSetupEvent = useEffectEvent(checkSetup);

  useEffect(() => {
    void checkSetupEvent();
  }, []);

  return {
    credentials,
    isSetup,
    isLoading,
    checkSetup,
    completeOnboarding,
    renewCredentials,
    resetCredentials,
    exportProfile,
  };
};
