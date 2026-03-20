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
    secrets: {
      get: () => "/dashboard/secrets",
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
