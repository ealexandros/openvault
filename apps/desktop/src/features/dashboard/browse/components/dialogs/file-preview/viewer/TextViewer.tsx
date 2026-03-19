import { ScrollArea, ScrollBar } from "@/components/ui/shadcn/scroll-area";

type TextViewerProps = {
  codeRef: React.RefObject<HTMLElement | null>;
};

export const TextViewer = ({ codeRef }: TextViewerProps) => (
  <ScrollArea className="h-full w-full bg-zinc-950">
    <div className="min-w-max p-6">
      <pre className="font-mono text-sm leading-relaxed text-zinc-300">
        <code ref={codeRef} />
      </pre>
    </div>
    <ScrollBar orientation="horizontal" />
    <ScrollBar orientation="vertical" />
  </ScrollArea>
);
