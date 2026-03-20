"use client";

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/shadcn/breadcrumb";
import { PathSegment } from "@/features/dashboard/browse/types";
import React from "react";

type BrowseBreadcrumbsProps = {
  pathSegments: PathSegment[];
  onPathClick: (index: number) => void;
};

export const BrowseBreadcrumbs = ({ pathSegments, onPathClick }: BrowseBreadcrumbsProps) => {
  if (pathSegments.length === 0) return null;

  return (
    <Breadcrumb>
      <BreadcrumbList className="text-sm">
        {pathSegments.map((segment, index) => {
          const isLast = index === pathSegments.length - 1;
          const Icon = segment.icon;

          return (
            <React.Fragment key={segment.id ?? `${segment.name}-${index}`}>
              <BreadcrumbItem>
                {isLast ? (
                  <BreadcrumbPage className="flex items-center gap-1.5 font-semibold text-foreground">
                    {Icon && <Icon className="size-4 shrink-0" />}
                    <span>{segment.name}</span>
                  </BreadcrumbPage>
                ) : (
                  <BreadcrumbLink asChild>
                    <button
                      onClick={() => onPathClick(index)}
                      className="flex cursor-pointer items-center gap-1.5 transition-colors hover:text-foreground">
                      {Icon && <Icon className="size-4 shrink-0" />}
                      <span>{segment.name}</span>
                    </button>
                  </BreadcrumbLink>
                )}
              </BreadcrumbItem>

              {!isLast && <BreadcrumbSeparator />}
            </React.Fragment>
          );
        })}
      </BreadcrumbList>
    </Breadcrumb>
  );
};
