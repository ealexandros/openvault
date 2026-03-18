import { DefaultOptions, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { minutesToMilliseconds } from "date-fns";
import { PropsWithChildren } from "react";

const queryConfig: DefaultOptions = {
  queries: {
    refetchOnWindowFocus: false,
    retry: false,
    staleTime: minutesToMilliseconds(60),
  },
};

export const queryClient = new QueryClient({ defaultOptions: queryConfig });

export const ReactQueryProvider = ({ children }: PropsWithChildren) => (
  <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
);
