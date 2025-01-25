import Item from "@/components/molecules/shop/Item";
import AddItemDrawer from "@/components/organisms/shop/AddItemDrawer";
import { APIContext } from "@/lib/api";
import { useShop } from "@/lib/state";
import {
  ShoppingList,
  ShoppingListItem,
  ShoppingListItemState,
} from "@/lib/types";
import {
  ActionIcon,
  Affix,
  Button,
  Group,
  ScrollArea,
  Stack,
  useMantineTheme,
} from "@mantine/core";
import { useForm, UseFormReturnType } from "@mantine/form";
import { useCounter, useDisclosure } from "@mantine/hooks";
import { IconPlus } from "@tabler/icons-react";
import { useContext, useEffect } from "react";
import { useTranslation } from "react-i18next";

export default function ShopPage() {
  const { t } = useTranslation();
  const { fetch } = useShop();
  const theme = useMantineTheme();
  const [
    openedAddingModal,
    { toggle: toggleAddingModal, close: closeAddingModal },
  ] = useDisclosure(false);
  const [generation, handlers] = useCounter(0);
  const api = useContext(APIContext);
  const shoppingLists = fetch(api.getShoppingLists);
  const lastShoppingList = shoppingLists[shoppingLists.length - 1];

  const form = useForm<ShoppingList>({
    initialValues: lastShoppingList,
  });

  useEffect(() => {
    return api.subscribeToEvents((msg) => {
      if (msg.id !== lastShoppingList.id) return;
      console.log("Event", msg);
      switch (msg.kind) {
        case "updated": {
          for (const [path, change] of msg.changes) {
            switch (change.kind) {
              case "insert": {
                form.insertListItem(path, change.value, change.position);
                break;
              }
              case "move": {
                form.reorderListItem(path, {
                  from: change.from,
                  to: change.to,
                });
                break;
              }
              case "remove": {
                form.removeListItem(path, change.index);
                break;
              }
              case "replace": {
                form.setFieldValue(path, change.value);
                break;
              }
              case "replaceWithUndefined": {
                form.setFieldValue(path, undefined);
                break;
              }
              default:
                break;
            }
          }
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
    <Stack bg="dark.8" h="100%" gap="0">
      <ScrollArea>
        <ShoppingListForm form={form} />
        <Group mt="md" p="md" mb="128px" grow>
          <Button size="md" color="green">
            {t("pages.shop.finish_shopping")}
          </Button>
        </Group>
      </ScrollArea>
      <Affix position={{ bottom: 78, right: 20 }}>
        <ActionIcon onClick={toggleAddingModal} size="xl" radius="xl">
          <IconPlus />
        </ActionIcon>
      </Affix>
      <AddItemDrawer
        opened={openedAddingModal}
        onClose={closeAddingModal}
        generation={generation}
      />
    </Stack>
  );
}

interface ShoppingListFormProps {
  form: UseFormReturnType<ShoppingList>;
}

function ShoppingListForm({ form }: ShoppingListFormProps) {
  const items = form.getInputProps("items");
  const values: ShoppingListItem[] = items.value ?? items.defaultValue ?? [];
  // Doing shallow copy to avoid mutating original items.value;
  const sortedValues = [...values].sort(compareItems);
  return (
    <>
      {sortedValues.map((item) => (
        <Item key={item.id} item={item} />
      ))}
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
