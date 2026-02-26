"use client";

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/shadcn/breadcrumb";
import React from "react";

type BreadcrumbsProps = {
  currentPath: string[];
  onClick: (index: number) => void;
};

export function Breadcrumbs({ currentPath, onClick }: BreadcrumbsProps) {
  return (
    <Breadcrumb>
      <BreadcrumbList>
        {currentPath.map((segment, i) => {
          const isLast = i === currentPath.length - 1;

          return (
            <React.Fragment key={i}>
              <BreadcrumbItem>
                {isLast ? (
                  <BreadcrumbPage>{segment}</BreadcrumbPage>
                ) : (
                  <BreadcrumbLink asChild className="cursor-pointer">
                    <button onClick={() => onClick(i)}>{segment}</button>
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
}
