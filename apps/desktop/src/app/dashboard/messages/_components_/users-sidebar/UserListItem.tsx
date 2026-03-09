import { cn } from "@/utils/cn";
import { motion } from "framer-motion";
import { ShieldCheck } from "lucide-react";
import { MessageUserProfile } from "../../useMessagesPage";

type UserListItemProps = {
  user: MessageUserProfile;
  selected: boolean;
  onClick: () => void;
};

export const UserListItem = ({ user, selected, onClick }: UserListItemProps) => (
  <motion.button
    layout
    onClick={onClick}
    className={cn(
      "group w-full rounded-xl border p-3 text-left transition-all",
      selected
        ? "border-primary/50 bg-primary/5"
        : "border-transparent hover:border-border hover:bg-muted/50",
    )}>
    <div className="flex items-start gap-3">
      <div
        className={cn(
          "flex h-8 w-8 items-center justify-center rounded-lg border text-xs font-semibold",
          selected
            ? "border-primary/20 bg-primary/10 text-primary"
            : "border-border bg-muted/30 text-muted-foreground",
        )}>
        {user.displayName.charAt(0).toUpperCase()}
      </div>

      <div className="flex-1 overflow-hidden">
        <div className="flex justify-between">
          <p className="truncate text-sm font-medium">{user.displayName}</p>

          {user.trusted && !user.isExpired && <ShieldCheck className="h-3 w-3 text-primary" />}
        </div>

        <p className="truncate text-xs text-muted-foreground">{user.email}</p>
      </div>
    </div>
  </motion.button>
);
