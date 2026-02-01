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
    algorithm: z.string().default("aes-256-gcm"),
    password: z.string().min(8, "Password must be at least 8 characters"),
    verifyPassword: z.string().min(1, "Please verify your password"),
  })
  .refine(data => data.password === data.verifyPassword, {
    message: "Passwords do not match",
    path: ["verifyPassword"],
  });

export type SetupVaultType = z.infer<typeof setupVaultSchema>;

export const useSetupVault = () => {
  const router = useRouter();
  const [isEncrypting, setIsEncrypting] = useState(false);

  const formik = useFormik<SetupVaultType>({
    initialValues: {
      path: "",
      name: "",
      algorithm: "aes-256-gcm",
      password: "",
      verifyPassword: "",
    },
    validationSchema: toFormikValidationSchema(setupVaultSchema),
    onSubmit: () => {
      setIsEncrypting(true);
    },
  });

  return {
    formik,
    isEncrypting,
    setIsEncrypting,
    router,
  };
};
