import AddItem from "@/components/molecules/shop/AddItem";
import EditUnitDrawer from "@/components/molecules/shop/EditUnitDrawer";
import { AddShoppingListItem, ShoppingListItemState } from "@/lib/types";
import { APIContext } from "@lib/api";
import { useItemsOrSuggestions } from "@lib/state";
import {
  Drawer,
  Loader,
  Stack,
  TextInput,
  useDrawersStack,
} from "@mantine/core";
import { Suspense, useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { v4 as uuidv4 } from "uuid";

const DOCUMENT_ID = 1;

export interface AddItemModalProps {
  opened: boolean;
  onClose: () => void;
}

interface EditedItem {
  id: string;
  amount: number;
  unit?: string;
}

export default function AddItemDrawer({ opened, onClose }: AddItemModalProps) {
  const { t } = useTranslation();
  const [query, setQuery] = useState("");
  const trimmedQuery = query.trim();
  const stack = useDrawersStack(["addItem", "editUnit"]);
  const [editedItem, setEditedItem] = useState<EditedItem | undefined>(
    undefined,
  );

  const onSearch = (query: string) => {
    setQuery(query);
  };

  const editUnit = (id: string, amount: number, unit?: string) => {
    setEditedItem({ id, amount, unit });
    stack.open("editUnit");
  };

  return (
    <Drawer.Stack>
      <Drawer
        {...stack.register("addItem")}
        size="100%"
        position="bottom"
        opened={opened}
        onClose={onClose}
        styles={{
          header: {
            marginBottom: 0,
          },
        }}
      >
        <Stack gap="xs" justify="flex-end" align="stretch" h="100%">
          <TextInput
            placeholder={t("organisms.shop.add_item")}
            size="md"
            value={query}
            onChange={async (e) => onSearch(e.currentTarget.value)}
          />
          <Suspense fallback={<Loader />}>
            <SuggestionItemList query={trimmedQuery} editUnit={editUnit} />
          </Suspense>
        </Stack>
      </Drawer>
      <Drawer {...stack.register("editUnit")} position="bottom" size="25%">
        {editedItem && (
          <EditUnitDrawer
            id={editedItem.id}
            amount={editedItem.amount}
            unit={editedItem.unit}
            setUnit={(_id, _unit) => {}}
            setAmount={(_id, _amount) => {}}
          />
        )}
      </Drawer>
    </Drawer.Stack>
  );
}

interface SuggestionItemListProps {
  query: string;
  editUnit: (id: string, amount: number, unit?: string) => void;
}

function SuggestionItemList({ query, editUnit }: SuggestionItemListProps) {
  const api = useContext(APIContext);
  const [entry, setEntry] = useState<AddShoppingListItem | undefined>(
    undefined,
  );
  const { fetchWithArgs } = useItemsOrSuggestions();
  const items = fetchWithArgs(api.getItemsAndSuggestions, query);

  useEffect(() => {
    const itemMatchingQuery = items.find((s) => s.name === query);
    if (query.length > 0 && itemMatchingQuery === undefined) {
      setEntry({
        kind: "suggestion",
        id: uuidv4(),
        name: query,
        amount: 0,
      });
    } else if (itemMatchingQuery !== undefined) {
      setEntry(itemMatchingQuery);
    } else {
      setEntry(undefined);
    }
  }, [query, items]);

  const onIncrease = (item: AddShoppingListItem) => {
    api.updateDocument(DOCUMENT_ID, [
      [`items.${item.id}.amount`, { kind: "replace", data: item.amount + 1 }],
    ]);
  };

  const onDecrease = (item: AddShoppingListItem) => {
    api.updateDocument(DOCUMENT_ID, [
      [`items.${item.id}.amount`, { kind: "replace", data: item.amount - 1 }],
    ]);
  };

  const onRemove = (item: AddShoppingListItem) => {
    api.updateDocument(DOCUMENT_ID, [
      [`items`, { kind: "remove", key: item.id }],
    ]);
  };

  const onAdd = (item: AddShoppingListItem) => {
    const itemToAdd = {
      state: ShoppingListItemState.Todo,
      ...item,
    };
    api.updateDocument(DOCUMENT_ID, [
      [`items`, { kind: "insert", value: itemToAdd, key: itemToAdd.id }],
    ]);
  };

  return (
    <Stack gap="xs">
      {entry && (
        <AddItem
          item={entry}
          onIncrease={() => onIncrease(entry)}
          onDecrease={() => onDecrease(entry)}
          onRemove={() => onRemove(entry)}
          onAdd={() => onAdd(entry)}
          onEditUnit={() => editUnit(entry.id, entry.amount, entry.unit)}
        />
      )}

      {items
        .filter((item) => item.id !== entry?.id)
        .map((item, index) => (
          <AddItem
            key={index}
            item={item}
            onIncrease={() => onIncrease(item)}
            onDecrease={() => onDecrease(item)}
            onRemove={() => onRemove(item)}
            onAdd={() => onAdd(item)}
            onEditUnit={() => editUnit(item.id, item.amount, item.unit)}
          />
        ))}
    </Stack>
  );
}
