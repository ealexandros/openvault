"use client";

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/shadcn/breadcrumb";
import { HomeIcon } from "lucide-react";
import React from "react";

type BrowseBreadcrumbsProps = {
  currentPath: string[];
  onPathClick: (index: number) => void;
};

export const BrowseBreadcrumbs = ({ currentPath, onPathClick }: BrowseBreadcrumbsProps) => {
  if (!currentPath.length) return null;

  const lastIndex = currentPath.length - 1;
  const lastSegment = currentPath[lastIndex];
  const clickableSegments = currentPath.slice(0, lastIndex);

  return (
    <Breadcrumb>
      <BreadcrumbList className="text-sm">
        {clickableSegments.map((segment, index) => (
          <React.Fragment key={`${segment}-${index}`}>
            <BreadcrumbItem>
              <BreadcrumbLink asChild>
                <button
                  onClick={() => onPathClick(index)}
                  className="flex cursor-pointer items-center gap-1.5 transition-colors hover:text-foreground">
                  {index === 0 && <HomeIcon className="size-4 shrink-0" />}
                  <span>{segment}</span>
                </button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
          </React.Fragment>
        ))}
        <BreadcrumbItem>
          <BreadcrumbPage className="flex items-center gap-1.5 font-semibold text-foreground">
            {lastIndex === 0 && <HomeIcon className="size-4 shrink-0" />}
            <span>{lastSegment}</span>
          </BreadcrumbPage>
        </BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
  );
};
