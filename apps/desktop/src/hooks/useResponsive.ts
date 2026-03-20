import { useMediaQuery } from "./useMediaQuery";

export type Responsive = ReturnType<typeof useResponsive>;

export const Breakpoints = {
  xs: 440,
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  "2xl": 1536,
};

const MOBILE_BREAKPOINT = 768;

export const useResponsive = () => {
  const isMobileQuery = useMediaQuery({ maxWidth: MOBILE_BREAKPOINT - 1 });

  const isXs = useMediaQuery({ minWidth: Breakpoints.xs });
  const isSm = useMediaQuery({ minWidth: Breakpoints.sm });
  const isMd = useMediaQuery({ minWidth: Breakpoints.md });
  const isLg = useMediaQuery({ minWidth: Breakpoints.lg });
  const isXl = useMediaQuery({ minWidth: Breakpoints.xl });
  const is2xl = useMediaQuery({ minWidth: Breakpoints["2xl"] });

  return {
    isReady: is2xl.isReady,
    isMobile: isMobileQuery.matchesQuery,
    isXs: isXs.matchesQuery,
    isSm: isSm.matchesQuery,
    isMd: isMd.matchesQuery,
    isLg: isLg.matchesQuery,
    isXl: isXl.matchesQuery,
    is2xl: is2xl.matchesQuery,
  };
};
