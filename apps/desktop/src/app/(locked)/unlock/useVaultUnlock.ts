import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { tauriApi } from "@/libraries/tauri-api";
import { useRouter } from "next/navigation";
import { useState } from "react";

export const useVaultUnlock = () => {
  const { selectedPath, setIsUnlocked } = useVault();
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  const handleBack = () => {
    router.push(hrefs.home.get());
  };

  const handleProcessVault = async () => {
    const result = await tauriApi.openVault({
      path: selectedPath ?? "",
      password,
    });

    if (!result.success) {
      const message = typeof result.error === "string" ? result.error : "Failed to open vault";
      setError(message);
      return;
    }

    setIsUnlocked(true);
    router.push(hrefs.dashboard.get());
    setPassword("");
  };

  const toggleShowPassword = () => {
    setShowPassword(prev => !prev);
  };

  return {
    selectedPath,
    password,
    setPassword,
    showPassword,
    toggleShowPassword,
    handleBack,
    handleProcessVault,
    router,
    error,
  };
};
