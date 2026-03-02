import { Badge } from "@/components/ui/shadcn/badge";
import { type LucideIcon } from "lucide-react";
import { type ReactNode } from "react";

type BrowseSectionProps = {
  title: string;
  count: number;
  icon: LucideIcon;
  children: ReactNode;
};

export const BrowseSection = ({ title, count, icon: Icon, children }: BrowseSectionProps) => (
  <section className="space-y-4 animate-in fade-in slide-in-from-bottom-2 duration-300">
    <div className="flex items-center gap-3">
      <Icon className="size-4 text-muted-foreground" />
      <h2 className="text-base font-semibold">{title}</h2>
      <Badge variant="outline">{count}</Badge>
    </div>
    {children}
  </section>
);
