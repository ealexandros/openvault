import { Button } from "@/components/ui/shadcn/button";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/shadcn/empty";
import { hrefs } from "@/config/hrefs";
import { ConstructionIcon, HomeIcon } from "lucide-react";
import Link from "next/link";

type UnderConstructionProps = {
  title: string;
};

export const UnderConstruction = ({ title }: UnderConstructionProps) => (
  <Empty className="flex h-screen items-center justify-center pb-16">
    <EmptyHeader>
      <EmptyMedia variant="icon">
        <ConstructionIcon />
      </EmptyMedia>
      <EmptyTitle>Under Construction</EmptyTitle>
      <EmptyDescription>
        The &quot;{title}&quot; will be with you shortly. Please stay tuned for next versions.
      </EmptyDescription>
    </EmptyHeader>
    <EmptyContent className="flex-row justify-center gap-2">
      <Button className="h-8 px-3" asChild>
        <Link href={hrefs.dashboard.home.get()}>
          <HomeIcon className="size-3.5" />
          <span>Go to Browse</span>
        </Link>
      </Button>
    </EmptyContent>
  </Empty>
);
