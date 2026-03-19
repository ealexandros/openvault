type RequestMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

type RequestOptions = {
  method?: RequestMethod;
  headers?: Record<string, string>;
  body?: Record<string, unknown>;
  params?: Record<string, string | number | boolean | undefined | null>;
  cache?: RequestCache;
  next?: NextFetchRequestConfig;
  signal?: AbortSignal;
};

const appendQueryParams = (baseUrl: string, params?: RequestOptions["params"]) => {
  const url = new URL(baseUrl);

  if (!params) return url.toString();

  for (const [key, value] of Object.entries(params)) {
    if (value != null) {
      url.searchParams.append(key, String(value));
    }
  }

  return url.toString();
};

const makeRequest = async (url: string, options: RequestOptions = {}) => {
  try {
    const completeUrl = appendQueryParams(url, options.params);

    const body = options.body ? JSON.stringify(options.body) : undefined;

    const config: RequestInit = {
      method: options.method ?? "GET",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        ...options.headers,
      },
      body,
      cache: options.cache ?? "no-store",
      signal: options.signal,
    };

    return await fetch(completeUrl, config);
  } catch {
    return null;
  }
};

const buildRequestWithoutBody = (method: RequestMethod) => {
  return (url: string, options?: RequestOptions) => {
    return makeRequest(url, { ...options, method });
  };
};

const buildRequestWithBody = (method: RequestMethod) => {
  return (url: string, body?: Record<string, unknown>, options?: RequestOptions) => {
    return makeRequest(url, { ...options, body, method });
  };
};

export const fetchApi = {
  get: buildRequestWithoutBody("GET"),
  post: buildRequestWithBody("POST"),
  put: buildRequestWithBody("PUT"),
  patch: buildRequestWithBody("PATCH"),
  delete: buildRequestWithoutBody("DELETE"),
};
