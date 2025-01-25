import { Combobox, TextInput } from "@mantine/core";
import { useEffect } from "react";

import { useAPI } from "@/lib/api";
import { Document, DocumentKind } from "@/lib/types";
import { useCombobox } from "@mantine/core";
import { IconSearch } from "@tabler/icons-react";
import { useSuspenseQuery } from "@tanstack/react-query";
import { useState } from "react";
import { useTranslation } from "react-i18next";

interface SearchForProps {
  kind: DocumentKind;
  onSelect: (document: Document | string) => void;
}

export default function SearchFor({ kind, onSelect }: SearchForProps) {
  const { t } = useTranslation();
  const [value, setValue] = useState<string>("");
  const combobox = useCombobox({
    onDropdownClose: () => {
      combobox.resetSelectedOption();
      if (value.length > 0) {
        onSelect(value);
      }
    },
  });
  const api = useAPI();
  const { data } = useSuspenseQuery({
    queryKey: ["search_for", kind],
    queryFn: () => api.getDocuments(kind),
  });

  const shouldFilterOptions = !data.some((item) => item.title === value);
  const filteredOptions = shouldFilterOptions
    ? data.filter((item) =>
        item.title.toLowerCase().includes(value.toLowerCase().trim()),
      )
    : data;

  const options = filteredOptions.map((item) => (
    <Combobox.Option value={item.id.toString()} key={item.id}>
      {item.title}
    </Combobox.Option>
  ));

  useEffect(() => {
    // we need to wait for options to render before we can select first one
    combobox.selectFirstOption();
  }, [value]);

  return (
    <Combobox
      onOptionSubmit={(optionValue) => {
        const id = parseInt(optionValue);
        const option = filteredOptions.find((item) => item.id === id)!;
        setValue(option.title);
        onSelect(option);
        combobox.closeDropdown();
      }}
      store={combobox}
    >
      <Combobox.Target>
        <TextInput
          label={t(`molecules.search_for.${kind}.label`)}
          placeholder={t(`molecules.search_for.${kind}.placeholder`)}
          leftSection={<IconSearch />}
          value={value}
          onChange={(event) => {
            setValue(event.currentTarget.value);
            combobox.openDropdown();
          }}
          onClick={() => combobox.openDropdown()}
          onFocus={() => combobox.openDropdown()}
          onBlur={() => combobox.closeDropdown()}
        />
      </Combobox.Target>

      <Combobox.Dropdown>
        <Combobox.Options mah={200} style={{ overflowY: "auto" }}>
          {options.length === 0 ? (
            <Combobox.Empty>{t(`molecules.search_for.empty`)}</Combobox.Empty>
          ) : (
            options
          )}
        </Combobox.Options>
      </Combobox.Dropdown>
    </Combobox>
  );
}
