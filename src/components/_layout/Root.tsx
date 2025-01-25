import "@/App.css";
import ErrorPage from "@/components/_layout/Error";
import { API, APIContext } from "@/lib/api";
import { MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";
import "@mantine/dates/styles.css";
import { Notifications } from "@mantine/notifications";
import "@mantine/notifications/styles.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "mantine-react-table/styles.css";
import { ErrorBoundary } from "react-error-boundary";

export interface RootProps {
  api?: API;
  children: React.ReactNode;
}

const queryClient = new QueryClient();
export default function Root({ api, children }: RootProps) {
  return (
    <ErrorBoundary FallbackComponent={ErrorPage}>
      <MantineProvider defaultColorScheme="dark">
        <QueryClientProvider client={queryClient}>
          <Notifications />
          {api && (
            <APIContext.Provider value={api}>{children}</APIContext.Provider>
          )}
          {!api && <>{children}</>}
        </QueryClientProvider>
      </MantineProvider>
    </ErrorBoundary>
  );
}
