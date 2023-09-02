import * as typechat from 'typechat';
import { PRIVATE_OPENAI_API_KEY } from '$env/static/private';
import type { RecipeMap } from '../recipe/map';
import type { RecipeResponse } from './models';
import aiSchema from '$lib/server/domains/ai/models.ts?raw';
import generateRecipePrompt from '$lib/server/domains/ai/generateRecipe.prompt?raw';
import type { RecipeCreateOrUpdate } from '$lib/shared/domains/recipe/models';
import type { AIMap } from './map';

export class AIService {
	constructor(readonly recipeMap: RecipeMap, readonly aiMap: AIMap) {}

	async getRecipe(prompt: RecipeCreateOrUpdate) {
		const model = typechat.createOpenAILanguageModel(PRIVATE_OPENAI_API_KEY, 'gpt-3.5-turbo');

		const translator = typechat.createJsonTranslator<RecipeResponse>(
			model,
			aiSchema,
			'RecipeResponse'
		);

		const response = await translator.translate(
			generateRecipePrompt + '\n' + JSON.stringify(this.aiMap.toExample(prompt))
		);

		if (!response.success) {
			throw new Error('GPT Error' + response.message);
		}

		return this.recipeMap.fromAI(response.data);
	}
}
