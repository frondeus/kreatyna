import SuggestionItem from "@components/molecules/shop/SuggestionItem";
import { APIContext } from "@lib/api";
import { useSuggestions } from "@lib/state";
import { Drawer, Loader, Stack, TextInput } from "@mantine/core";
import { Suspense, useContext, useEffect, useState } from "react";

export interface AddItemModalProps {
  opened: boolean;
  onClose: () => void;
}

export default function AddItemModal({ opened, onClose }: AddItemModalProps) {
  const [query, setQuery] = useState("");

  const onSearch = (query: string) => {
    setQuery(query);
  };

  return (
    <Drawer
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
          placeholder="Add new item"
          size="md"
          value={query}
          onChange={async (e) => onSearch(e.currentTarget.value)}
        />
        <Suspense fallback={<Loader />}>
          <SuggestionItemList query={query} />
        </Suspense>
      </Stack>
    </Drawer>
  );
}

function SuggestionItemList({ query }: { query: string }) {
  const api = useContext(APIContext);
  const { fetch, refresh } = useSuggestions();
  useEffect(() => {
    refresh();
  }, [query]);
  const suggestions = fetch(() => api.getItemSuggestions(query));
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
          onEditUnit={() => {}}
        />
      ))}
    </Stack>
  );
}
