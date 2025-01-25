import Item from "@/components/molecules/shop/Item";
import AddItemDrawer from "@/components/organisms/shop/AddItemDrawer";
import { useAPI } from "@/lib/api";
import {
  ShoppingList,
  ShoppingListItem,
  ShoppingListItemState,
} from "@/lib/types";
import { Button, Group } from "@mantine/core";
import { useForm, UseFormReturnType } from "@mantine/form";
import { useTranslation } from "react-i18next";

import { useSubscribeForm } from "@/lib/utils";
import { useQueryClient, useSuspenseQuery } from "@tanstack/react-query";
import ListPage from "./ListPage";

export default function ShopPage() {
  const api = useAPI();
  const { data: shoppingLists } = useSuspenseQuery({
    queryKey: ["shopping-lists"],
    queryFn: () => api.getDocuments("shoppingList"),
  });
  const lastShoppingList = shoppingLists[shoppingLists.length - 1];
  if (lastShoppingList.kind !== "shoppingList") {
    throw new Error("Last document is not a shopping list");
  }

  const form = useForm<ShoppingList>({
    initialValues: lastShoppingList,
  });

  const queryClient = useQueryClient();
  useSubscribeForm(lastShoppingList.id, api, form, () => {
    queryClient.invalidateQueries({
      queryKey: ["items-or-suggestions"],
    });
  });

  const handleFinishShopping = async () => {
    await api.finishShopping(lastShoppingList.id);
  };

  return (
    <ListPage
      renderModal={(params) => {
        return <AddItemDrawer {...params} />;
      }}
    >
      <ShoppingListForm form={form} onFinishShopping={handleFinishShopping} />
    </ListPage>
  );
}

interface ShoppingListFormProps {
  form: UseFormReturnType<ShoppingList>;
  onFinishShopping: () => Promise<void>;
}

function ShoppingListForm({ form, onFinishShopping }: ShoppingListFormProps) {
  const { t } = useTranslation();
  const items = form.getInputProps("items");
  const values: ShoppingListItem[] = items.value ?? items.defaultValue ?? [];
  // Doing shallow copy to avoid mutating original items.value;
  const sortedValues = [...values].sort(compareItems);
  const hasAtLeastOneItemInCart = sortedValues.some(
    (item) => item.state === ShoppingListItemState.InCart,
  );
  return (
    <>
      {sortedValues.map((item) => (
        <Item key={item.id} item={item} />
      ))}
      <Group mt="md" p="md" mb="128px" grow>
        <Button
          size="md"
          color="green"
          onClick={onFinishShopping}
          disabled={!hasAtLeastOneItemInCart}
        >
          {t("pages.shop.finish_shopping")}
        </Button>
      </Group>
    </>
  );
}

function compareItems(a: ShoppingListItem, b: ShoppingListItem) {
  // 1. Sort by state
  const stateOrder: ShoppingListItemState[] = [
    ShoppingListItemState.Todo,
    ShoppingListItemState.NotBuying,
    ShoppingListItemState.InCart,
    ShoppingListItemState.Bought,
  ];
  const stateA = stateOrder.indexOf(a.state);
  const stateB = stateOrder.indexOf(b.state);
  if (stateA !== stateB) return stateA - stateB;

  // 2. Sort by name
  return a.name.localeCompare(b.name);
}
