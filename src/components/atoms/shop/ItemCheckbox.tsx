import { ShoppingListItemState } from "@/lib/types";
import { ActionIcon, NavLink, Popover } from "@mantine/core";
import { useClickOutside } from "@mantine/hooks";
import {
  IconArchive,
  IconCircle,
  IconCircleCheck,
  IconCircleOff,
} from "@tabler/icons-react";
import { useCallback, useRef, useState } from "react";
import { useTranslation } from "react-i18next";

export interface ItemCheckboxProps {
  state: ShoppingListItemState;
  onChange?: (state: ShoppingListItemState) => void;
}

const STATES = {
  [ShoppingListItemState.Todo]: {
    icon: IconCircle,
    color: "blue",
  },
  [ShoppingListItemState.InCart]: {
    icon: IconCircleCheck,
    color: "green",
  },
  [ShoppingListItemState.Bought]: {
    icon: IconArchive,
    color: "gray",
  },
  [ShoppingListItemState.NotBuying]: {
    icon: IconCircleOff,
    color: "red",
  },
};

export default function ItemCheckbox({
  state,
  onChange = () => {},
}: ItemCheckboxProps) {
  const { t } = useTranslation();
  const [menuOpened, setMenuOpened] = useState(false);
  const popoverRef = useClickOutside(() => setMenuOpened(false));

  const { icon: Icon, color } = STATES[state];

  const longPressTimer = useRef<number>();
  const isLongPress = useRef(false);

  const handleMouseDown = useCallback(() => {
    isLongPress.current = false;
    longPressTimer.current = window.setTimeout(() => {
      isLongPress.current = true;
      setMenuOpened(true);
    }, 1000);
  }, [onChange]);

  const handleMouseUp = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
    }
    if (!isLongPress.current) {
      onChange(circleState(state));
    }
  }, [state, onChange]);

  const menuOptions = Object.entries(STATES).map(
    ([newState, { icon: Icon, color }]) => (
      <NavLink
        key={newState}
        leftSection={<Icon />}
        rightSection={null}
        label={t(`atoms.shop.state.${newState.toLowerCase()}`)}
        onClick={() => {
          onChange(newState as ShoppingListItemState);
          setMenuOpened(false);
        }}
        active={true}
        color={color}
        variant={state !== newState ? "subtle" : "light"}
        p="xs"
      />
    ),
  );

  return (
    <Popover
      withArrow
      shadow="md"
      opened={menuOpened}
      middlewares={{ size: true }}
    >
      <Popover.Target>
        <ActionIcon
          variant="subtle"
          color={color}
          onMouseDown={handleMouseDown}
          onMouseUp={handleMouseUp}
        >
          <Icon />
        </ActionIcon>
      </Popover.Target>
      <Popover.Dropdown ref={popoverRef} p="0">
        {menuOptions}
      </Popover.Dropdown>
    </Popover>
  );
}

function circleState(current: ShoppingListItemState): ShoppingListItemState {
  switch (current) {
    case ShoppingListItemState.Todo:
      return ShoppingListItemState.InCart;
    default:
      return ShoppingListItemState.Todo;
  }
}
