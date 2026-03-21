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

    home: {
      get: () => "/dashboard/browse",
    },
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

  github: {
    get: () => "https://github.com/ealexandros/openvault",

    issue: {
      get: () => "https://github.com/ealexandros/openvault/issues",
    },
  },
};
