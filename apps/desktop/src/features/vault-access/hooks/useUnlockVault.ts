import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { tauriApi } from "@/libraries/tauri-api";
import { useRouter } from "next/navigation";
import { useRef, useState } from "react";
import { toast } from "sonner";
import { useRecentVault } from "./useRecentVault";

export const getVaultName = (path: string) =>
  path
    .replace(/\\/g, "/")
    .split("/")
    .pop()
    ?.replace(/\.[^/.]+$/, "") ?? "";

export const useUnlockVault = (selectedVaultPath: string | null) => {
  const [hasPassword, setHasPassword] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [isUnlocking, setIsUnlocking] = useState(false);
  const [rememberVault, setRememberVault] = useState(true);

  const passwordRef = useRef<HTMLInputElement>(null);
  const router = useRouter();
  const { setSelectedPath, setIsUnlocked } = useVault();

  const { addVaultToRecents } = useRecentVault();

  const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setHasPassword(e.target.value.length > 0);
  };

  const handleUnlock = async () => {
    if (!passwordRef.current) return;
    if (selectedVaultPath == null || isUnlocking) return;

    setIsUnlocking(true);

    const encoder = new TextEncoder();
    const passwordBytes = encoder.encode(passwordRef.current.value);

    const result = await tauriApi.openVault({
      path: selectedVaultPath,
      password: Array.from(passwordBytes),
    });

    passwordBytes.fill(0);
    passwordRef.current.value = "";
    setHasPassword(false);

    if (!result.success) {
      toast.error("Incorrect password", {
        description: "The password was incorrect. Please try again.",
      });
      setIsUnlocking(false);
      return;
    }

    if (rememberVault) {
      addVaultToRecents(selectedVaultPath);
    }

    setIsUnlocked(true);
    setSelectedPath(selectedVaultPath);
    router.push(hrefs.dashboard.get());
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    void handleUnlock();
  };

  const toggleShowPassword = () => setShowPassword(prev => !prev);

  const isButtonDisabled = !hasPassword || isUnlocking;

  return {
    isUnlocking,
    hasPassword,
    showPassword,
    rememberVault,
    passwordRef,
    isButtonDisabled,
    handlePasswordChange,
    toggleShowPassword,
    handleSubmit,
    setRememberVault,
  };
};
