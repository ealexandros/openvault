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
      <iframe
        src={url}
        title="PDF Document"
        className="h-full w-full rounded-md border-none bg-white shadow-sm"
        allow="fullscreen"
      />
    </div>
  );
};
