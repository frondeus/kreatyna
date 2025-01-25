import { ActionIcon, Affix, Button, Center, Modal, Paper, Stack, Text, TextInput } from "@mantine/core";
import { APIContext } from "../../api";
import { useTranslation } from "react-i18next";
import { useShop } from "../../state";
import { Item } from "./components/Item";
import { IconPlus } from "@tabler/icons-react";
import { useDisclosure } from "@mantine/hooks";
import { useContext } from "react";
import { AddItemModal } from "./components/AddItemModal";

export function ShopPage() {
    const { refresh, fetch} = useShop();
    const { t } = useTranslation();
    const [openedAddingModal, { toggle: toggleAddingModal, close: closeAddingModal }] = useDisclosure(false);
    const api = useContext(APIContext);
    const shoppingLists = fetch(api.getShoppingLists);

    if (shoppingLists.length === 0) {
        const createList = async () => {
            let id = await api.createShoppingList({
                kind: 'shoppingList',
                title: 'New shopping list',
                items: [

                ]
            });
            console.log(id);
            refresh();
        };
        return <Paper>
            <Center>
                <Stack>
                    <Text>{t('pages.shop.no_lists')}</Text>

                    <Button onClick={createList}>
                        {t('pages.shop.create_list')}
                    </Button>
                </Stack>
            </Center>
        </Paper>
    }

    const lastShoppingList = shoppingLists[shoppingLists.length - 1];

    return (
    <Stack>
        {lastShoppingList.items.map((item: any) => (
            <Item key={item.id} item={item} />
        ))}
        <Affix position={{ bottom: 78, right: 20 }}>
            <ActionIcon onClick={toggleAddingModal} size="xl" radius="xl">
                <IconPlus />
            </ActionIcon>
        </Affix>
        <AddItemModal opened={openedAddingModal} onClose={closeAddingModal} />
    </Stack>
    );
}