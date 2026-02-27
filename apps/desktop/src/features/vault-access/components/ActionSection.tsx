import { cn } from "@/utils/cn";

type FileActionProps = {
  onBrowse: () => void;
};

export const ActionSection = ({ onBrowse }: FileActionProps) => (
  <div
    onClick={onBrowse}
    className={cn(
      "group relative cursor-pointer overflow-hidden rounded-2xl border-2 border-dashed border-slate-200 bg-white p-14 transition-all duration-500",
      "hover:border-primary/40 hover:bg-primary/2 hover:shadow-[0_20px_50px_rgba(0,0,0,0.02)] active:scale-[0.99]",
    )}>
    <div className="relative flex flex-col items-center justify-center space-y-6 text-center">
      <div className="space-y-2">
        <h2 className="text-xl font-semibold tracking-tight text-slate-900 transition-colors duration-500 group-hover:text-primary">
          Choose a Vault
        </h2>
        <p className="mx-auto max-w-72 text-sm leading-relaxed font-medium text-slate-500/80 transition-colors duration-500 group-hover:text-slate-600">
          Select an encrypted vault file from your device to unlock your data.
        </p>
      </div>
      <div className="flex items-center gap-2 rounded-xl border border-slate-100 bg-slate-50 px-8 py-3 text-[11px] font-bold tracking-[0.15em] text-slate-400 uppercase transition-all duration-300 group-hover:border-transparent group-hover:bg-primary group-hover:text-white">
        Browse Files
      </div>
    </div>
  </div>
);
