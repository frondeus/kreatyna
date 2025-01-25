import { APIContext } from "@/lib/api";
import { useRawIngredients } from "@/lib/state";
import { RawIngredient } from "@/lib/types";
import { NavLink, TextInput } from "@mantine/core";
import { IconSearch } from "@tabler/icons-react";
import { useContext, useState } from "react";
import ListPage from "./ListPage";

export default function RawIngredientList() {
  const api = useContext(APIContext);
  const fetch = useRawIngredients((s) => s.fetch);
  const rawIngredients = fetch(api.getRawIngredients);

  return (
    <ListPage
      renderModal={(_params) => {
        return <></>;
      }}
    >
      <Inner rawIngredients={rawIngredients} />
    </ListPage>
  );
}

function Inner({ rawIngredients }: { rawIngredients: RawIngredient[] }) {
  const [search, setSearch] = useState("");

  const filtered = rawIngredients.filter((ri) =>
    ri.name.toLowerCase().includes(search.toLowerCase()),
  );

  return (
    <>
      <TextInput
        radius="0"
        placeholder="Search"
        leftSection={<IconSearch />}
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />
      {filtered.map((ri) => {
        return <NavLink key={ri.id} label={ri.name} />;
      })}
    </>
  );
}
