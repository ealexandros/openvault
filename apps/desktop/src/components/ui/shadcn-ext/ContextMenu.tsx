import {
  ContextMenu as BaseContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/shadcn/context-menu";
import { type ReactNode } from "react";

export type ContextMenuItem = {
  label: string;
  icon?: React.ElementType;
  onClick: () => void;
  variant?: "destructive" | "default";
  disabled?: boolean;
};

export type ContextMenuProps = {
  children: ReactNode;
  items: ContextMenuItem[];
};

export const ContextMenu = ({ children, items }: ContextMenuProps) => (
  <BaseContextMenu>
    <ContextMenuTrigger asChild>{children}</ContextMenuTrigger>
    <ContextMenuContent className="w-48 overflow-hidden rounded-lg border-border/50 bg-background/95 backdrop-blur-xl">
      {items.map(({ label, icon: Icon, onClick, variant, disabled }, idx) => (
        <ContextMenuItem
          key={idx}
          variant={variant}
          disabled={disabled}
          onClick={e => {
            e.stopPropagation();
            onClick();
          }}
          className="gap-2.5 px-3 py-2">
          {Icon != null && <Icon className="size-3.5 text-muted-foreground" />}
          <span className="font-medium">{label}</span>
        </ContextMenuItem>
      ))}
    </ContextMenuContent>
  </BaseContextMenu>
);
