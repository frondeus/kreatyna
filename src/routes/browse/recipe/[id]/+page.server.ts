import { superValidate } from 'sveltekit-superforms/server';
import type { PageServerLoad } from './$types';
import { fail, redirect, error as returnError, type Actions } from '@sveltejs/kit';
import { RawMap } from '$lib/server/domains/raw/map';
import { notNull, notNullOrUndefined } from '$lib/shared/utils';
import { editRecipeSchema } from '$lib/shared/domains/recipe/schema';

export const load: PageServerLoad = async ({ params, locals: { session, prisma } }) => {
	if (!session) {
		throw redirect(303, '/login');
	}

	const id = Number(params.id);

	const data = await prisma.recipie.findUnique({
		where: { id },
		include: {
			recipiesIngredients: {
				include: {
					raw: true
				}
			}
		}
	});

	if (data === null) {
		throw returnError(404, 'Recipe was not found');
	}

	const ingredients = data.recipiesIngredients ?? [];
	const transformedData = {
		id: Number(data.id),
		name: data.name,
		portions: Number(data.portions),
		steps: data.steps as string[],
		ingredients: ingredients.map((raw) => ({
			id: Number(raw.id),
			name: raw.name,
			amount: raw.amount.toNumber(),
			unit: raw.unit ?? undefined,
			rawId: raw.raw?.id ?? undefined
		}))
	};

	const map = new RawMap(session);

	const relatedIngredients = new Map(
		ingredients
			.map((i) => i.raw)
			.filter(notNull)
			.map((raw) => map.toDomain(raw))
			.map((raw) => [raw.id, raw] as const)
	);

	const form = superValidate(transformedData, editRecipeSchema);

	return { session, form, id, relatedIngredients };
};

export const actions: Actions = {
	default: async ({ request, locals: { session, prisma } }) => {
		if (!session) {
			throw redirect(303, '/login');
		}

		const form = await superValidate(request, editRecipeSchema);

		if (!form.valid) {
			console.error(form);
			return fail(400, { form });
		}

		const ingredientsToKeep = form.data.ingredients.map((i) => i.id).filter(notNullOrUndefined);

		await prisma.recipie.update({
			where: { id: form.data.id },
			data: {
				name: form.data.name,
				portions: form.data.portions,
				steps: form.data.steps
			}
		});

		await prisma.recipiesIngredient.deleteMany({
			where: {
				recipe_id: form.data.id,
				id: {
					notIn: ingredientsToKeep
				}
			}
		});

		for (const ingredient of form.data.ingredients) {
			console.log('ingredient', ingredient);
			if (ingredient.id) {
				await prisma.recipiesIngredient.update({
					data: {
						name: ingredient.name,
						amount: ingredient.amount,
						unit: ingredient.unit ?? null,
						raw_id: ingredient.rawId ?? null
					},
					where: {
						id: ingredient.id,
						recipe_id: form.data.id
					}
				});
			} else {
				await prisma.recipiesIngredient.create({
					data: {
						recipe_id: form.data.id,
						name: ingredient.name,
						amount: ingredient.amount,
						unit: ingredient.unit ?? null,
						raw_id: ingredient.rawId ?? null
					}
				});
			}
		}

		return { form };
	}
};
