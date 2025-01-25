import { APIContext } from "@/lib/api";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import ErrorPage from "../_layout/Error";

export interface EditRawIngredientProps {
  id: string;
}

export default function EditRawIngredient({ id }: EditRawIngredientProps) {
  const api = useContext(APIContext);
  const { data } = useSuspenseQuery({
    queryKey: ["raw-ingredient", id],
    queryFn: () => api.getDocument(parseInt(id)),
  });

  if (data.kind !== "rawIngredient") {
    return <ErrorPage error={new Error("Document is not a raw ingredient")} />;
  }

  return (
    <div>
      <h1>{data.name}</h1>
    </div>
  );
}
