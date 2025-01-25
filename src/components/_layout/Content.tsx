import ErrorPage from "@components/_layout/Error";
import ShopPage from "@components/pages/Shop";
import { useRoute } from "@lib/state";
import { Loader } from "@mantine/core";
import { Suspense } from "react";
import { ErrorBoundary } from "react-error-boundary";

export default function Content() {
  return (
    <ErrorBoundary FallbackComponent={ErrorPage}>
      <Suspense fallback={<Loader />}>
        <ContentInner />
      </Suspense>
    </ErrorBoundary>
  );
}

function ContentInner() {
  const { route } = useRoute();
  return <>{route === "shop" && <ShopPage />}</>;
}
