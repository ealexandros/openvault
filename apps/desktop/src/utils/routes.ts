import { match } from "path-to-regexp";

export const matchRoute = (pathname: string, pattern: string) => {
  const matcher = match(pattern, { decode: decodeURIComponent });
  return matcher(pathname) !== false;
};

export const matchRoutes = (pathname: string, patterns: string[]) => {
  return patterns.some(pattern => matchRoute(pathname, pattern));
};

export const routeJoin = (base: string, ...paths: (string | undefined)[]) => {
  const formatBase = base.replace(/\/+$/, "");
  const formatPaths = paths.filter(Boolean).map(p => p!.replace(/(^\/+|\/+$)/g, ""));
  return [formatBase, ...formatPaths].join("/");
};
