import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { MealForDay } from "$shared/domains/plan/models";
import type { RecipeSummary } from "$shared/domains/recipe/models";

export const load: PageServerLoad = async ({ locals: { session, services } }) => {
    if (!session) {
        throw redirect(303, '/login');
    }
	const dayIndices = [...Array(7).keys()]
        .map((day) => day + 1)
        .filter((day) => day % 2 === 1);

    const forBreakfast = await services.tag    .findIds({ category: 'for', name: 'breakfast' });
    const forSnack = await services.tag        .findIds({ category: 'for', name: 'snack' });
    const forLunch = await services.tag        .findIds({ category: 'for', name: 'lunch' });
    const forDinner = await services.tag       .findIds({ category: 'for', name: 'dinner' });
    const forDesert = await services.tag       .findIds({ category: 'for', name: 'desert' });


    const breakfasts = await services.recipe.getAll(forBreakfast);
    const snacks = await services.recipe.getAll(forSnack);
    const lunches = await services.recipe.getAll(forLunch);
    const dinners = await services.recipe.getAll(forDinner);
    const deserts = await services.recipe.getAll(forDesert);

   
    const breakfast = () => breakfasts.splice(Math.floor(Math.random() * breakfasts.length), 1)[0];
    const snack = () => snacks.splice(Math.floor(Math.random() * snacks.length), 1)[0];

	const meals: MealForDay[] = [
		...dayIndices.map((day) => ({
    		day,
    		slot: { index: 2, size: 2 },
            tags: [...forBreakfast],
    		name: 'Breakfast',
            recipe: breakfast()
        })),
		...dayIndices.map((day) => ({
			day,
			slot: { index: 6 },
            tags: [...forSnack],
			name: 'Snack',
            recipe: snack()
		})),
		...dayIndices.map((day) => ({
			day,
			slot: { index: 10, size: 2 },
            tags: [...forLunch],
			name: 'Lunch',
			// recipe: 'Chicken breast with a side of rice and vegetables',
		})),
		...dayIndices.map((day) => ({
			day,
			slot: { index: 14 },
            tags: [...forDesert],
			name: 'Desert',
			// recipe: 'Frozen grapes'
		})),
		...dayIndices.map((day) => ({
			day,
			slot: { index: 17, size: 2 },
            tags: [...forDinner],
			name: 'Dinner',
			// recipe: 'Grilled Tofu with stir-fried vegetables',
		}))
	]
    .flatMap(meal => {
        if (meal.day < 7) {
            return [ meal, { ...meal, day: meal.day + 1 }];
        } else return [ meal ]
    })
    ;

    return { session, meals }
};