import * as typechat from 'typechat';
import { PRIVATE_OPENAI_API_KEY } from '$env/static/private';
import type { RecipeResponse } from './models/recipe';
import aiRecipeSchema from '$lib/server/domains/ai/models/recipe.ts?raw';
import aiCategorySchema from '$lib/server/domains/ai/models/category.ts?raw';
import generateRecipePrompt from '$lib/server/domains/ai/generateRecipe.prompt?raw';
import generateCategoryPrompt from '$lib/server/domains/ai/generateCategory.prompt?raw';
import type {
	GroupedRecipeIngredient,
	RecipeCreateOrUpdate
} from '$lib/shared/domains/recipe/models';
import type { AIMap } from './map';
import type { ShoppingListItem } from '$shared/domains/shopping/models';
import type { CategoryPrompt, CategoryResponse } from './models/category';
import type { PrismaClient } from '@prisma/client';

export class AIService {
	readonly model: typechat.TypeChatLanguageModel;
	constructor(readonly aiMap: AIMap, readonly prisma: PrismaClient) {
		this.model = typechat.createOpenAILanguageModel(PRIVATE_OPENAI_API_KEY, 'gpt-3.5-turbo');
	}

	async getRecipe(prompt: RecipeCreateOrUpdate) {
		const translator = typechat.createJsonTranslator<RecipeResponse>(
			this.model,
			aiRecipeSchema,
			'RecipeResponse'
		);

		const response = await translator.translate(
			generateRecipePrompt + '\n' + JSON.stringify(this.aiMap.toRecipeExample(prompt))
		);

		if (!response.success) {
			throw new Error('GPT Error' + response.message);
		}

		return this.aiMap.toRecipe(response.data);
	}

	private async categorize(prompt: CategoryPrompt): Promise<CategoryResponse> {
		const translator = typechat.createJsonTranslator<CategoryResponse>(
			this.model,
			aiCategorySchema,
			'CategoryResponse'
		);

		const categories = (
			await this.prisma.shoppingListCategory.findMany({
				select: {
					id: true
				}
			})
		).map((c) => c.id);

		const promptInstance = generateCategoryPrompt
			.replace('{{item}}', JSON.stringify(prompt))
			.replace('{{categories}}', JSON.stringify(categories));

		const response = await translator.translate(promptInstance);

		if (!response.success) {
			throw new Error('GPT Error' + response.message);
		}

		return response.data;
	}

	async categorizeUnlinkedShoppingListItem(prompt: string): Promise<CategoryResponse> {
		return this.categorize({
			name: prompt,
			ingredients: []
		});
	}

	async categorizeGroupedShoppingListItem(
		prompt: GroupedRecipeIngredient
	): Promise<CategoryResponse> {
		return this.categorize(this.aiMap.toCategoryPromptFromGroup(prompt));
	}

	async categorizeLinkedShoppingListItem(prompt: ShoppingListItem): Promise<CategoryResponse> {
		return this.categorize(this.aiMap.toCategoryPrompt(prompt));
	}
}
