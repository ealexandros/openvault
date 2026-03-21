import { ViewerLoading } from "./ViewerLoading";

type AudioViewerProps = {
  url: string | null;
  fileName: string;
};

export const AudioViewer = ({ url, fileName }: AudioViewerProps) => {
  if (url == null) {
    return <ViewerLoading message="Processing audio..." />;
  }

  return (
    <div className="flex h-full w-full flex-col items-center justify-center gap-6 p-8">
      <div className="text-center">
        <h3 className="text-lg font-medium text-foreground">{fileName}</h3>
        <p className="text-sm text-muted-foreground">Audio Preview</p>
      </div>
      <audio
        src={url}
        controls
        className="w-full max-w-md rounded-md bg-muted/50 outline-hidden">
        Your browser doesn&apos;t support the audio element.
      </audio>
    </div>
  );
};
