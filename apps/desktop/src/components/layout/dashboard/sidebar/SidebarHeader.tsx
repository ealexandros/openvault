import { Input } from "@/components/ui/shadcn/input";
import { SearchIcon, ShieldIcon } from "lucide-react";

export const SidebarHeader = () => (
  <header className="flex flex-col gap-8 py-6">
    <h1 className="flex cursor-default items-center gap-2 text-2xl font-semibold">
      <ShieldIcon className="size-7.5 text-primary" strokeWidth={2.5} />
      <span className="mt-1">OpenVault</span>
    </h1>

    <div className="relative">
      <SearchIcon
        className="absolute top-1/2 left-3 size-4 -translate-y-1/2"
        strokeWidth={1.5}
      />
      <Input
        placeholder="Search vault..."
        className="h-11 rounded-lg bg-white pl-9 text-base text-muted-foreground"
      />
    </div>
  </header>
);
