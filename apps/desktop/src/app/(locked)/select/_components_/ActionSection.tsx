import { cn } from "@/utils/cn";
import { EmptyFileAction } from "./EmptyFileAction";
import { IdentifiedFileAction } from "./IdentifiedFileAction";

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
  <div
    className={cn(
      "group relative overflow-hidden rounded-xl border border-border bg-card/40 p-12 backdrop-blur-2xl transition-all",
    )}>
    {selectedPath == null ? (
      <EmptyFileAction onSelect={onSelectFolder} />
    ) : (
      <IdentifiedFileAction
        path={selectedPath}
        onChange={onSelectFolder}
        onConnect={() => onConnect(selectedPath)}
      />
    )}
  </div>
);
