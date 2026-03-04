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

    browse: {
      get: () => "/dashboard/browse",
    },
    decoy: {
      get: () => "/dashboard/decoy",
    },
    notes: {
      get: () => "/dashboard/notes",
    },
    passwords: {
      get: () => "/dashboard/passwords",
    },
    messages: {
      get: () => "/dashboard/messages",
    },
    logs: {
      get: () => "/dashboard/logs",
    },
    settings: {
      get: () => "/dashboard/settings",
    },
  },
};
