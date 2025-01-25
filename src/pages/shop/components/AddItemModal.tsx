import { Modal, Stack, TextInput } from "@mantine/core";
import { useContext, useState } from "react";
import { APIContext } from "../../../api";
import { SuggestionItem } from "./SuggestionItem";
import { useSuggestions } from "../../../state";
import { ShoppingListItem } from "../../../types";

export interface AddItemModalProps {
    opened: boolean;
    onClose: () => void;
}

export function AddItemModal({ opened, onClose }: AddItemModalProps) {
    const [query, setQuery] = useState("");
    const api = useContext(APIContext);
    const { fetch: fetchSuggestions } = useSuggestions();
    
    const suggestions = query.length > 0 
        ? fetchSuggestions(() => api.getItemSuggestions(query))
        : [];

    return (
        <Modal 
            opened={opened} 
            onClose={onClose}
            fullScreen
            title="Add new item"
            styles={{
                header: {
                    marginBottom: 0
                }
            }}
        >
            <Stack gap="xs">
                <TextInput 
                    placeholder="Add new item"
                    size="md"
                    value={query}
                    onChange={(e) => setQuery(e.currentTarget.value)}
                />
                
                {suggestions.map((item, index) => (
                    <SuggestionItem 
                        key={index}
                        name={item.name}
                        amount={item.amount}
                        hasScale={item.hasScale}
                        onAdd={() => {
                            // This would call an API to add item
                            console.log("Add", item);
                        }}
                    />
                ))}
            </Stack>
        </Modal>
    );
}