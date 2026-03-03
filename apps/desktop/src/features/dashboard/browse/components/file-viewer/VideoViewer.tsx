import { ViewerLoading } from "./ViewerLoading";

type VideoViewerProps = {
  url: string | null;
  fileName: string;
};

export const VideoViewer = ({ url, fileName }: VideoViewerProps) => {
  if (url == null) {
    return <ViewerLoading message="Processing video..." />;
  }

  return (
    <div className="flex h-full w-full flex-col items-center justify-center gap-4 p-4">
      <div className="absolute top-4 left-0 z-10 w-full p-2 text-center opacity-50 transition-opacity hover:opacity-100">
        <h3 className="text-lg font-medium text-foreground drop-shadow-md">{fileName}</h3>
      </div>
      <video
        controls
        src={url}
        className="max-h-full max-w-full rounded-md bg-black shadow-sm outline-hidden">
        Your browser doesn&apos;t support the video element.
      </video>
    </div>
  );
};
