import { PropsWithChildren } from "react";

export type SkeletonProps = PropsWithChildren & {
  isLoading: boolean;
};

export type SVGProps = React.SVGProps<SVGSVGElement>;
