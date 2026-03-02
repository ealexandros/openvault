export const hrefs = {
  home: {
    get: () => "/",
  },
  create: {
    get: () => "/create",
  },
  samesite: {
    get: () => "#",
  },
  dashboard: {
    get: () => "/dashboard",
    browse: "/dashboard/browse",
    decoy: "/dashboard/decoy",
    notes: "/dashboard/notes",
    passwords: "/dashboard/passwords",
    logs: "/dashboard/logs",
    settings: "/dashboard/settings",
  },
};
