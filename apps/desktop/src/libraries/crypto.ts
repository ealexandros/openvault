import { createHash } from "node:crypto";

export const crypto = {
  sha256: (data: string) => {
    return createHash("sha256").update(data).digest("hex");
  },
  sha512: (data: string) => {
    return createHash("sha512").update(data).digest("hex");
  },
};
