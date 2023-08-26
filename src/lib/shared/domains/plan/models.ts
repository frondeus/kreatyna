import type { RecipeSummary } from "../recipe/models";
import type { TagSelect } from "../tags/models";

export interface Size {
    index: number;
    size?: number;
}

export interface MealForDay {
    name: string;
    day: number;
    slot: Size;

    tags: number[];
    recipeId?: number;
    recipe?: RecipeSummary;
}