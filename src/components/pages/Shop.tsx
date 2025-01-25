import Item from "@/components/molecules/shop/Item";
import AddItemDrawer from "@/components/organisms/shop/AddItemDrawer";
import { APIContext } from "@/lib/api";
import { useShop } from "@/lib/state";
import {
  ShoppingList,
  ShoppingListItem,
  ShoppingListItemState,
} from "@/lib/types";
import { Button, Group } from "@mantine/core";
import { useForm, UseFormReturnType } from "@mantine/form";
import { useCounter } from "@mantine/hooks";
import { useContext, useEffect } from "react";
import { useTranslation } from "react-i18next";

import { updateForm } from "@/lib/utils";
import ListPage from "./ListPage";

export default function ShopPage() {
  const { fetch } = useShop();
  const [generation, handlers] = useCounter(0);
  const api = useContext(APIContext);
  const shoppingLists = fetch(api.getShoppingLists);
  const lastShoppingList = shoppingLists[shoppingLists.length - 1];

  const form = useForm<ShoppingList>({
    initialValues: lastShoppingList,
  });

  const handleFinishShopping = async () => {
    await api.finishShopping(lastShoppingList.id);
  };

  useEffect(() => {
    return api.subscribeToEvents((msg) => {
      if (msg.id !== lastShoppingList.id) return;
      console.log("Event", msg);
      switch (msg.kind) {
        case "updated": {
          updateForm(form, msg.changes);
          break;
        }
        default: {
          break;
        }
      }
      handlers.increment();
    });
  }, []);

  return (
    <ListPage
      renderModal={(params) => {
        return <AddItemDrawer generation={generation} {...params} />;
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
