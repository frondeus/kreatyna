import ErrorPage from "@/components/_layout/Error";
import EditRawIngredient from "@/components/pages/EditRawIngredient";
import EditRecipe from "@/components/pages/EditRecipe";
import RawIngredientList from "@/components/pages/RawIngredientList";
import RecipesList from "@/components/pages/RecipesList";
import ShopPage from "@/components/pages/Shop";
import { Route, useRoute } from "@/lib/state";
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
  const route: Route = useRoute((st) => st.route);

  if (route === "shop") {
    return <ShopPage />;
  }

  if (route === "raw-ingredient-list") {
    return <RawIngredientList />;
  }

  if (route === "recipe-list") {
    return <RecipesList />;
  }

  if (route.type === "recipe") {
    return <EditRecipe id={route.id} />;
  }

  if (route.type === "raw-ingredient") {
    return <EditRawIngredient id={route.id} />;
  }

  return null;
}
