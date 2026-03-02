import { ShieldIcon } from "lucide-react";

export const SidebarHeader = () => (
  <header className="flex flex-col gap-8 py-6">
    <h1 className="flex cursor-default items-center gap-2 text-2xl font-semibold">
      <ShieldIcon className="size-7.5 text-primary" strokeWidth={2.5} />
      <span className="mt-1">OpenVault</span>
    </h1>
  </header>
);
