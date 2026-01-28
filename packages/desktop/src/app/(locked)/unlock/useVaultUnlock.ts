import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { useRouter } from "next/navigation";
import { useState } from "react";

export const useVaultUnlock = () => {
  const { selectedPath, setIsUnlocked } = useVault();
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const router = useRouter();

  const handleBack = () => {
    router.push(hrefs.home.get());
  };

  const handleProcessVault = () => {
    setIsUnlocked(true);
    router.push(hrefs.dashboard.get());
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
  };
};
