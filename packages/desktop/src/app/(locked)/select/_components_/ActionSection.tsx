import { Button } from "@/components/ui/shadcn/button";
import { cn } from "@/utils/cn";
import { FolderOpenIcon } from "lucide-react";

type ActionSectionProps = {
  selectedPath: string | null;
  onSelectFolder: () => void;
  onConnect: (path: string) => void;
};

export const ActionSection = ({
  selectedPath,
  onSelectFolder,
  onConnect,
}: ActionSectionProps) => (
  <div className="group relative">
    <div className="absolute -inset-0.5 rounded-2xl bg-linear-to-r from-primary/10 to-primary/5 opacity-50 blur-sm transition duration-500 group-hover:opacity-100" />

    <div
      onClick={!selectedPath ? onSelectFolder : undefined}
      className={cn(
        "relative flex flex-col items-center justify-center space-y-4 rounded-2xl border border-border bg-card p-8 text-center backdrop-blur-xl transition-all",
        !selectedPath && "cursor-pointer hover:bg-accent/30",
      )}>
      <div className="flex h-12 w-12 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 transition-transform duration-300 group-hover:scale-105">
        <FolderOpenIcon className="h-6 w-6 text-primary" />
      </div>
      <div className="space-y-1">
        <h2 className="text-base font-medium">
          {selectedPath ? "Folder selected" : "Open a folder"}
        </h2>
        <p className="mx-auto max-w-[280px] truncate text-xs text-muted-foreground">
          {selectedPath ? selectedPath : "Select a directory to encrypt or decrypt"}
        </p>
      </div>

      {selectedPath ? (
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={onSelectFolder}
            className="h-8 rounded-lg border-border px-4 text-xs font-medium hover:bg-muted">
            Change
          </Button>
          <Button
            onClick={() => onConnect(selectedPath)}
            size="sm"
            className="h-8 rounded-lg bg-white px-6 text-xs font-medium text-black hover:bg-zinc-200">
            Connect
          </Button>
        </div>
      ) : null}
    </div>
  </div>
);
