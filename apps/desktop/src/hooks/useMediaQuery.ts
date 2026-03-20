import { useEffect, useState } from "react";

type UseMediaQueryProps = {
  maxWidth?: number;
  minWidth?: number;
};

export const useMediaQuery = ({ maxWidth, minWidth }: UseMediaQueryProps) => {
  const [matchesQuery, setMatchesQuery] = useState<boolean>();

  useEffect(() => {
    if (typeof window === "undefined") return;

    const parts: string[] = [];
    if (minWidth !== undefined) parts.push(`(min-width: ${minWidth}px)`);
    if (maxWidth !== undefined) parts.push(`(max-width: ${maxWidth}px)`);

    if (parts.length === 0) return;
    const query = parts.join(" and ");

    const mediaQueryList = window.matchMedia(query);
    const updateMatch = () => setMatchesQuery(mediaQueryList.matches);

    updateMatch();
    mediaQueryList.addEventListener("change", updateMatch);

    return () => {
      mediaQueryList.removeEventListener("change", updateMatch);
    };
  }, [minWidth, maxWidth]);

  return {
    isReady: matchesQuery !== undefined,
    matchesQuery: matchesQuery ?? false,
  };
};
