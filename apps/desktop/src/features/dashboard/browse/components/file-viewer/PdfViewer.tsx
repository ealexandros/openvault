import { ViewerLoading } from "./ViewerLoading";

type PdfViewerProps = {
  url: string | null;
};

export const PdfViewer = ({ url }: PdfViewerProps) => {
  if (url == null) {
    return <ViewerLoading message="Processing PDF..." />;
  }

  return (
    <div className="h-full w-full p-2">
      <object
        data={url}
        type="application/pdf"
        className="h-full w-full overflow-hidden rounded-md bg-white">
        <div className="flex flex-col items-center justify-center gap-2 p-8 text-center text-muted-foreground">
          <p>Your browser doesn&apos;t support PDF viewing.</p>
        </div>
      </object>
    </div>
  );
};
