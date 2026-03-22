import { motion } from "framer-motion";
import { Search } from "lucide-react";

// @todo-now fix this..

export const UserListEmpty = () => (
  <motion.div
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    className="flex flex-col items-center justify-center rounded-xl border border-dashed py-12 text-center">
    <div className="rounded-full bg-muted p-3">
      <Search className="h-5 w-5 text-muted-foreground" />
    </div>

    <p className="mt-2 text-sm font-medium">No results found</p>
    <p className="text-xs text-muted-foreground">Try a different search term</p>
  </motion.div>
);
