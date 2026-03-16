import { getPasswordStrength } from "@/components/ui/password-strength";
import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { tauriApi } from "@/libraries/tauri-api";
import { useForm } from "@tanstack/react-form";
import { open } from "@tauri-apps/plugin-dialog";
import { useRouter } from "next/navigation";
import { useRef, useState } from "react";
import z from "zod";

const setupVaultFormSchema = z.object({
  path: z.string().min(1, "Please select a location"),
  name: z.string().min(1, "Vault name is required"),
  encryption: z.enum(["xchacha20"]).default("xchacha20"),
  compression: z.enum(["zstd"]).default("zstd"),
});

export type SetupVaultFormValues = z.infer<typeof setupVaultFormSchema>;

const defaultFormValues: SetupVaultFormValues = {
  path: "",
  name: "",
  encryption: "xchacha20",
  compression: "zstd",
};

export const useSetupVault = () => {
  const { setIsUnlocked } = useVault();
  const router = useRouter();
  const [isEncrypting, setIsEncrypting] = useState(false);
  const [passwordError, setPasswordError] = useState<string | null>(null);
  const [passwordStrengthScore, setPasswordStrengthScore] = useState(0);
  const [showPassword, setShowPassword] = useState(false);

  const passwordRef = useRef<HTMLInputElement>(null);
  const verifyPasswordRef = useRef<HTMLInputElement>(null);

  const toggleShowPassword = () => setShowPassword(prev => !prev);

  const handlePasswordChange = () => {
    if (passwordRef.current) {
      setPasswordStrengthScore(getPasswordStrength(passwordRef.current.value));
    }
  };

  const form = useForm({
    defaultValues: defaultFormValues,
    validators: {
      onBlur: ({ value }) => {
        const result = setupVaultFormSchema.safeParse(value);
        return !result.success ? result.error.issues[0]?.message : undefined;
      },
    },
    onSubmit: async ({ value }) => {
      setPasswordError(null);
      const password = passwordRef.current?.value ?? "";
      const verifyPassword = verifyPasswordRef.current?.value ?? "";

      if (password.length < 8) {
        setPasswordError("Password must be at least 8 characters");
        return;
      }
      if (password !== verifyPassword) {
        setPasswordError("Passwords do not match");
        return;
      }

      setIsEncrypting(true);

      const encoder = new TextEncoder();
      const passwordBytes = encoder.encode(password);

      const result = await tauriApi.createVault({
        path: value.path,
        name: value.name,
        password: Array.from(passwordBytes),
        encryption: value.encryption,
        compression: value.compression,
      });

      passwordBytes.fill(0);
      if (passwordRef.current) passwordRef.current.value = "";
      if (verifyPasswordRef.current) verifyPasswordRef.current.value = "";
      setPasswordStrengthScore(0);

      if (result.success) {
        setIsUnlocked(true);
        router.push(hrefs.dashboard.get());
      }

      setIsEncrypting(false);
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    e.stopPropagation();
    void form.handleSubmit();
  };

  const chooseFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select folder to encrypt",
    });

    if (selected != null && typeof selected === "string") {
      form.setFieldValue("path", selected);
    }
  };

  return {
    form,
    isEncrypting,
    router,
    passwordRef,
    verifyPasswordRef,
    passwordError,
    showPassword,
    passwordStrengthScore,
    handleSubmit,
    toggleShowPassword,
    handlePasswordChange,
    setIsEncrypting,
    chooseFolder,
  };
};
