export type MessageCredentials = {
  name: string;
  signingPubKey: number[];
  ephemeralPubKey: number[];
  secure: boolean;
  expiresAt: string | null;
};

export type MessageContact = {
  id: string;
  name: string;
  signingPubKey: number[];
  ephemeralPubKey: number[];
  secure: boolean;
  expiresAt: string | null;
  createdAt: string;
};
