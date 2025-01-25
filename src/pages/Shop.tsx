import { Button, Center, Paper, Stack, Text } from "@mantine/core";
import { createShoppingList} from "../api";
import { useTranslation } from "react-i18next";
import { useShop } from "../state";

export function ShopPage() {
    const { refresh, fetch } = useShop();
    const { t } = useTranslation();
    const shoppingLists = fetch();

    if (shoppingLists.length === 0) {
        const createList = async () => {
            let id = await createShoppingList({
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

    console.log(shoppingLists);
    return <div>
        Test
        {JSON.stringify(shoppingLists)}
    </div>
}