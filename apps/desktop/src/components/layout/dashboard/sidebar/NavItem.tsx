import { cn } from "@/utils/cn";
import type { LucideIcon } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

type NavItemProps = {
  href: string;
  label: string;
  icon: LucideIcon;
  onClick?: () => void;
  className?: string;
  iconClassName?: string;
};

export function NavItem({
  href,
  label,
  icon: Icon,
  onClick,
  className,
  iconClassName,
}: NavItemProps) {
  const pathname = usePathname();
  const isActive = pathname === href;

  return (
    <div className="group relative w-full">
      <Link
        href={href}
        data-active={isActive}
        onClick={onClick}
        className={cn(
          "group flex w-full items-center gap-3 p-3 transition-all duration-300",
          "text-muted-foreground/80 hover:text-foreground/80",
          "data-[active=true]:text-primary",
          "data-[active=true]:backdrop-blur-sm",
          className,
        )}>
        <Icon
          data-active={isActive}
          className={cn(
            "size-5 transition-colors",
            "text-muted-foreground/60 group-hover:text-foreground/80",
            "data-[active=true]:text-primary",
            iconClassName,
          )}
        />
        <span className="text-base font-medium tracking-tight">{label}</span>
      </Link>
      <div
        data-active={isActive}
        className={cn(
          "absolute top-1/2 -left-3 h-6 w-1 -translate-y-1/2 rounded-full",
          "data-[active=true]:bg-primary",
        )}
      />
    </div>
  );
}
