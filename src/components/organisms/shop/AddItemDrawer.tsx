import AddItem from "@/components/molecules/shop/AddItem";
import EditUnitDrawer from "@/components/molecules/shop/EditUnitDrawer";
import { API, APIContext } from "@/lib/api";
import { AddShoppingListItem, ShoppingListItemState } from "@/lib/types";
import {
  Drawer,
  FocusTrap,
  Loader,
  Stack,
  TextInput,
  useDrawersStack,
} from "@mantine/core";
import { useSuspenseQuery } from "@tanstack/react-query";
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
  unit: string | undefined;
}

function useAPICalls(api: API, onClose: () => void) {
  const onAdd = async (item: AddShoppingListItem) => {
    const itemToAdd = {
      state: ShoppingListItemState.Todo,
      ...item,
    };
    await api.updateDocument(DOCUMENT_ID, [
      [`items`, { kind: "insert", value: itemToAdd, key: itemToAdd.id }],
    ]);
    onClose();
  };

  const onRemove = async (item: AddShoppingListItem) => {
    await api.updateDocument(DOCUMENT_ID, [
      [`items`, { kind: "remove", key: item.id }],
    ]);
  };

  const onSetUnit = async (id: string, unit: string | undefined) => {
    console.log("onSetUnit", id, unit);
    if (unit) {
      await api.updateDocument(DOCUMENT_ID, [
        [`items.${id}.unit`, { kind: "replace", value: unit }],
      ]);
    } else {
      await api.updateDocument(DOCUMENT_ID, [
        [`items.${id}.unit`, { kind: "replaceWithUndefined" }],
      ]);
    }
  };

  const onSetAmount = async (id: string, amount: number) => {
    console.log("onSetAmount", id, amount);
    await api.updateDocument(DOCUMENT_ID, [
      [`items.${id}.amount`, { kind: "replace", value: amount }],
    ]);
  };
  const onIncrease = async (item: AddShoppingListItem) => {
    await onSetAmount(item.id, item.amount + 1);
  };

  const onDecrease = async (item: AddShoppingListItem) => {
    await onSetAmount(item.id, item.amount - 1);
  };

  return { onAdd, onIncrease, onDecrease, onRemove, onSetUnit, onSetAmount };
}

export default function AddItemDrawer({ opened, ...props }: AddItemModalProps) {
  const { t } = useTranslation();
  const api = useContext(APIContext);
  const [query, setQuery] = useState("");
  const trimmedQuery = query.trim();
  const stack = useDrawersStack(["addItem", "editUnit"]);

  const [editedItem, setEditedItem] = useState<EditedItem | undefined>(
    undefined,
  );
  const [entry, setEntry] = useState<AddShoppingListItem | undefined>(
    undefined,
  );

  const onSearch = (query: string) => {
    setQuery(query);
  };

  const editUnit = (id: string, amount: number, unit: string | undefined) => {
    setEditedItem({ id, amount, unit });
    stack.open("editUnit");
  };

  const onClose = () => {
    setQuery("");
    props.onClose();
  };

  const apiCalls = useAPICalls(api, onClose);

  const onSubmit = () => {
    if (entry) {
      apiCalls.onAdd(entry);
    }
  };

  const onSetUnit = async (id: string, unit: string | undefined) => {
    await apiCalls.onSetUnit(id, unit);
    if (editedItem) {
      const newEditedItem = { ...editedItem };
      newEditedItem.unit = unit;
      setEditedItem(newEditedItem);
    }
  };

  const onSetAmount = async (id: string, amount: number) => {
    await apiCalls.onSetAmount(id, amount);
    if (editedItem) {
      setEditedItem({ ...editedItem, amount });
    }
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
        trapFocus={false}
        autoFocus={false}
        returnFocus={false}
      >
        <Stack gap="xs" justify="flex-end" align="stretch" h="100%">
          <FocusTrap active={opened}>
            <div>
              <TextInput
                placeholder={t("organisms.shop.add_item")}
                size="md"
                value={query}
                onChange={(e) => onSearch(e.currentTarget.value)}
                onKeyUp={(e) => {
                  if (e.key === "Enter") {
                    onSubmit();
                  }
                }}
                data-autofocus
                enterKeyHint="go"
              />
            </div>
          </FocusTrap>
          <Suspense fallback={<Loader />}>
            <SuggestionItemList
              query={trimmedQuery}
              editUnit={editUnit}
              entry={entry}
              setEntry={setEntry}
              onClose={onClose}
            />
          </Suspense>
        </Stack>
      </Drawer>
      <Drawer {...stack.register("editUnit")} position="bottom" size="25%">
        {editedItem && (
          <EditUnitDrawer
            id={editedItem.id}
            amount={editedItem.amount}
            unit={editedItem.unit}
            setUnit={onSetUnit}
            setAmount={onSetAmount}
          />
        )}
      </Drawer>
    </Drawer.Stack>
  );
}

interface SuggestionItemListProps {
  query: string;
  editUnit: (id: string, amount: number, unit: string | undefined) => void;
  entry: AddShoppingListItem | undefined;
  setEntry: (entry: AddShoppingListItem | undefined) => void;
  onClose: () => void;
}

function SuggestionItemList({
  query,
  editUnit,
  entry,
  setEntry,
  onClose,
}: SuggestionItemListProps) {
  const api = useContext(APIContext);
  const { data: items } = useSuspenseQuery({
    queryKey: ["items-or-suggestions", query],
    queryFn: () => api.getItemsAndSuggestions(query),
  });

  const { onAdd, onIncrease, onDecrease, onRemove } = useAPICalls(api, onClose);

  useEffect(() => {
    const itemMatchingQuery = items.find((s) => s.name === query);
    if (query.length > 0 && itemMatchingQuery === undefined) {
      setEntry({
        kind: "suggestion",
        id: uuidv4(),
        name: query,
        amount: 1,
      });
    } else if (itemMatchingQuery !== undefined) {
      setEntry(itemMatchingQuery);
    } else {
      setEntry(undefined);
    }
  }, [query, items]);

  return (
    <Stack gap="xs">
      {entry && (
        <AddItem
          key={entry.id}
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
        .map((item) => (
          <AddItem
            key={item.id}
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
