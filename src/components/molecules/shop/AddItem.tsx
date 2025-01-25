import { AddShoppingListItem } from "@/lib/types";
import EditItem from "./EditItem";
import SuggestionItem from "./SuggestionItem";

export interface AddItemProps {
  item: AddShoppingListItem;
  onIncrease: () => void;
  onDecrease: () => void;
  onRemove: () => void;
  onAdd: () => void;
  onEditUnit: () => void;
}

export default function AddItem({
  item,
  onIncrease,
  onDecrease,
  onRemove,
  onEditUnit,
  onAdd,
}: AddItemProps) {
  if (item.kind === "suggestion") {
    return <SuggestionItem suggestion={item} onAdd={onAdd} />;
  }
  return (
    <EditItem
      item={item}
      onIncrease={onIncrease}
      onDecrease={onDecrease}
      onRemove={onRemove}
      onEditUnit={onEditUnit}
    />
  );
}
