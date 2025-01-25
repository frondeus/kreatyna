import { ErrorBoundary } from "react-error-boundary";
import { ShopPage } from "./pages/Shop";
import { useRoute } from "./state";
import { Suspense } from "react";
import { Loader } from "@mantine/core";
import { ErrorPage } from "./layout/Error";

export function Content() {

    return <ErrorBoundary FallbackComponent={ErrorPage}>
        <Suspense fallback={<Loader />}>
            <ContentInner />
        </Suspense>
    </ErrorBoundary>
}

function ContentInner() {
    const { route } = useRoute();
    return <>
        {route === "shop" && <ShopPage />}
    </>
}