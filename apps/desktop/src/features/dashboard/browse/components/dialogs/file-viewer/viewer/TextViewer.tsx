import { ScrollArea, ScrollBar } from "@/components/ui/shadcn/scroll-area";
import { ViewerLoading } from "./ViewerLoading";

type TextViewerProps = {
  text: string | null;
};

export const TextViewer = ({ text }: TextViewerProps) => {
  if (text == null) {
    return <ViewerLoading />;
  }

  return (
    <ScrollArea className="h-full w-full bg-zinc-950">
      <div className="min-w-max p-6">
        <pre className="font-mono text-sm leading-relaxed text-zinc-300">
          <code>{text}</code>
        </pre>
      </div>
      <ScrollBar orientation="horizontal" />
      <ScrollBar orientation="vertical" />
    </ScrollArea>
  );
};
