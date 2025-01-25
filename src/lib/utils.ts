import { UseFormReturnType } from "@mantine/form";
import { useEffect } from "react";
import { API } from "./api";
import {
  DocumentId,
  EventNotification,
  EventNotificationChange,
} from "./types";

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

export function useSubscribeForm<T>(
  id: DocumentId,
  api: API,
  form: UseFormReturnType<T>,
  cbk?: (msg: EventNotification) => void,
) {
  useEffect(() => {
    return api.subscribeToDocumentEvents(id, (msg) => {
      switch (msg.kind) {
        case "updated": {
          updateForm(form, msg.changes);
          if (cbk) cbk(msg);
          break;
        }
        default: {
          break;
        }
      }
    });
  }, [id]);
}

export function onScalarChange<
  T extends { id: DocumentId },
  I extends { value: unknown },
>(form: UseFormReturnType<T>, api: API, path: string) {
  const id = form.getValues().id;
  return (e: React.ChangeEvent<I> | string | number) => {
    const value =
      typeof e === "string" || typeof e === "number" ? e : e.target.value;
    api.updateDocument(id, [[path, { kind: "replace", value: value }]]);
  };
}
