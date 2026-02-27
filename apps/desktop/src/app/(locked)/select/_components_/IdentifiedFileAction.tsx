import { Button } from "@/components/ui/shadcn/button";
import { ArrowRightIcon, FileIcon } from "lucide-react";

type IdentifiedFileActionProps = {
  path: string;
  onChange: () => void;
  onConnect: () => void;
};

export const IdentifiedFileAction = ({
  path,
  onChange,
  onConnect,
}: IdentifiedFileActionProps) => (
  <div className="relative flex flex-col items-center justify-center space-y-6 text-center">
    <div className="relative">
      <div className="relative flex h-20 w-20 items-center justify-center rounded-2xl border border-primary/20 bg-primary/5">
        <FileIcon className="h-10 w-10 text-primary" />
      </div>
    </div>

    <div className="space-y-2">
      <h2 className="text-xl font-bold tracking-tight text-slate-900">File Identified</h2>
      <p className="mx-auto max-w-[320px] truncate text-sm leading-relaxed text-muted-foreground/80">
        {path}
      </p>
    </div>

    <div className="flex gap-3 pt-2">
      <Button
        variant="ghost"
        size="sm"
        onClick={onChange}
        className="h-10 rounded-xl px-6 text-sm font-semibold hover:bg-muted">
        Change file
      </Button>
      <Button
        onClick={onConnect}
        size="sm"
        className="h-10 rounded-xl bg-primary px-8 text-sm font-bold text-primary-foreground transition-all">
        Unlock
        <ArrowRightIcon className="ml-2 h-4 w-4" />
      </Button>
    </div>
  </div>
);
