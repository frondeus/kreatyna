import ErrorPage from "@/components/_layout/Error";
import RawIngredientList from "@/components/pages/RawIngredientList";
import ShopPage from "@/components/pages/Shop";
import { useRoute } from "@/lib/state";
import { Loader } from "@mantine/core";
import { Suspense } from "react";
import { ErrorBoundary } from "react-error-boundary";
import EditRawIngredient from "../pages/EditRawIngredient";
import RecipesList from "../pages/RecipesList";

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
  const route = useRoute((st) => st.route);

  if (route === "shop") {
    return <ShopPage />;
  }

  if (route === "raw-ingredient-list") {
    return <RawIngredientList />;
  }

  if (route === "recipe-list") {
    return <RecipesList />;
  }

  if (route.type === "raw-ingredient") {
    return <EditRawIngredient id={route.id} />;
  }

  return null;
}
