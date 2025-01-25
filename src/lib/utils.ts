import { UseFormReturnType } from "@mantine/form";
import { EventNotificationChange } from "./types";

export function updateForm<T>(
  form: UseFormReturnType<T>,
  changes: [string, EventNotificationChange][],
) {
  const _form = form as UseFormReturnType<unknown>;
  for (const [path, change] of changes) {
    switch (change.kind) {
      case "insert": {
        _form.insertListItem(path, change.value, change.position);
        break;
      }
      case "move": {
        _form.reorderListItem(path, {
          from: change.from,
          to: change.to,
        });
        break;
      }
      case "remove": {
        _form.removeListItem(path, change.index);
        break;
      }
      case "replace": {
        _form.setFieldValue(path, change.value);
        break;
      }
      case "replaceWithUndefined": {
        _form.setFieldValue(path, undefined);
        break;
      }
      default:
        break;
    }
  }
}
