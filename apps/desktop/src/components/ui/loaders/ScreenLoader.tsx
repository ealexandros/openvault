import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";
import { Spinner } from "@/components/ui/shadcn/spinner";
import { PropsWithClassName } from "@/types/react";
import { cn } from "@/utils/cn";

type ScreenLoaderProps = PropsWithClassName & {
  message: string;
  description?: string;
};

export const ScreenLoader = ({ message, description, className }: ScreenLoaderProps) => (
  <Empty className={cn("h-full w-full", className)}>
    <EmptyHeader>
      <EmptyMedia variant="icon">
        <Spinner />
      </EmptyMedia>
      <EmptyTitle>{message}</EmptyTitle>
      {description != null && <EmptyDescription>{description}</EmptyDescription>}
    </EmptyHeader>
  </Empty>
);
