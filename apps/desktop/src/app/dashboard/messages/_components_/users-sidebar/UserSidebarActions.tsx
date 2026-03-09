import { Button } from "@/components/ui/shadcn/button";
import { motion } from "framer-motion";
import { Download, UserPlus } from "lucide-react";
import { MessageUserProfile } from "../../useMessagesPage";

type UserSidebarActionsProps = {
  openImportPicker: () => void;
  exportSelectedUserProfile: () => void;
  exportCurrentUserProfile: () => void;
  selectedUser: MessageUserProfile | null;
  importError: string | null;
};

export const UserSidebarActions = ({
  openImportPicker,
  exportSelectedUserProfile,
  exportCurrentUserProfile,
  selectedUser,
  importError,
}: UserSidebarActionsProps) => (
  <div className="grid grid-cols-1 gap-2 px-6 pb-4">
    <Button
      variant="outline"
      size="sm"
      className="h-9 justify-start gap-2 rounded-lg"
      onClick={openImportPicker}>
      <UserPlus className="h-4 w-4 text-primary" />
      Import Profile
    </Button>

    <div className="flex gap-2">
      <Button
        variant="secondary"
        size="sm"
        className="flex-1 gap-2"
        onClick={exportCurrentUserProfile}>
        <Download className="h-4 w-4" />
        Mine
      </Button>

      <Button
        variant="secondary"
        size="sm"
        className="flex-1 gap-2"
        disabled={!Boolean(selectedUser)}
        onClick={exportSelectedUserProfile}>
        <Download className="h-4 w-4" />
        Selected
      </Button>
    </div>

    {importError != null && (
      <motion.div
        initial={{ opacity: 0, y: -10 }}
        animate={{ opacity: 1, y: 0 }}
        className="rounded-lg border border-destructive/20 bg-destructive/10 p-2.5 text-[11px] text-destructive">
        {importError}
      </motion.div>
    )}
  </div>
);
