import ErrorPage from "@components/_layout/Error";
import { API, APIContext } from "@lib/api";
import { MantineProvider } from "@mantine/core";
import { ErrorBoundary } from "react-error-boundary";

export interface RootProps {
  api: API;
  children: React.ReactNode;
}

export default function Root({ api, children }: RootProps) {
  return (
    <ErrorBoundary FallbackComponent={ErrorPage}>
      <MantineProvider defaultColorScheme="dark">
        <APIContext.Provider value={api}>{children}</APIContext.Provider>
      </MantineProvider>
    </ErrorBoundary>
  );
}
