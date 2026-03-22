export const MessageModes = {
  Encrypt: "encrypt",
  Decrypt: "decrypt",
} as const;

export type MessageMode = (typeof MessageModes)[keyof typeof MessageModes];

export const WorkModes = {
  Text: "text",
  File: "file",
} as const;

export type WorkMode = (typeof WorkModes)[keyof typeof WorkModes];

export const FileSources = {
  Computer: "computer",
  Vault: "vault",
} as const;

export type FileSource = (typeof FileSources)[keyof typeof FileSources];

export const UserPresences = {
  Online: "online",
  Offline: "offline",
} as const;

export type UserPresence = (typeof UserPresences)[keyof typeof UserPresences];

export type FileInfo = {
  name: string;
  size: number;
  source: FileSource;
  id?: string;
  path?: string;
};

export type MessageUserProfile = {
  id: string;
  displayName: string;
  email: string;
  publicKey: string;
  status: UserPresence;
  trusted: boolean;
  importedAt: string;
  expiresAt: string;
  isExpired: boolean;
};

export type MessageUserProfileImport = {
  name?: string;
  signingPubKey?: number[];
  ephemeralPubKey?: number[];
  expiresAt?: string | null;
};
