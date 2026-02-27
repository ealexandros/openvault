import { FileIcon } from "lucide-react";

type EmptyFileActionProps = {
  onSelect: () => void;
};

export const EmptyFileAction = ({ onSelect }: EmptyFileActionProps) => (
  <div
    onClick={onSelect}
    className="relative flex cursor-pointer flex-col items-center justify-center space-y-6 text-center">
    <div className="relative">
      <div className="relative flex h-20 w-20 items-center justify-center rounded-2xl border border-primary/20 bg-primary/5 transition-all duration-500 group-hover:bg-primary/10">
        <FileIcon className="h-10 w-10 text-primary" />
      </div>
    </div>

    <div className="space-y-2">
      <h2 className="text-xl font-bold tracking-tight text-slate-900">Open Your File</h2>
      <p className="mx-auto max-w-[320px] text-sm leading-relaxed text-muted-foreground/80">
        Choose an encrypted file to unlock or a plain file to protect.
      </p>
    </div>

    <div className="pt-2">
      <p className="text-[10px] font-bold tracking-[0.2em] text-primary uppercase transition-all duration-300">
        Click to browse files
      </p>
    </div>
  </div>
);
