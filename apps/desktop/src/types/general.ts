import { PropsWithChildren } from "react";

export type SkeletonProps = PropsWithChildren & {
  isLoading: boolean;
};
