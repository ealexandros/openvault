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

type BreadcrumbsProps = {
  currentPath: string[];
  onClick: (index: number) => void;
};

export const Breadcrumbs = ({ currentPath, onClick }: BreadcrumbsProps) => (
  <Breadcrumb>
    <BreadcrumbList>
      {currentPath.map((segment, index) => {
        const isLast = index === currentPath.length - 1;
        const isFirst = index === 0;

        const content = (
          <>
            {isFirst && <HomeIcon className="size-4 shrink-0" />}
            <span className="mt-0.5">{segment}</span>
          </>
        );

        return (
          <React.Fragment key={`${segment}-${index}`}>
            <BreadcrumbItem className="text-sm">
              {isLast ? (
                <BreadcrumbPage className="flex items-center gap-1.5 font-semibold text-foreground">
                  {content}
                </BreadcrumbPage>
              ) : (
                <BreadcrumbLink asChild>
                  <button
                    onClick={() => onClick(index)}
                    className="flex cursor-pointer items-center gap-1.5 transition-colors hover:text-foreground">
                    {content}
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
