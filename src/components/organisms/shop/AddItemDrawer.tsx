import EditUnitDrawer from "@/components/molecules/shop/EditUnitDrawer";
import SuggestionItem from "@components/molecules/shop/SuggestionItem";
import { APIContext } from "@lib/api";
import { useSuggestions } from "@lib/state";
import {
  Drawer,
  Loader,
  Stack,
  TextInput,
  useDrawersStack,
} from "@mantine/core";
import { Suspense, useContext, useState } from "react";
import { useTranslation } from "react-i18next";

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
            <SuggestionItemList query={query} editUnit={editUnit} />
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
  const { fetchWithArgs } = useSuggestions();
  const suggestions = fetchWithArgs(api.getItemSuggestions, query);

  return (
    <Stack gap="xs">
      {suggestions.map((item, index) => (
        <SuggestionItem
          key={index}
          {...item}
          onDecrease={() => {}}
          onIncrease={() => {}}
          onRemove={() => {}}
          onAdd={() => {}}
          // Safety: If we are running this function, then it means the item is already in the list
          // So it must have an id
          onEditUnit={() => editUnit(item.id!, item.amount, item.unit)}
        />
      ))}
    </Stack>
  );
}
