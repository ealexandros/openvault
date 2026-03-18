import { Spinner } from "@/components/ui/shadcn/spinner";

type ViewerLoadingProps = {
  message?: string;
};

export const ViewerLoading = ({ message = "Loading file content..." }: ViewerLoadingProps) => {
  return (
    <div className="flex flex-col items-center justify-center gap-4 py-20 text-muted-foreground">
      <Spinner className="size-8" />
      <p className="text-sm">{message}</p>
    </div>
  );
};
