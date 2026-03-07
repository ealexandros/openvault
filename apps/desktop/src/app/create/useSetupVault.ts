import { getPasswordStrength } from "@/components/ui/password-strength";
import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { tauriApi } from "@/libraries/tauri-api";
import { open } from "@tauri-apps/plugin-dialog";
import { useFormik } from "formik";
import { useRouter } from "next/navigation";
import { useRef, useState } from "react";
import z from "zod";
import { toFormikValidationSchema } from "zod-formik-adapter";

const setupVaultSchema = z.object({
  path: z.string().min(1, "Please select a location"),
  name: z.string().min(1, "Vault name is required"),
  encryption: z.enum(["xchacha20"]).default("xchacha20"),
  compression: z.enum(["zstd"]).default("zstd"),
});
export type SetupVaultType = z.infer<typeof setupVaultSchema>;

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

  const formik = useFormik<SetupVaultType>({
    initialValues: {
      path: "",
      name: "",
      encryption: "xchacha20",
      compression: "zstd",
    },
    validationSchema: toFormikValidationSchema(setupVaultSchema),
    onSubmit: async values => {
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
        path: values.path,
        name: values.name,
        password: Array.from(passwordBytes),
        encryption: values.encryption,
        compression: values.compression,
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

  const chooseFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select folder to encrypt",
    });

    if (selected != null && typeof selected === "string") {
      await formik.setFieldValue("path", selected);
    }
  };

  return {
    formik,
    isEncrypting,
    router,
    passwordRef,
    verifyPasswordRef,
    passwordError,
    showPassword,
    passwordStrengthScore,
    toggleShowPassword,
    handlePasswordChange,
    setIsEncrypting,
    chooseFolder,
  };
};
