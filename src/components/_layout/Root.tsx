import { MantineProvider } from "@mantine/core";
import { ErrorBoundary } from "react-error-boundary";
import { API, APIContext } from "@lib/api";
import ErrorPage from "./Error";

export interface RootProps {
    api: API;
    children: React.ReactNode;
}

export default function Root({api, children}: RootProps) {
    return (
        <ErrorBoundary FallbackComponent={ErrorPage}>
            <MantineProvider defaultColorScheme="dark">
                <APIContext.Provider value={api}>
                    {children}
                </APIContext.Provider>
            </MantineProvider>
        </ErrorBoundary>
    )
}