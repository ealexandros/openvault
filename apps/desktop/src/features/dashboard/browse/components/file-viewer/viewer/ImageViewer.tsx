import { ViewerLoading } from "./ViewerLoading";

type ImageViewerProps = {
  url: string | null;
  alt: string;
};

export const ImageViewer = ({ url, alt }: ImageViewerProps) => {
  if (url == null) {
    return <ViewerLoading message="Processing image..." />;
  }

  return (
    <div className="flex h-full w-full items-center justify-center p-4">
      <img
        src={url}
        alt={alt}
        className="max-h-full max-w-full rounded-md object-contain shadow-sm"
      />
    </div>
  );
};
