import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { tauriApi } from "@/libraries/tauri-api";
import { useFormik } from "formik";
import { useRouter } from "next/navigation";
import { useState } from "react";
import z from "zod";
import { toFormikValidationSchema } from "zod-formik-adapter";

// @todo-soon rethink about position..

const setupVaultSchema = z
  .object({
    path: z.string().min(1, "Please select a location"),
    name: z.string().min(1, "Vault name is required"),
    algorithm: z.string().default("xchacha"),
    password: z.string().min(8, "Password must be at least 8 characters"),
    verifyPassword: z.string().min(1, "Please verify your password"),
  })
  .refine(data => data.password === data.verifyPassword, {
    message: "Passwords do not match",
    path: ["verifyPassword"],
  });

export type SetupVaultType = z.infer<typeof setupVaultSchema>;

export const useSetupVault = () => {
  const { setIsUnlocked } = useVault();
  const router = useRouter();
  const [isEncrypting, setIsEncrypting] = useState(false);

  const formik = useFormik<SetupVaultType>({
    initialValues: {
      path: "",
      name: "",
      algorithm: "xchacha",
      password: "",
      verifyPassword: "",
    },
    validationSchema: toFormikValidationSchema(setupVaultSchema),
    onSubmit: async values => {
      setIsEncrypting(true);
      const result = await tauriApi.safeInvoke("create_vault", {
        params: {
          path: values.path,
          name: values.name,
          password: values.password,
        },
      });

      if (result.error == null) {
        setIsUnlocked(true);
        router.push(hrefs.dashboard.get());
      }

      setIsEncrypting(false);
    },
  });

  return {
    formik,
    isEncrypting,
    setIsEncrypting,
    router,
  };
};
