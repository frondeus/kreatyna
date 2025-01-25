import AddItemDrawer from "@/components/organisms/shop/AddItemDrawer";
import Item from "@components/molecules/shop/Item";
import { APIContext } from "@lib/api";
import { useShop } from "@lib/state";
import { ActionIcon, Affix, Stack } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { IconPlus } from "@tabler/icons-react";
import { useContext } from "react";

export default function ShopPage() {
  const { fetch } = useShop();
  const [
    openedAddingModal,
    { toggle: toggleAddingModal, close: closeAddingModal },
  ] = useDisclosure(false);
  const api = useContext(APIContext);
  const shoppingLists = fetch(api.getShoppingLists);
  const lastShoppingList = shoppingLists[shoppingLists.length - 1];

  return (
    <Stack>
      {lastShoppingList.items.map((item) => (
        <Item key={item.id} item={item} />
      ))}
      <Affix position={{ bottom: 78, right: 20 }}>
        <ActionIcon onClick={toggleAddingModal} size="xl" radius="xl">
          <IconPlus />
        </ActionIcon>
      </Affix>
      <AddItemDrawer opened={openedAddingModal} onClose={closeAddingModal} />
    </Stack>
  );
}
