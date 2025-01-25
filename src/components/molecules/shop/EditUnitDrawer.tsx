import {
  ActionIcon,
  Button,
  Group,
  NumberInput,
  TextInput,
} from "@mantine/core";
import { IconX } from "@tabler/icons-react";
import { useTranslation } from "react-i18next";

export interface EditUnitDrawerProps {
  id: string;
  amount: number;
  unit: string | undefined;
  setUnit: (id: string, unit: string | undefined) => void;
  setAmount: (id: string, amount: number) => void;
}
export default function EditUnitDrawer({
  id,
  amount,
  unit,
  setUnit,
  setAmount,
}: EditUnitDrawerProps) {
  const { t } = useTranslation();
  return (
    <Group grow align="flex-end">
      <NumberInput
        label={t("molecules.shop.amount")}
        value={amount}
        onChange={(e) => setAmount(id, parseFloat(e.toString()))}
      />
      <EditUnitForm unit={unit} setUnit={(u) => setUnit(id, u)} />
    </Group>
  );
}

interface EditUnitFormProps {
  unit: string | undefined;
  setUnit: (unit: string | undefined) => void;
}
function EditUnitForm({ unit, setUnit }: EditUnitFormProps) {
  const { t } = useTranslation();

  if (!unit) {
    const addUnit = () => {
      setUnit("g");
    };
    return <Button onClick={addUnit}>{t("molecules.shop.add_unit")}</Button>;
  }

  return (
    <TextInput
      label={t("molecules.shop.unit")}
      value={unit}
      onChange={(e) => setUnit(e.currentTarget.value)}
      rightSection={
        <ActionIcon onClick={() => setUnit(undefined)}>
          <IconX size="16" />
        </ActionIcon>
      }
    />
  );
}
