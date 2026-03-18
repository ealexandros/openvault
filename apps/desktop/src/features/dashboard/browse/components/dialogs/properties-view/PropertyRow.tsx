type PropertyRowProps = {
  label: string;
  value: string;
};

export const PropertyRow = ({ label, value }: PropertyRowProps) => (
  <div className="flex items-center justify-between gap-4 rounded-md border border-border/60 bg-muted/20 px-3 py-2 text-sm">
    <span className="font-medium text-muted-foreground">{label}</span>
    <span className="truncate text-right font-mono text-foreground/90">{value}</span>
  </div>
);
