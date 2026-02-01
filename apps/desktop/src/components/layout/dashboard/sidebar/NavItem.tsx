import { cn } from "@/utils/cn";
import { ChevronRightIcon, type LucideIcon } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

type NavItemProps = {
  href: string;
  label: string;
  icon: LucideIcon;
};

export const NavItem = ({ href, label, icon: Icon }: NavItemProps) => {
  const pathname = usePathname();
  const isActive = pathname === href;

  return (
    <Link
      href={href}
      className={cn(
        "flex w-full items-center justify-between rounded-xl p-3 py-2.5 transition-all duration-200",
        isActive
          ? "border border-primary/20 bg-primary/10 text-primary"
          : "border border-transparent text-muted-foreground hover:bg-muted/80 hover:text-foreground/80",
      )}>
      <div className="flex items-center gap-3">
        <Icon className="size-4 transition-transform group-hover:scale-110" />
        <span className="text-sm font-medium">{label}</span>
      </div>
      {isActive && <ChevronRightIcon className="size-4" />}
    </Link>
  );
};
